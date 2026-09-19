//! User class (object) support.
//!
//! Instances are plain objects whose payload is a dense array of 8-byte slots,
//! one per instance field, at `HEADER + 8*i`. Every slot holds a managed
//! pointer (boxed scalars or object references) exactly like a list element,
//! so class descriptors mark *all* `slot_count` slots as managed. The
//! descriptor for each user class is registered once at program start by
//! `pickle_class_register` (emitted by the compiler ahead of `main`), which
//! copies the class name and the managed-mask words into stable memory.

use crate::gc::Gc;
use crate::object::{
    builtin_nop_finalizer, ClassDescriptor, PickleObject, PICKLE_CLASS_USER_BASE,
};
use crate::object::pickle_header_size;

/// Total byte size of a class instance with `field_count` managed slots.
pub fn class_object_size(field_count: usize) -> usize {
    pickle_header_size() + 8 * field_count
}

/// Create an object of user class `class_id` holding `field_count` null field
/// slots. Slots are zeroed so uninitialised fields are never traced.
pub fn class_new(class_id: u64, field_count: usize, gc: &mut Gc) -> *mut PickleObject {
    let size = class_object_size(field_count);
    let obj = gc.alloc(size as u32, class_id as u32);
    unsafe {
        if field_count > 0 {
            std::ptr::write_bytes((*obj).payload_mut(), 0, 8 * field_count);
        }
    }
    obj
}

/// Read field slot `index`; null when out of bounds (the slot must hold a
/// managed pointer, so an in-range zeroed slot reads as null).
pub fn class_slot(obj: *const PickleObject, index: i64) -> *mut PickleObject {
    unsafe {
        if obj.is_null() || index < 0 {
            return std::ptr::null_mut();
        }
        let idx = index as u64;
        let size = (*obj).size as u64;
        if size < (pickle_header_size() as u64 + 8) {
            return std::ptr::null_mut();
        }
        let slots = (size - pickle_header_size() as u64 - 8) / 8;
        if idx > slots {
            return std::ptr::null_mut();
        }
        (*obj).slot(idx as usize)
    }
}

/// Write field slot `index`; silently ignored when out of bounds (the
/// compiler emits only statically-known indices within the field count).
pub fn class_set_slot(obj: *mut PickleObject, index: i64, value: *mut PickleObject) {
    unsafe {
        if obj.is_null() || index < 0 {
            return;
        }
        let idx = index as u64;
        let size = (*obj).size as u64;
        if size < (pickle_header_size() as u64 + 8) {
            return;
        }
        let slots = (size - pickle_header_size() as u64 - 8) / 8;
        if idx > slots {
            return;
        }
        (*obj).set_slot(idx as usize, value);
    }
}

// ---- Exported ABI ----------------------------------------------------------
// Thin `extern "C"` wrappers so compiled programs can allocate class instances
// and read/write their fields. Values must be boxed first
// (`boxscalar::pickle_box_*`), matching list-element rules.

/// Register one user-class descriptor (copies the name and mask into stable
/// heap memory) and return its assigned class id.
#[no_mangle]
pub extern "C" fn pickle_class_register(
    name_ptr: *const u8,
    name_len: usize,
    slot_count: usize,
    mask: u64,
) -> u32 {
    class_register(name_ptr, name_len, slot_count, mask)
}

fn class_register(name_ptr: *const u8, name_len: usize, slot_count: usize, mask: u64) -> u32 {
    let name: Box<[u8]> = if name_ptr.is_null() || name_len == 0 {
        Box::default()
    } else {
        // SAFETY: the caller (compiler-emitted code) passes a module string's
        // bytes with a matching length.
        unsafe { std::slice::from_raw_parts(name_ptr, name_len) }.to_vec().into_boxed_slice()
    };
    let nwords = slot_count.div_ceil(32);
    let mut words = vec![0u32; nwords];
    for i in 0..slot_count {
        if i < 64 && mask & (1u64 << i) != 0 {
            words[i / 32] |= 1 << (i % 32);
        }
    }
    // Leaked deliberately: descriptors outlive every collection and are never
    // freed; the volumes are tiny (one copy per user class).
    let name_copy = Box::leak(name);
    let mask_copy = Box::leak(words.into_boxed_slice());
    let d = ClassDescriptor {
        name_ptr: name_copy.as_ptr(),
        name_len: name_len as u32,
        flags: 0,
        slot_count: slot_count as u32,
        mask_words: mask_copy.len() as u32,
        managed_mask: mask_copy.as_ptr(),
        finalizer: builtin_nop_finalizer,
    };
    let g = crate::gc::gc_mut();
    g.register_class(d)
}

