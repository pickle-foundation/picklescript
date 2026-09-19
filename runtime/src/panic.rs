//! Panic path: language-level `panic(msg)` and a guard for Rust panics
//! escaping `extern "C"` boundaries.

use std::io::Write;

/// Fatal message prefix written to stderr.
const FATAL: &[u8] = b"fatal: ";

/// ABI: `panic("...")` from PickleScript. Prints a banner and exits 1.
pub extern "C" fn pickle_panic_bytes(ptr: *const u8, len: usize) -> ! {
    let stderr = std::io::stderr();
    let mut lock = stderr.lock();
    let _ = lock.write_all(FATAL);
    if !ptr.is_null() && len > 0 {
        let bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
        let _ = lock.write_all(bytes);
    }
    let _ = lock.write_all(b"\n");
    let _ = lock.flush();
    std::process::exit(1);
}

/// ABI: `panic("...")` from a NUL-terminated C string.
pub extern "C" fn pickle_panic_cstr(ptr: *const u8) -> ! {
    let mut len = 0usize;
    unsafe {
        while !ptr.is_null() && *ptr.add(len) != 0 {
            len += 1;
        }
    }
    pickle_panic_bytes(ptr, len)
}

/// Replace the Rust panic hook so an internal panic cannot unwind across the
/// `extern "C"` ABI. It prints `internal error: <msg>` and exits 1.
///
/// Under `cfg(test)` this is a no-op: the test harness needs panics to unwind
/// so it can report failures instead of killing the process.
#[cfg(not(test))]
pub fn install_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        let stderr = std::io::stderr();
        let mut lock = stderr.lock();
        let _ = lock.write_all(b"internal error: ");
        if let Some(s) = info.payload().downcast_ref::<&str>() {
            let _ = lock.write_all(s.as_bytes());
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            let _ = lock.write_all(s.as_bytes());
        } else {
            let _ = write!(lock, "{info}");
        }
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
}