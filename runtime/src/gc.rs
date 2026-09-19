//! Garbage collector orchestration: the process-wide heap + descriptor
//! registry + static-root table, and the exported `pickle_gc_*` ABI.
//!
//! v1 collector: non-moving, stop-the-world style mark-and-sweep. Runs on
//! demand (`pickle_gc_collect`) and automatically when allocation passes a
//! threshold.

use crate::heap::{raw_free, Heap};
use crate::layout::{list_data, map_entries};
use crate::object::{
    DescriptorTable, PickleObject, RootCell, PICKLE_CLASS_FLAG_FINALIZER, PICKLE_CLASS_LIST, PICKLE_CLASS_MAP,
    PICKLE_CLASS_STRING,
};
use crate::shadow;
use crate::trace;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Mutex;

/// Static root cells (compiler-emitted global variables and class statics).
static STATIC_ROOTS: Mutex<Vec<&'static mut RootCell>> = Mutex::new(Vec::new());

/// Serialises collectors; also acts as the world stop on multi-thread.
static COLLECT_LOCK: Mutex<()> = Mutex::new(());

/// Set while a GC is in progress so a re-entrant allocation can panic.
static COLLECTING: AtomicBool = AtomicBool::new(false);

/// Allocation counter since the last collection.
static ALLOC_SINCE_GC: AtomicU32 = AtomicU32::new(0);

/// Number of object-bytes allocated since the last collection.
static BYTES_SINCE_GC: AtomicU32 = AtomicU32::new(0);

/// Default threshold in object-bytes that triggers a collection.
pub const DEFAULT_AUTO_COLLECT_THRESHOLD: u32 = 8 * 1024 * 1024;

/// Current auto-collect threshold, settable at runtime.
static AUTO_THRESHOLD: AtomicU32 = AtomicU32::new(DEFAULT_AUTO_COLLECT_THRESHOLD);

/// Total completed collection cycles (observability for tests).
static COLLECTIONS: AtomicU32 = AtomicU32::new(0);

/// The runtime heap and class registry.
pub struct Gc {
    pub heap: Heap,
    pub descriptors: DescriptorTable,
    /// Total live object bytes (updated at sweep).
    pub live_bytes: AtomicU32,
}

impl Gc {
    pub fn new() -> Gc {
        Gc {
            heap: Heap::new(),
            descriptors: DescriptorTable::new(),
            live_bytes: AtomicU32::new(0),
        }
    }

    /// Allocate an object of `size` bytes (>= header, multiple of 8),
    /// initialising its header. Increments the allocation counters.
    pub fn alloc(&mut self, size: u32, class_id: u32) -> *mut PickleObject {
        let size = crate::heap::align8(size as usize)
            .max(crate::heap::MIN_OBJECT)
            .min(u32::MAX as usize) as u32;
        let obj = self.heap.alloc_object(size as usize);
        unsafe {
            (*obj).init(class_id, size);
        }
        let _ = ALLOC_SINCE_GC.fetch_add(1, Ordering::Relaxed);
        let _ = BYTES_SINCE_GC.fetch_add(size, Ordering::Relaxed);
        self.maybe_collect();
        obj
    }

    /// Class name of `class_id` (for printing/reflection), if registered.
    pub fn class_name(&self, class_id: u32) -> Option<&[u8]> {
        let d = self.descriptors.get(class_id)?;
        unsafe { Some(std::slice::from_raw_parts(d.name_ptr, d.name_len as usize)) }
    }

    /// Register a class descriptor; returns its class id (index).
    pub fn register_class(&mut self, d: crate::object::ClassDescriptor) -> u32 {
        self.descriptors.register(d)
    }

    /// Run a full mark-and-sweep cycle.
    pub fn collect(&mut self) {
        // Only one collector at a time.
        let _guard = match COLLECT_LOCK.try_lock() {
            Ok(g) => g,
            Err(_) => return,
        };
        if COLLECTING.swap(true, Ordering::AcqRel) {
            return;
        }
        let _ = COLLECTIONS.fetch_add(1, Ordering::Relaxed);

        // Snapshot static roots.
        let roots: Vec<*mut *mut crate::object::PickleObject> = {
            let cells = STATIC_ROOTS.lock().unwrap();
            cells.iter().map(|cell| cell.cell).collect()
        };
        let descriptors = &self.descriptors;
        trace::trace_from_roots(descriptors, &roots);

        // Sweep: release dead list/map raw buffers, run finalizers.
        let heap = &mut self.heap;
        let mut live: u32 = 0;
        heap.sweep(|obj| {
            unsafe {
                let class_id = (*obj).class_id;
                match class_id {
                    PICKLE_CLASS_LIST => {
                        let data = list_data(obj);
                        if !data.is_null() {
                            raw_free(data as *mut u8);
                        }
                    }
                    PICKLE_CLASS_MAP => {
                        let entries = map_entries(obj);
                        if !entries.is_null() {
                            raw_free(entries as *mut u8);
                        }
                    }
                    PICKLE_CLASS_STRING => {}
                    _ => {
                        if let Some(d) = descriptors.get(class_id) {
                            if d.flags & PICKLE_CLASS_FLAG_FINALIZER != 0 {
                                (d.finalizer)(obj);
                            }
                        }
                    }
                }
                live += (*obj).size;
            }
        });
        self.live_bytes.store(live, Ordering::Relaxed);

        ALLOC_SINCE_GC.store(0, Ordering::Relaxed);
        BYTES_SINCE_GC.store(0, Ordering::Relaxed);
        COLLECTING.store(false, Ordering::Release);
    }