/// Allocate an instance of user `class_id` with `slot_count` null field slots.
#[no_mangle]
pub extern "C" fn pickle_class_new(class_id: i64, slot_count: i64) -> *mut PickleObject {
    let gc = crate::gc::gc_mut();
    let n = if slot_count < 0 { 0 } else { slot_count as usize };
    let id = if class_id < PICKLE_CLASS_USER_BASE as i64 {
        PICKLE_CLASS_USER_BASE as u64
    } else {
        class_id as u64
    };
    class_new(id, n, gc)
}

/// A field slot (managed pointer); null when out of bounds.
#[no_mangle]
pub extern "C" fn pickle_obj_slot_get(
    obj: *const PickleObject,
    slot: i64,
) -> *mut PickleObject {
    class_slot(obj, slot)
}

/// Write a field slot.
#[no_mangle]
pub extern "C" fn pickle_obj_slot_set(
    obj: *mut PickleObject,
    slot: i64,
    value: *mut PickleObject,
) {
    class_set_slot(obj, slot, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> std::sync::MutexGuard<'static, ()> {
        crate::gc::test_begin()
    }

    #[test]
    fn new_zeroes_slots() {
        let _guard = setup();
        let gc = crate::gc::gc_mut();
        unsafe {
            let o = class_new(7, 3, gc);
            for i in 0..3 {
                assert!(class_slot(o, i).is_null());
            }
            assert_eq!((*o).class_id, 7);
            assert_eq!((*o).size, (pickle_header_size() + 24) as u32);
        }
    }

    #[test]
    fn get_set_roundtrip() {
        let _guard = setup();
        let gc = crate::gc::gc_mut();
        let o = class_new(7, 2, gc);
        let a = crate::boxscalar::pickle_box_i64(10);
        let b = crate::boxscalar::pickle_box_i64(20);
        class_set_slot(o, 0, a);
        class_set_slot(o, 1, b);
        assert_eq!(crate::boxscalar::box_bits(class_slot(o, 0)), 10);
        assert_eq!(crate::boxscalar::box_bits(class_slot(o, 1)), 20);
        assert!(class_slot(o, 2).is_null(), "out-of-range reads null");
        assert!(class_slot(o, -1).is_null(), "negative indices read null");
        assert!(class_slot(o, 1_000_000).is_null());
    }

    #[test]
    fn register_copies_name_and_mask() {
        let _guard = crate::gc::test_begin();
        crate::pickle_runtime_init();
        let name = b"Hero".to_vec();
        let id = pickle_class_register(name.as_ptr(), name.len(), 4, 0b1111);
        assert!(id >= PICKLE_CLASS_USER_BASE, "user classes come after the builtins");
        let g = crate::gc::gc_mut();
        assert_eq!(g.class_name(id).map(|b| b.to_vec()), Some(b"Hero".to_vec()));
        let d = g.descriptors.get(id).unwrap();
        assert_eq!(d.slot_count, 4);
        assert_eq!(d.mask_words, 1);
        assert_eq!(unsafe { *d.managed_mask }, 0b1111);
    }

    #[test]
    fn gc_marks_through_field_slots() {
        // Every field slot is a managed pointer, so a descriptor with an
        // all-ones mask must keep reachable fields alive across a trace.
        let _guard = crate::gc::test_begin();
        use crate::heap::Heap;
        use crate::object::DescriptorTable;
        let heap = Heap::new();
        // Dummies for ids 0..6 (matching the reserved builtin range), then the
        // class descriptor at id 7, the id a class-new object carries.
        let mut desc = DescriptorTable::new();
        let dummy = ClassDescriptor {
            name_ptr: b"x\0".as_ptr(),
            name_len: 1,
            flags: 0,
            slot_count: 0,
            mask_words: 0,
            managed_mask: std::ptr::null(),
            finalizer: builtin_nop_finalizer,
        };
        for _ in 0..(PICKLE_CLASS_USER_BASE) {
            desc.register(dummy);
        }
        desc.register(ClassDescriptor {
            name_ptr: b"Hero\0".as_ptr(),
            name_len: 4,
            flags: 0,
            slot_count: 1,
            mask_words: 1,
            managed_mask: &[1u32] as *const u32,
            finalizer: builtin_nop_finalizer,
        });
        let gc = crate::gc::gc_mut();
        unsafe {
            let payload = crate::boxscalar::pickle_box_i64(99);
            let o = class_new(7, 1, gc);
            class_set_slot(o, 0, payload);
            assert!(!(*o).is_marked());
            assert!(!(*payload).is_marked());
            let root: *mut PickleObject = o;
            let roots: Vec<*mut *mut PickleObject> =
                vec![(&root as *const *mut PickleObject) as *mut _];
            crate::trace::trace_from_roots(&desc, &roots);
            assert!((*o).is_marked());
            assert!((*payload).is_marked(), "field slot must be traced");
        }
        let _ = heap;
        let _ = gc;
    }
}