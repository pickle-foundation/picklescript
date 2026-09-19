//! Builtin `string` support: allocation, concat, and number-to-text.

use crate::gc::Gc;
use crate::layout::{builtin_string_total, str_set_len, str_bytes, str_len};
use crate::object::{PickleObject, PICKLE_CLASS_STRING};

/// Allocate a `string` object copying `bytes[0..len]`.
pub fn string_from_bytes(bytes: *const u8, len: usize, gc: &mut Gc) -> *mut PickleObject {
    let total = builtin_string_total(len);
    let obj = gc.alloc(total as u32, PICKLE_CLASS_STRING);
    unsafe {
        str_set_len(obj, len);
        if len > 0 {
            std::ptr::copy_nonoverlapping(bytes, str_bytes(obj) as *mut u8, len);
        }
    }
    obj
}

/// Allocate a `string` from a NUL-terminated C string.
pub fn string_from_cstr(cs: *const u8) -> *mut PickleObject {
    let mut len = 0usize;
    unsafe {
        while *cs.add(len) != 0 {
            len += 1;
        }
    }
    let gc = crate::gc::gc_mut();
    string_from_bytes(cs, len, gc)
}

/// Raw bytes of a string object.
pub fn string_bytes_ptr(obj: *const PickleObject) -> *const u8 {
    str_bytes(obj)
}

/// Byte length of a string object.
pub fn string_bytes_len(obj: *const PickleObject) -> usize {
    str_len(obj)
}

/// Concatenate two strings into a fresh third.
pub fn string_concat(a: *const PickleObject, b: *const PickleObject) -> *mut PickleObject {
    let la = string_bytes_len(a);
    let lb = string_bytes_len(b);
    let total = la + lb;
    let gc = crate::gc::gc_mut();
    let obj = gc.alloc(builtin_string_total(total) as u32, PICKLE_CLASS_STRING);
    unsafe {
        str_set_len(obj, total);
        if la > 0 {
            std::ptr::copy_nonoverlapping(str_bytes(a), str_bytes(obj) as *mut u8, la);
        }
        if lb > 0 {
            std::ptr::copy_nonoverlapping(str_bytes(b), (str_bytes(obj) as *mut u8).add(la), lb);
        }
    }
    obj
}

/// Format an `i64` as a string.
pub fn int64_to_string(v: i64) -> *mut PickleObject {
    let mut buf = [0u8; 24];
    let (neg, digits) = if v == i64::MIN {
        (true, i64::MIN.unsigned_abs())
    } else {
        (v < 0, v.unsigned_abs())
    };
    let mut n = digits;
    let mut idx = 0usize;
    loop {
        buf[idx] = b'0' + (n % 10) as u8;
        idx += 1;
        n /= 10;
        if n == 0 {
            break;
        }
    }
    let total_len = idx + if neg { 1 } else { 0 };
    let gc = crate::gc::gc_mut();
    let obj = gc.alloc(builtin_string_total(total_len) as u32, PICKLE_CLASS_STRING);
    unsafe {
        str_set_len(obj, total_len);
        let base = str_bytes(obj) as *mut u8;
        let mut o = total_len;
        if neg {
            *base = b'-';
        }
        for i in 0..idx {
            o -= 1;
            *base.add(o) = buf[i];
        }
    }
    obj
}

/// Format an `f64` as a string (Rust's shortest round-trip formatting).
pub fn float64_to_string(v: f64) -> *mut PickleObject {
    let text = format!("{}", v);
    let gc = crate::gc::gc_mut();
    string_from_bytes(text.as_ptr(), text.len(), gc)
}

/// Compare two strings (contents); returns -1/0/1.
pub fn string_cmp(a: *const PickleObject, b: *const PickleObject) -> i32 {
    let la = string_bytes_len(a);
    let lb = string_bytes_len(b);
    unsafe {
        let common = la.min(lb);
        for i in 0..common {
            let x = *str_bytes(a).add(i);
            let y = *str_bytes(b).add(i);
            if x != y {
                return if x < y { -1 } else { 1 };
            }
        }
        if la == lb {
            0
        } else {
            if la < lb {
                -1
            } else {
                1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> std::sync::MutexGuard<'static, ()> {
        crate::gc::test_begin()
    }

    #[test]
    fn roundtrip_bytes() {
        let _guard = setup();
        let s = string_from_bytes(b"hello".as_ptr(), 5, crate::gc::gc_mut());
        assert_eq!(string_bytes_len(s), 5);
        let bytes = unsafe {
            std::slice::from_raw_parts(string_bytes_ptr(s), string_bytes_len(s))
        };
        assert_eq!(bytes, b"hello");
    }

    #[test]
    fn concat_contents() {
        let _guard = setup();
        let a = string_from_bytes(b"foo".as_ptr(), 3, crate::gc::gc_mut());
        let b = string_from_bytes(b"bar".as_ptr(), 3, crate::gc::gc_mut());
        let c = string_concat(a, b);
        let bytes = unsafe {
            std::slice::from_raw_parts(string_bytes_ptr(c), string_bytes_len(c))
        };
        assert_eq!(bytes, b"foobar");
    }

    #[test]
    fn int_format() {
        let _guard = setup();
        for (v, expect) in [
            (0i64, &b"0"[..]),
            (42, &b"42"[..]),
            (-7, &b"-7"[..]),
            (i64::MAX, &b"9223372036854775807"[..]),
            (i64::MIN, &b"-9223372036854775808"[..]),
        ] {
            let s = int64_to_string(v);
            let bytes =
                unsafe { std::slice::from_raw_parts(string_bytes_ptr(s), string_bytes_len(s)) };
            assert_eq!(bytes, expect, "int64 {v}");
        }
    }

    #[test]
    fn float_format() {
        let _guard = setup();
        let s = float64_to_string(1.5);
        let bytes =
            unsafe { std::slice::from_raw_parts(string_bytes_ptr(s), string_bytes_len(s)) };
        assert_eq!(bytes, b"1.5");
    }
}