    /// Maybe collect if the allocation threshold has been crossed.
    pub fn maybe_collect(&mut self) {
        if BYTES_SINCE_GC.load(Ordering::Relaxed) >= AUTO_THRESHOLD.load(Ordering::Relaxed) {
            self.collect();
        }
    }
}

impl Default for Gc {
    fn default() -> Self {
        Self::new()
    }
}

/// The process-global GC. Initialised by `pickle_runtime_init`.
static mut GLOBAL_GC: *mut Gc = std::ptr::null_mut();

/// Whether the runtime has been initialised.
pub fn is_initialized() -> bool {
    unsafe { !GLOBAL_GC.is_null() }
}

pub(crate) fn gc_mut() -> &'static mut Gc {
    unsafe {
        assert!(!GLOBAL_GC.is_null(), "pickle runtime not initialised");
        &mut *GLOBAL_GC
    }
}

/// Initialise the collector. Idempotent enough for tests.
pub(crate) fn init() {
    unsafe {
        if GLOBAL_GC.is_null() {
            GLOBAL_GC = Box::into_raw(Box::new(Gc::new()));
        }
    }
    // Ensure the calling thread is registered so its roots are seen.
    shadow::register_thread();
}

/// Shut the collector down (tests / process exit).
pub(crate) fn shutdown() {
    unsafe {
        if !GLOBAL_GC.is_null() {
            let mut gc = Box::from_raw(GLOBAL_GC);
            gc.heap.destroy();
            GLOBAL_GC = std::ptr::null_mut();
        }
    }
}

/// Allocate a managed object of `size` bytes for class `class_id`.
#[no_mangle]
pub extern "C" fn pickle_gc_alloc(size: u32, class_id: u32) -> *mut PickleObject {
    let gc = gc_mut();
    gc.alloc(size, class_id)
}

/// Register a static root cell (`*mut *mut PickleObject`). The cell's pointee
/// is traced at every collection. Returns a handle for `pickle_gc_root_drop`.
#[no_mangle]
pub extern "C" fn pickle_gc_root_add(cell: *mut *mut PickleObject) -> *mut RootCell {
    let node = Box::into_raw(Box::new(RootCell {
        cell,
        next: std::ptr::null_mut(),
    }));
    STATIC_ROOTS.lock().unwrap().push(unsafe { &mut *node });
    node
}

/// Remove a static root cell previously registered with `pickle_gc_root_add`.
#[no_mangle]
pub extern "C" fn pickle_gc_root_drop(handle: *mut RootCell) {
    if handle.is_null() {
        return;
    }
    let mut list = STATIC_ROOTS.lock().unwrap();
    list.retain(|c| {
        let ptr = &**c as *const RootCell;
        !std::ptr::eq(ptr, handle)
    });
    // SAFETY: handle came from `pickle_gc_root_add`.
    unsafe {
        drop(Box::from_raw(handle));
    }
}

/// Collect garbage now. `force` is accepted for ABI compatibility; v1 always
/// collects (same as `force = true`).
#[no_mangle]
pub extern "C" fn pickle_gc_collect(_force: bool) {
    let gc = gc_mut();
    gc.collect();
}

/// Number of object allocations since the last collection (debug/stats).
#[no_mangle]
pub extern "C" fn pickle_gc_allocations_since_gc() -> u32 {
    ALLOC_SINCE_GC.load(Ordering::Relaxed)
}

/// Number of completed collection cycles (debug/stats).
#[no_mangle]
pub extern "C" fn pickle_gc_collection_count() -> u32 {
    COLLECTIONS.load(Ordering::Relaxed)
}

/// Set the auto-collect threshold in object bytes.
#[no_mangle]
pub extern "C" fn pickle_gc_set_threshold(bytes: u32) {
    AUTO_THRESHOLD.store(bytes, Ordering::Relaxed);
}

