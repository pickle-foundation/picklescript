//! Builtin enum support.
//!
//! Layout per `docs/05-memory.md`: `PEnum { header, tag: i64, fields: [] }`.
//! The tag at payload slot 0 is *not* a managed pointer; the payload fields
//! (payload slot 1..) are managed and get traced by the collector. Fields are
//! boxed scalars or object pointers, exactly like list elements, so the field
//! count is recovered at runtime from the object's stored `size`.

use crate::gc::Gc;
use crate::layout::{
    enum_field_count, enum_object_size, enum_set_tag, enum_tag, ENUM_FIELDS_OFF,
};
use crate::object::{PickleObject, PICKLE_CLASS_ENUM};

/// Create an enum object holding `tag` and `field_count` null payload fields.
pub fn enum_new(tag: i64, field_count: usize, gc: &mut Gc) -> *mut PickleObject {
    let size = enum_object_size(field_count);
    let obj = gc.alloc(size as u32, PICKLE_CLASS_ENUM);
    enum_init(obj, tag, field_count);
    obj
}

/// Write `tag` and zero every payload field of an already-allocated enum.
pub(crate) fn enum_init(obj: *mut PickleObject, tag: i64, field_count: usize) {
    unsafe {
        enum_set_tag(obj, tag);
        // Zero the payload fields so uninitialised slots are never traced.
        // `ENUM_FIELDS_OFF` is measured from the object start, and
        // `size - ENUM_FIELDS_OFF` is exactly the byte length of the field
        // region (the tag occupies the 8 bytes before it).
        if field_count > 0 {
            let size = enum_object_size(field_count);
            let fields = (obj as *mut u8).add(ENUM_FIELDS_OFF);
            std::ptr::write_bytes(fields, 0, size - ENUM_FIELDS_OFF);
        }
    }
}

/// Write a payload field (unchecked bounds; callers pass constant indices into
/// a freshly constructed enum of matching width).
pub fn enum_set_field(obj: *mut PickleObject, index: usize, value: *mut PickleObject) {
    unsafe {
        let n = enum_field_count(obj);
        assert!(index < n, "pickle: enum field index out of bounds");
        let base = (*obj).payload_mut() as *mut *mut PickleObject;
        base.add(1 + index).write(value);
    }
}

/// Read a payload field; null when the index is out of bounds.
pub fn enum_field(obj: *const PickleObject, index: usize) -> *mut PickleObject {
    unsafe {
        if index >= enum_field_count(obj) {
            return std::ptr::null_mut();
        }
        let base = (*obj).payload() as *const *mut PickleObject;
        base.add(1 + index).read()
    }
}

/// The variant tag of an enum object.
pub fn enum_tag_of(obj: *const PickleObject) -> i64 {
    enum_tag(obj)
}

// ---- Exported ABI ----------------------------------------------------------
// Thin `extern "C"` wrappers over the internal Rust APIs so compiled programs
// can construct enum values and inspect their tag/fields. Payload fields must
// be boxed first (`boxscalar::pickle_box_*`), matching list-element rules.

/// Create an enum object with `tag` and `field_count` null payload fields.
#[no_mangle]
pub extern "C" fn pickle_enum_new(tag: i64, field_count: usize) -> *mut PickleObject {
    let gc = crate::gc::gc_mut();
    enum_new(tag, field_count, gc)
}

/// Write a payload field.
#[no_mangle]
pub extern "C" fn pickle_enum_set_field(
    obj: *mut PickleObject,
    index: usize,
    value: *mut PickleObject,
) {
    enum_set_field(obj, index, value)
}

/// The variant tag.
#[no_mangle]
pub extern "C" fn pickle_enum_tag(obj: *const PickleObject) -> i64 {
    enum_tag_of(obj)
}

/// A payload field; null when out of bounds.
#[no_mangle]
pub extern "C" fn pickle_enum_field(
    obj: *const PickleObject,
    index: usize,
) -> *mut PickleObject {
    enum_field(obj, index)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> std::sync::MutexGuard<'static, ()> {
        crate::gc::test_begin()
    }

    #[test]
    fn new_zeroes_payload_and_stores_tag() {
        let _guard = setup();
        let gc = crate::gc::gc_mut();
        unsafe {
            let e = enum_new(2, 3, gc);
            assert_eq!(enum_tag_of(e), 2);
            assert_eq!(enum_field_count(e), 3);
            for i in 0..3 {
                assert!(enum_field(e, i).is_null());
            }
            assert!(enum_field(e, 5).is_null(), "out of range reads null");
            assert_eq!((*e).class_id, PICKLE_CLASS_ENUM);
        }
    }

    #[test]
    fn new_zeroes_reused_dirty_memory() {
        // Regression: the field-zeroing write must start at the object-relative
        // `ENUM_FIELDS_OFF`, not at `payload + ENUM_FIELDS_OFF` (which is
        // HEADER bytes too far and leaves the whole field region dirty). We
        // poison the region first so the check is deterministic regardless of
        // whether the allocator hands back zeroed pages.
        let _guard = setup();
        let gc = crate::gc::gc_mut();
        unsafe {
            let size = crate::layout::enum_object_size(3);
            let e = gc.alloc(size as u32, PICKLE_CLASS_ENUM);
            let fields = (e as *mut u8).add(ENUM_FIELDS_OFF);
            std::ptr::write_bytes(fields, 0xAB, size - ENUM_FIELDS_OFF);
            enum_init(e, 7, 3);
            assert_eq!(enum_tag_of(e), 7);
            for i in 0..3 {
                assert!(enum_field(e, i).is_null(), "field {i} not re-zeroed");
            }
        }
    }

    #[test]
    fn set_and_read_fields() {
        let _guard = setup();
        let gc = crate::gc::gc_mut();
        let a = crate::boxscalar::pickle_box_i64(10);
        let b = crate::boxscalar::pickle_box_i64(20);
        let e = enum_new(0, 2, gc);
        enum_set_field(e, 0, a);
        enum_set_field(e, 1, b);
        assert_eq!(crate::boxscalar::box_bits(enum_field(e, 0)), 10);
        assert_eq!(crate::boxscalar::box_bits(enum_field(e, 1)), 20);
    }

    #[test]
    fn gc_marks_through_payload_fields() {
        // A unit variant (no fields) holds no refs; a payload variant must be
        // traced through its field slots but never through the tag slot.
        let _guard = crate::gc::test_begin();
        use crate::heap::Heap;
        use crate::object::DescriptorTable;
        let heap = Heap::new();
        let desc = DescriptorTable::new();
        let gc = crate::gc::gc_mut();
        unsafe {
            let payload = crate::boxscalar::pickle_box_i64(99);
            let e = enum_new(1, 1, gc);
            enum_set_field(e, 0, payload);
            assert!(!(*e).is_marked());
            assert!(!(*payload).is_marked());

            let root: *mut PickleObject = e;
            let roots: Vec<*mut *mut PickleObject> =
                vec![(&root as *const *mut PickleObject) as *mut _];
            crate::trace::trace_from_roots(&desc, &roots, &[], &[]);

            assert!((*e).is_marked());
            assert!((*payload).is_marked(), "payload field must be traced");
        }
        let _ = heap;
        let _ = gc;
    }
}