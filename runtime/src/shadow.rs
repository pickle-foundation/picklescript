//! Shadow-stack protocol.
//!
//! Compiled functions own a fixed-size slot frame (see `docs/05-memory.md`);
//! the prologue calls `pickle_shadow_push`, spills every live managed pointer
//! into its slot around each safepoint, and `pickle_shadow_pop` in the
//! epilogue. The collector treats every registered thread's frame slots as
//! roots.

use crate::object::PickleObject;

/// Layout of a shadow frame. Slots follow the header inline:
///
/// ```text
/// #[repr(C)]
/// struct ShadowFrame {
///     prev: *mut ShadowFrame,      // 8
///     slot_count: u32,             // 4
///     _pad: u32,                   // 4
///     // slots: [*mut PickleObject; slot_count]
/// }
/// ```
///
/// Total header = 16 bytes; slot `i` lives at byte offset `16 + i * 8`.
#[repr(C)]
pub struct ShadowFrame {
    pub prev: *mut ShadowFrame,
    pub slot_count: u32,
    pub _pad: u32,
}

const FRAME_HEADER_BYTES: usize = 16;

/// Per-OS-thread GC state, kept in a process-wide registry so any thread can
/// collect from every thread's shadow stack.
#[repr(C)]
pub struct GcThread {
    /// Head of this thread's shadow-frame list (LIFO).
    pub frames: *mut ShadowFrame,
    pub id: u32,
    /// Registry link.
    pub next: *mut GcThread,
}

/// Flat registry of threads; also the intrusive list of `GcThread` nodes.
static mut THREAD_HEAD: *mut GcThread = std::ptr::null_mut();
static THREAD_COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

std::thread_local! {
    /// Thread-local handle into the registry (null until registered).
    static CURRENT_THREAD: std::cell::Cell<*mut GcThread> = const { std::cell::Cell::new(std::ptr::null_mut()) };
}

fn assert_ptr_empty(p: *mut u8, len: usize) {
    for i in 0..len {
        assert_eq!(unsafe { *p.add(i) }, 0, "frame slot memory not zeroed at offset {i}");
    }
}

impl ShadowFrame {
    /// Pointer to the first slot.
    pub fn slots_ptr(&self) -> *mut *mut PickleObject {
        unsafe { (self as *const ShadowFrame as *mut u8).add(FRAME_HEADER_BYTES) as *mut *mut PickleObject }
    }

    /// Direct access to a slot.
    pub fn slot(&self, index: u32) -> *mut PickleObject {
        assert!(index < self.slot_count);
        unsafe { self.slots_ptr().add(index as usize).read() }
    }

    pub fn set_slot(&mut self, index: u32, value: *mut PickleObject) {
        assert!(index < self.slot_count);
        unsafe { self.slots_ptr().add(index as usize).write(value) }
    }

    /// Debug: require the whole slot area to be zeroed before first use.
    pub fn debug_check_zeroed(&self) {
        let len = (self.slot_count as usize) * 8;
        assert_ptr_empty(self.slots_ptr() as *mut u8, len);
    }
}

/// Returns this thread's registered `GcThread`, or null if not registered.
#[inline]
pub fn current_thread() -> *mut GcThread {
    CURRENT_THREAD.with(|c| c.get())
}

/// Allocate a `GcThread` node off the registry (leaked intentionally; freed
/// only at process exit). Sets the thread-local handle.
///
/// Returns the new node. Safe to call once per OS thread.
pub fn register_thread() -> *mut GcThread {
    unsafe {
        let existing = current_thread();
        if !existing.is_null() {
            return existing;
        }
        let node = Box::into_raw(Box::new(GcThread {
            frames: std::ptr::null_mut(),
            id: THREAD_COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            next: std::ptr::null_mut(),
        }));
        // Insert into the intrusive list. Guard with a tiny spinlock to be safe
        // on multi-threaded startup (single writer takes the lock).
        static LOCK: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
        while LOCK
            .compare_exchange_weak(
                false,
                true,
                std::sync::atomic::Ordering::AcqRel,
                std::sync::atomic::Ordering::Acquire,
            )
            .is_err()
        {
            std::hint::spin_loop();
        }
        (*node).next = THREAD_HEAD;
        THREAD_HEAD = node;
        LOCK.store(false, std::sync::atomic::Ordering::Release);

        CURRENT_THREAD.with(|c| c.set(node));
        node
    }
}

