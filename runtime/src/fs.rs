//! `std.fs` intrinsics: read, write, and test files on the host filesystem.
//!
//! These are the thin runtime layer for the stdlib's file module. Read
//! failures are surfaced as `none` (`null` object pointer) — errors are
//! values — and writes report success as a `bool`.

use crate::object::PickleObject;
use crate::strings::{string_bytes_len, string_bytes_ptr, string_from_bytes};

fn path_bytes(path: *const PickleObject) -> &'static [u8] {
    unsafe { std::slice::from_raw_parts(string_bytes_ptr(path), string_bytes_len(path)) }
}

fn path_of(path: *const PickleObject) -> &'static std::path::Path {
    std::path::Path::new(std::str::from_utf8(path_bytes(path)).unwrap_or(""))
}

/// Read a whole file as a `string`. Strings are opaque UTF-8 byte arrays, so
/// binary or non-UTF-8 content still round-trips byte-exact; a path that
/// cannot be read (missing, is a directory, no permission) is `none`.
#[no_mangle]
pub extern "C" fn pickle_read_file(path: *const PickleObject) -> *mut PickleObject {
    match std::fs::read(path_of(path)) {
        Ok(data) => {
            let gc = crate::gc::gc_mut();
            string_from_bytes(data.as_ptr(), data.len(), gc)
        }
        Err(_) => std::ptr::null_mut(),
    }
}

/// Overwrite `path` with `text`'s bytes; returns whether the write succeeded.
#[no_mangle]
pub extern "C" fn pickle_write_file(path: *const PickleObject, text: *const PickleObject) -> bool {
    std::fs::write(path_of(path), path_bytes(text)).is_ok()
}

/// Whether `path` exists on the filesystem (any kind: file, directory, ...).
#[no_mangle]
pub extern "C" fn pickle_file_exists(path: *const PickleObject) -> bool {
    std::fs::metadata(path_of(path)).is_ok()
}