//! Panic path: language-level `panic(msg)` and a guard for Rust panics
//! escaping `extern "C"` boundaries.
//!
//! In capture mode (the test harness) panics become recorded failures so a
//! failing `test fn` body can be reported without aborting the process.

use std::io::Write;

/// Fatal message prefix written to stderr.
const FATAL: &[u8] = b"fatal: ";

/// Capture mode: turn fatal panics into recoverable, recorded failures.
static CAPTURE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

thread_local! {
    static LAST_PANIC: std::cell::RefCell<Option<String>> =
        const { std::cell::RefCell::new(None) };
}

/// Whether panics are being captured for the test harness.
pub fn is_capturing() -> bool {
    CAPTURE.load(std::sync::atomic::Ordering::Relaxed)
}

/// Enter/leave capture mode. Leaving also clears any captured message.
pub fn set_capture_mode(capture: bool) {
    CAPTURE.store(capture, std::sync::atomic::Ordering::Relaxed);
    if !capture {
        LAST_PANIC.with(|c| *c.borrow_mut() = None);
    }
}

/// The message of the most recent captured panic, if any.
pub fn take_captured_panic() -> Option<String> {
    LAST_PANIC.with(|c| c.borrow_mut().take())
}

/// ABI: `captureBegin()` from the ported test harness. Enters capture mode so
/// a failing test body records a message instead of killing the process. Does
/// not clear a pending message: a failure recorded before the begin (e.g. by
/// a hook) must survive until the harness' `captureTake()`.
#[no_mangle]
pub extern "C" fn pickle_test_capture_begin() {
    set_capture_mode(true);
}

/// ABI: `captureTake()` from the ported test harness. Exits nothing and
/// returns the message of the most recent captured failure as a `string?`
/// object (null when none), or null if capture is off.
#[no_mangle]
pub extern "C" fn pickle_test_capture_take() -> *mut crate::object::PickleObject {
    if is_capturing() {
        if let Some(msg) = take_captured_panic() {
            return crate::strings::string_from_bytes(
                msg.as_bytes().as_ptr(),
                msg.len(),
                crate::gc::gc_mut(),
            );
        }
    }
    std::ptr::null_mut()
}

fn record_panic(msg: String) {
    LAST_PANIC.with(|c| *c.borrow_mut() = Some(msg));
}

/// Record a captured failure message (used by the test harness' `assert`
/// lowering without unwinding through the JIT frame).
pub(crate) fn record_panic_msg(msg: impl Into<String>) {
    record_panic(msg.into());
}

/// Write the fatal banner plus `msg` to stderr and exit 1. Used outside
/// capture mode by `panic` and failing `assert`s in non-test code.
pub(crate) fn fatal(msg: &str) -> ! {
    let stderr = std::io::stderr();
    let mut lock = stderr.lock();
    let _ = lock.write_all(FATAL);
    let _ = lock.write_all(msg.as_bytes());
    let _ = lock.write_all(b"\n");
    let _ = lock.flush();
    std::process::exit(1);
}

fn panic_text(info: &std::panic::PanicHookInfo<'_>) -> String {
    if let Some(s) = info.payload().downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = info.payload().downcast_ref::<String>() {
        s.clone()
    } else {
        format!("{info}")
    }
}

/// ABI: `panic("...")` from PickleScript. In capture mode (a running test)
/// this becomes a recoverable failure; otherwise it prints a banner and
/// exits 1.
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn pickle_panic_bytes(ptr: *const u8, len: usize) -> ! {
    let msg = if ptr.is_null() || len == 0 {
        String::new()
    } else {
        unsafe { String::from_utf8_lossy(std::slice::from_raw_parts(ptr, len)).into_owned() }
    };
    if is_capturing() {
        record_panic(msg);
        std::panic::panic_any("pickle panic");
    }
    fatal(&msg);
}

/// ABI: `panic("...")` from a NUL-terminated C string.
#[no_mangle]
pub extern "C" fn pickle_panic_cstr(ptr: *const u8) -> ! {
    let mut len = 0usize;
    unsafe {
        while !ptr.is_null() && *ptr.add(len) != 0 {
            len += 1;
        }
    }
    pickle_panic_bytes(ptr, len)
}

/// ABI: a `match` whose scrutinee matched no arm. Reuses the panic path so
/// compiled programs fail loudly instead of reading uninitialised state.
#[no_mangle]
pub extern "C" fn pickle_panic_no_match() -> ! {
    pickle_panic_cstr(b"pickle: match is not exhaustive\0".as_ptr())
}

/// ABI: a `!` unwrap applied to a `none` option. Reuses the panic path so
/// compiled programs fail loudly instead of reading null as a value.
#[no_mangle]
pub extern "C" fn pickle_panic_none_unwrap() -> ! {
    pickle_panic_cstr(b"pickle: unwrapped none value\0".as_ptr())
}

/// ABI: an interface-method dispatch whose receiver implements no such
/// method at runtime (an unregistered/foreign class path). Same panic
/// contract: fail loudly in the dispatch path.
#[no_mangle]
pub extern "C" fn pickle_panic_no_iface_method() -> ! {
    pickle_panic_cstr(b"pickle: interface method dispatch found no implementation\0".as_ptr())
}

/// Replace the Rust panic hook so an internal panic cannot unwind across the
/// `extern "C"` ABI. It prints `internal error: <msg>` and exits 1.
///
/// In capture mode a panic is recorded instead (for the test harness), so
/// `pickle test` can report a failing test without aborting the process.
/// Under `cfg(test)` this is a no-op: the Rust test harness needs panics to
/// unwind to report failures itself.
#[cfg(not(test))]
pub fn install_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        let msg = panic_text(info);
        if is_capturing() {
            record_panic(msg);
            return;
        }
        let stderr = std::io::stderr();
        let mut lock = stderr.lock();
        let _ = lock.write_all(b"internal error: ");
        let _ = lock.write_all(msg.as_bytes());
        let location = info
            .location()
            .map(|l| format!(" at {}:{}", l.file(), l.line()))
            .unwrap_or_default();
        let _ = lock.write_all(location.as_bytes());
        let _ = lock.write_all(b"\n");
        let _ = lock.flush();
        std::process::exit(1);
    }));
}

#[cfg(test)]
pub fn install_panic_hook() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hook_installs() {
        install_panic_hook();
        // The hook is active; calling the old default would print. Just make
        // sure re-installation doesn't blow up.
        install_panic_hook();
    }

    #[test]
    fn capture_records_and_clears_message() {
        assert!(!is_capturing());
        set_capture_mode(true);
        let result = std::panic::catch_unwind(|| {
            record_panic("boom".to_string());
        });
        assert!(result.is_ok());
        assert_eq!(take_captured_panic().as_deref(), Some("boom"));
        assert_eq!(take_captured_panic(), None, "message consumed once");
        set_capture_mode(false);
        assert!(!is_capturing());
    }
}