/// Remove `node` (and any frames it still has) from the registry and detach.
pub fn unregister_thread(node: *mut GcThread) {
    unsafe {
        let mut cur = THREAD_HEAD;
        let mut prev: *mut GcThread = std::ptr::null_mut();
        while !cur.is_null() {
            if cur == node {
                if prev.is_null() {
                    THREAD_HEAD = (*cur).next;
                } else {
                    (*prev).next = (*cur).next;
                }
                break;
            }
            prev = cur;
            cur = (*cur).next;
        }
        // Detach local TLS, then drop the node.
        CURRENT_THREAD.with(|c| c.set(std::ptr::null_mut()));
        drop(Box::from_raw(node));
    }
}

/// Push `frame` onto the current thread's stack. Must be LIFO-matched with
/// `pop_frame`.
#[no_mangle]
pub extern "C" fn pickle_shadow_push(frame: *mut ShadowFrame) {
    unsafe {
        assert!(!frame.is_null());
        let thread = register_thread();
        (*frame).prev = (*thread).frames;
        (*thread).frames = frame;
    }
}

/// Pop `frame`, which must be the current top of this thread's stack.
#[no_mangle]
pub extern "C" fn pickle_shadow_pop(frame: *mut ShadowFrame) {
    unsafe {
        let thread = register_thread();
        assert_eq!((*thread).frames, frame, "shadow stack pop out of order");
        (*thread).frames = (*frame).prev;
        (*frame).prev = std::ptr::null_mut();
    }
}

/// Set slot `index` of `frame` to a managed pointer.
#[no_mangle]
pub extern "C" fn pickle_shadow_set(frame: *mut ShadowFrame, index: u32, value: *mut PickleObject) {
    unsafe {
        assert!(!frame.is_null());
        (*frame).set_slot(index, value)
    }
}

/// Read slot `index` of `frame` as a managed pointer.
#[no_mangle]
pub extern "C" fn pickle_shadow_get(frame: *mut ShadowFrame, index: u32) -> *mut PickleObject {
    unsafe { (*frame).slot(index) }
}

/// Register the calling OS thread with the collector. Returns the thread's id.
#[no_mangle]
pub extern "C" fn pickle_gc_register_thread() -> u32 {
    let node = register_thread();
    unsafe { (*node).id }
}

/// Unregister the calling OS thread from the collector.
#[no_mangle]
pub extern "C" fn pickle_gc_unregister_thread() {
    let node = current_thread();
    if !node.is_null() {
        unregister_thread(node);
    }
}

/// Process-wide list of registered thread nodes (for the collector).
pub fn all_threads() -> *mut GcThread {
    unsafe { THREAD_HEAD }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_is_16_byte_header() {
        assert_eq!(std::mem::size_of::<ShadowFrame>(), 16);
    }

    #[test]
    fn slots_roundtrip() {
        register_thread();
        let mut frame = ShadowFrame {
            prev: std::ptr::null_mut(),
            slot_count: 2,
            _pad: 0,
        };
        pickle_shadow_push(&mut frame as *mut ShadowFrame);
        let obj_a = 0x11 as *mut PickleObject;
        let obj_b = 0x22 as *mut PickleObject;
        pickle_shadow_set(&mut frame as *mut ShadowFrame, 0, obj_a);
        pickle_shadow_set(&mut frame as *mut ShadowFrame, 1, obj_b);
        assert_eq!(pickle_shadow_get(&mut frame as *mut ShadowFrame, 0), obj_a);
        assert_eq!(pickle_shadow_get(&mut frame as *mut ShadowFrame, 1), obj_b);
        pickle_shadow_pop(&mut frame as *mut ShadowFrame);

        let node = current_thread();
        assert!(!node.is_null());
        unregister_thread(node);
    }
}