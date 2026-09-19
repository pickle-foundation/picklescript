//! Console bridge: `print`/`println` and value-to-text formatting for the
//! simple builtins. The compiler emits calls to these primitives; richer
//! formatting is left to `std.text` later.

use crate::layout::{list_len, map_len, str_bytes, str_len};
use crate::object::{PickleObject, PICKLE_CLASS_LIST, PICKLE_CLASS_MAP, PICKLE_CLASS_STRING};
#[cfg(not(test))]
use std::io::Write;

#[cfg(test)]
mod testio {
    use std::cell::{Cell, RefCell};

    thread_local! {
        pub static CAPTURE: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
        pub static HAS_CAPTURE: Cell<bool> = const { Cell::new(false) };
    }
}

/// Write bytes either to a test capture buffer (when enabled) or stdout.
/// Errors are ignored on purpose: a closed stdout should not unwind GC paths.
fn write_to_con(bytes: &[u8]) {
    #[cfg(test)]
    {
        if testio::HAS_CAPTURE.with(|c| c.get()) {
            testio::CAPTURE.with(|buf| buf.borrow_mut().extend_from_slice(bytes));
            return;
        }
    }
    #[cfg(not(test))]
    {
        let stdout = std::io::stdout();
        let mut lock = stdout.lock();
        let _ = lock.write_all(bytes);
        let _ = lock.flush();
    }
}

/// Print raw bytes (from a string literal or the contents of a string).
#[no_mangle]
pub extern "C" fn pickle_print_bytes(ptr: *const u8, len: usize) {
    if !ptr.is_null() && len > 0 {
        let bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
        write_to_con(bytes);
    }
}

/// Print a NUL-terminated static string.
#[no_mangle]
pub extern "C" fn pickle_print_cstr(ptr: *const u8) {
    if ptr.is_null() {
        return;
    }
    let mut len = 0usize;
    unsafe {
        while *ptr.add(len) != 0 {
            len += 1;
        }
    }
    let bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
    write_to_con(bytes);
}

/// Print a signed 64-bit integer as text.
#[no_mangle]
pub extern "C" fn pickle_print_i64(v: i64) {
    let s = crate::strings::int64_to_string(v);
    emit_string(s);
}

/// Print an unsigned 64-bit integer as text.
#[no_mangle]
pub extern "C" fn pickle_print_u64(v: u64) {
    let s = crate::strings::int64_to_string(v as i64);
    emit_string(s);
}

/// Print a double as text.
#[no_mangle]
pub extern "C" fn pickle_print_f64(v: f64) {
    let s = crate::strings::float64_to_string(v);
    emit_string(s);
}

/// Print a boolean as `true` / `false`.
#[no_mangle]
pub extern "C" fn pickle_print_bool(v: bool) {
    if v {
        pickle_print_cstr(b"true\0".as_ptr());
    } else {
        pickle_print_cstr(b"false\0".as_ptr());
    }
}

/// Print a single byte (for char).
#[no_mangle]
pub extern "C" fn pickle_print_byte(v: u8) {
    write_to_con(&[v]);
}

fn emit_string(s: *mut PickleObject) {
    let len = str_len(s);
    let ptr = str_bytes(s);
    pickle_print_bytes(ptr, len);
}

/// Print the text form of a managed object. Strings print their contents;
/// collections print their length; user objects print their class name.
#[no_mangle]
pub extern "C" fn pickle_print_obj(obj: *mut PickleObject) {
    if obj.is_null() {
        pickle_print_cstr(b"none\0".as_ptr());
        return;
    }
    unsafe {
        let class_id = (*obj).class_id;
        match class_id {
            PICKLE_CLASS_STRING => emit_string(obj),
            PICKLE_CLASS_LIST => {
                pickle_print_cstr(b"List(len=".as_ptr());
                pickle_print_i64(list_len(obj) as i64);
                pickle_print_byte(b')');
            }
            PICKLE_CLASS_MAP => {
                pickle_print_cstr(b"Map(len=".as_ptr());
                pickle_print_i64(map_len(obj) as i64);
                pickle_print_byte(b')');
            }
            _ => {
                let gc = crate::gc::gc_mut();
                let name = gc.class_name(class_id);
                match name {
                    Some(bytes) => pickle_print_bytes(bytes.as_ptr(), bytes.len()),
                    None => pickle_print_cstr(b"<object>\0".as_ptr()),
                }
            }
        }
    }
}

/// Write a newline.
#[no_mangle]
pub extern "C" fn pickle_print_newline() {
    write_to_con(b"\n");
}

/// Flush stdout. Kept for explicit checkpointing from compiled code.
#[no_mangle]
pub extern "C" fn pickle_flush() {
    #[cfg(not(test))]
    {
        let stdout = std::io::stdout();
        let _ = stdout.lock().flush();
    }
}

#[cfg(test)]
mod tests {
    use super::testio::{CAPTURE, HAS_CAPTURE};
    use super::*;

    fn capture() -> Vec<u8> {
        HAS_CAPTURE.with(|c| c.set(true));
        CAPTURE.with(|b| {
            let mut b = b.borrow_mut();
            b.clear();
            b.clone()
        })
    }

    fn stop_capture() -> String {
        let out = CAPTURE.with(|b| b.borrow().clone());
        HAS_CAPTURE.with(|c| c.set(false));
        String::from_utf8(out).unwrap()
    }

    #[test]
    fn prints_values() {
        let _guard = crate::gc::test_begin();
        capture();
        pickle_print_cstr(b"hi\0".as_ptr());
        pickle_print_bytes(b" world".as_ptr(), 6);
        pickle_print_i64(42);
        pickle_print_f64(1.25);
        pickle_print_bool(true);
        pickle_print_newline();
        let out = stop_capture();
        assert_eq!(out, "hi world421.25true\n");
    }
}