/// Serialises tests that touch the shared global `Gc`. Each test fresh-starts
/// the collector under the lock so parallel tests cannot sweep each other's
/// allocations. Returns the guard; hold it for the test's duration.
#[cfg(test)]
pub(crate) fn test_begin() -> std::sync::MutexGuard<'static, ()> {
    static TEST_LOCK: Mutex<()> = Mutex::new(());
    let guard = TEST_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    ALLOC_SINCE_GC.store(0, Ordering::Relaxed);
    BYTES_SINCE_GC.store(0, Ordering::Relaxed);
    AUTO_THRESHOLD.store(DEFAULT_AUTO_COLLECT_THRESHOLD, Ordering::Relaxed);
    COLLECTIONS.store(0, Ordering::Relaxed);
    // Drop roots left behind by earlier tests (e.g. leaked static-field
    // cells) so they can never trace objects from a destroyed heap.
    STATIC_ROOTS.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).clear();
    unsafe {
        if !GLOBAL_GC.is_null() {
            let mut gc = Box::from_raw(GLOBAL_GC);
            gc.heap.destroy();
        }
        GLOBAL_GC = Box::into_raw(Box::new(Gc::new()));
        shadow::register_thread();
    }
    guard
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Allocate a fresh user-class-like object descriptor with zero managed
    /// slots (leaf).
    fn leaf_class(gc: &mut Gc) -> u32 {
        gc.register_class(crate::object::ClassDescriptor {
            name_ptr: b"leaf\0".as_ptr(),
            name_len: 4,
            flags: 0,
            slot_count: 0,
            mask_words: 0,
            managed_mask: std::ptr::null(),
            finalizer: crate::object::builtin_nop_finalizer,
        })
    }

    #[test]
    fn rooted_object_survives_collect() {
        let _guard = test_begin();
        // Register the builtin descriptors first so user classes occupy the
        // reserved id range (>= PICKLE_CLASS_USER_BASE); otherwise `leaf_class`
        // above lands on id 0 and is mistraced as the `string` builtin.
        crate::pickle_runtime_init();
        let gc = gc_mut();
        let cls = leaf_class(gc);
        let obj = gc.alloc(48, cls);
        let root: *mut *mut PickleObject = Box::into_raw(Box::new(obj));
        let handle = pickle_gc_root_add(root);
        gc.collect();
        unsafe {
            assert!(!(*obj).is_marked(), "marks cleared after sweep");
        }
        assert_eq!(unsafe { *root }, obj, "rooted object must survive");
        pickle_gc_root_drop(handle);
        unsafe { drop(Box::from_raw(root)) };
    }

    #[test]
    fn unreachable_object_is_collected_and_reused() {
        let _guard = test_begin();
        crate::pickle_runtime_init();
        let gc = gc_mut();
        let cls = leaf_class(gc);
        let orphan = gc.alloc(64, cls);
        let addr_before = orphan as usize;
        gc.collect();
        let recycled = gc.alloc(64, cls);
        assert_eq!(
            recycled as usize, addr_before,
            "expected the swept block to be reused"
        );
    }

    #[test]
    fn auto_collect_runs_at_threshold_and_reclaims() {
        let _guard = test_begin();
        crate::pickle_runtime_init();
        let gc = gc_mut();
        let cls = leaf_class(gc);
        pickle_gc_set_threshold(256);
        assert_eq!(pickle_gc_collection_count(), 0);
        let before = pickle_gc_collection_count();
        // This allocation crosses the 256-byte threshold, collecting any
        // unrooted blocks, then returns the block.
        let orphan = gc.alloc(256, cls);
        let after = pickle_gc_collection_count();
        assert!(after > before, "auto-collect must run at the threshold");
        // The orphan was unrooted, so it was swept; the next same-size
        // allocation reuses its block.
        let recycled = gc.alloc(256, cls);
        assert_eq!(recycled as usize, orphan as usize);
        pickle_gc_set_threshold(DEFAULT_AUTO_COLLECT_THRESHOLD);
    }

    #[test]
    fn collect_clears_marks_and_keeps_live() {
        let _guard = test_begin();
        // Register the builtin descriptors first: a bare table has `cls_parent`
        // land on id 1 == PICKLE_CLASS_LIST, so `pair` gets mistraced as a list
        // and the collector walks uninitialised payload slots for `list_len`
        // bytes (a misaligned-deref on Linux, where a fresh arena can be dirty).
        crate::pickle_runtime_init();
        let gc = gc_mut();
        let cls_leaf = leaf_class(gc);
        let cls_parent = gc.register_class(crate::object::ClassDescriptor {
            name_ptr: b"pair\0".as_ptr(),
            name_len: 4,
            flags: 0,
            slot_count: 1,
            mask_words: 1,
            managed_mask: &[1u32] as *const u32,
            finalizer: crate::object::builtin_nop_finalizer,
        });
        let pair = gc.alloc(64, cls_parent);
        let child = gc.alloc(48, cls_leaf);
        unsafe {
            (*pair).set_slot(0, child);
        }
        let root: *mut *mut PickleObject = Box::into_raw(Box::new(pair));
        let handle = pickle_gc_root_add(root);
        gc.collect();
        unsafe {
            assert!(!(*pair).is_marked());
            assert!(!(*child).is_marked());
            assert_eq!((*pair).slot(0), child);
        }
        pickle_gc_root_drop(handle);
        unsafe { drop(Box::from_raw(root)) };
    }
}