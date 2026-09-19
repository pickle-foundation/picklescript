//! Test harness registry (see `docs/07-runtime.md`, "Testing hooks").
//!
//! `test fn` bodies are discovered by the compiler, which emits a static
//! table of `PickleTest` descriptors into the generated object. The runtime
//! registers the table (`pickle_test_register_table`) and runs it on demand
//! (`pickle_runtime_run_tests`): bodies run with panics trapped, pass/fail is
//! reported per test plus a summary, and the exit code is nonzero when any
//! test failed — the behaviour `pickle test` links against.

use std::panic::{catch_unwind, AssertUnwindSafe};

/// A single compiler-emitted test descriptor. `#[repr(C)]`: generated code
/// lays this out statically in the object file.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PickleTest {
    pub name: *const u8,
    pub name_len: u32,
    pub body: extern "C" fn() -> i32,
}

// Descriptors are inert data (`name` points into the object's static bytes);
// the Vec copy is only ever read back on the running thread.
unsafe impl Send for PickleTest {}

/// Registered tests, in registration order.
static TESTS: std::sync::Mutex<Vec<PickleTest>> = std::sync::Mutex::new(Vec::new());

/// ABI: register `count` compiler-emitted test descriptors. Returns the
/// total number of registered tests.
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn pickle_test_register_table(table: *const PickleTest, count: u32) -> u32 {
    if table.is_null() || count == 0 {
        return 0;
    }
    let mut tests = TESTS.lock().unwrap_or_else(|p| p.into_inner());
    unsafe {
        for i in 0..count as usize {
            tests.push(table.add(i).read());
        }
    }
    tests.len() as u32
}

/// Run every registered test. Reports one line per test plus a summary.
/// Returns 0 when all pass, 1 when any body returned nonzero or panicked.
#[no_mangle]
pub extern "C" fn pickle_runtime_run_tests() -> i32 {
    crate::panic::set_capture_mode(true);
    let tests = TESTS.lock().unwrap_or_else(|p| p.into_inner()).clone();
    let mut passed = 0usize;
    let mut failed = 0usize;
    for t in &tests {
        let outcome = catch_unwind(AssertUnwindSafe(|| (t.body)()));
        let ok = matches!(outcome, Ok(code) if code == 0);
        let detail = if ok {
            None
        } else {
            crate::panic::take_captured_panic()
        };
        if ok {
            passed += 1;
        } else {
            failed += 1;
        }
        report(t, ok, detail.as_deref());
    }
    crate::panic::set_capture_mode(false);
    report_summary(passed, failed);
    if failed > 0 {
        1
    } else {
        0
    }
}

fn test_name(t: &PickleTest) -> String {
    if t.name.is_null() {
        "<unnamed>".to_string()
    } else {
        unsafe {
            String::from_utf8_lossy(std::slice::from_raw_parts(t.name, t.name_len as usize))
                .into_owned()
        }
    }
}

fn report(t: &PickleTest, ok: bool, detail: Option<&str>) {
    let mut line = format!("test {} ", test_name(t));
    if ok {
        line.push_str("... ok\n");
    } else {
        line.push_str("... FAILED");
        if let Some(d) = detail {
            line.push_str(" (");
            line.push_str(d);
            line.push(')');
        }
        line.push('\n');
    }
    crate::console::write_to_con(line.as_bytes());
}

fn report_summary(passed: usize, failed: usize) {
    let line = format!("test result: {passed} passed; {failed} failed\n");
    crate::console::write_to_con(line.as_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    extern "C" fn pass() -> i32 {
        0
    }

    extern "C" fn fail_code() -> i32 {
        1
    }

    fn reg(tests: &[PickleTest]) -> Vec<PickleTest> {
        TESTS.lock().unwrap_or_else(|p| p.into_inner()).clear();
        pickle_test_register_table(tests.as_ptr(), tests.len() as u32);
        tests.to_vec()
    }

    fn capture_start() {
        crate::console::testio::HAS_CAPTURE.with(|c| c.set(true));
        crate::console::testio::CAPTURE.with(|b| b.borrow_mut().clear());
    }

    fn capture_stop() -> String {
        let out = crate::console::testio::CAPTURE.with(|b| b.borrow().clone());
        crate::console::testio::HAS_CAPTURE.with(|c| c.set(false));
        String::from_utf8(out).unwrap()
    }

    #[test]
    fn all_pass_returns_zero() {
        let _guard = crate::gc::test_begin();
        capture_start();
        let table = [
            PickleTest {
                name: b"pass_a\0".as_ptr(),
                name_len: 6,
                body: pass,
            },
            PickleTest {
                name: b"pass_b\0".as_ptr(),
                name_len: 6,
                body: pass,
            },
        ];
        let regs = reg(&table);
        assert_eq!(regs.len(), 2);
        assert_eq!(pickle_runtime_run_tests(), 0);
        assert_eq!(
            capture_stop(),
            "test pass_a ... ok\ntest pass_b ... ok\ntest result: 2 passed; 0 failed\n"
        );
        TESTS.lock().unwrap().clear();
    }

    #[test]
    fn failing_body_returns_nonzero_and_reports() {
        let _guard = crate::gc::test_begin();
        capture_start();
        let table = [
            PickleTest {
                name: b"ok\0".as_ptr(),
                name_len: 2,
                body: pass,
            },
            PickleTest {
                name: b"bad\0".as_ptr(),
                name_len: 3,
                body: fail_code,
            },
        ];
        reg(&table);
        assert_eq!(pickle_runtime_run_tests(), 1);
        assert_eq!(
            capture_stop(),
            "test ok ... ok\ntest bad ... FAILED\ntest result: 1 passed; 1 failed\n"
        );
        TESTS.lock().unwrap().clear();
    }
}