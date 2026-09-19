//! Payload layouts for the builtin types (`docs/05-memory.md`).
//!
//! Every object's payload begins immediately after the 24-byte header. Slots
//! are 8-byte units.

use crate::object::{pickle_header_size, PickleObject};

pub const HEADER: usize = pickle_header_size();

/// `PString { header, len: usize, bytes: [u8] }` — inline UTF-8 bytes.
pub const STR_LEN_OFF: usize = HEADER; // usize
pub const STR_BYTES_OFF: usize = HEADER + 8; // [u8]

/// `PList { header, len: usize, cap: usize, data: *mut void }`.
pub const LIST_LEN_OFF: usize = HEADER; // usize
pub const LIST_CAP_OFF: usize = HEADER + 8; // usize
pub const LIST_DATA_OFF: usize = HEADER + 16; // *mut *mut PickleObject

/// `PMap { header, len: usize, cap: usize, entries: *mut Entry }` where
/// `Entry { key: *mut PickleObject, value: *mut PickleObject }`.
pub const MAP_LEN_OFF: usize = HEADER; // usize
pub const MAP_CAP_OFF: usize = HEADER + 8; // usize
pub const MAP_ENTRIES_OFF: usize = HEADER + 16; // *mut Entry

/// Fixed payload sizes of the builtin objects.
pub const STRING_PAYLOAD: usize = 8;
pub const LIST_PAYLOAD: usize = 24;
pub const MAP_PAYLOAD: usize = 24;

/// Fixed *object* sizes (header + payload) for the builtin object types.
/// Lists and maps need more than the 32-byte minimum allocator object, since
/// their 24-byte payload sits directly after the 24-byte header.
pub const LIST_OBJECT_SIZE: usize = HEADER + LIST_PAYLOAD;
pub const MAP_OBJECT_SIZE: usize = HEADER + MAP_PAYLOAD;

#[inline]
pub fn str_len(s: *const PickleObject) -> usize {
    unsafe { ((s as *const u8).add(STR_LEN_OFF) as *const usize).read() }
}

#[inline]
pub fn str_bytes(s: *const PickleObject) -> *const u8 {
    unsafe { (s as *const u8).add(STR_BYTES_OFF) }
}

#[inline]
pub fn str_set_len(s: *mut PickleObject, len: usize) {
    unsafe { ((s as *mut u8).add(STR_LEN_OFF) as *mut usize).write(len) }
}

#[inline]
pub fn list_len(l: *const PickleObject) -> usize {
    unsafe { ((l as *const u8).add(LIST_LEN_OFF) as *const usize).read() }
}

#[inline]
pub fn list_cap(l: *const PickleObject) -> usize {
    unsafe { ((l as *const u8).add(LIST_CAP_OFF) as *const usize).read() }
}

#[inline]
pub fn list_data(l: *const PickleObject) -> *mut *mut PickleObject {
    unsafe { ((l as *const u8).add(LIST_DATA_OFF) as *const *mut *mut PickleObject).read() }
}

#[inline]
pub fn list_set_len(l: *mut PickleObject, len: usize) {
    unsafe { ((l as *mut u8).add(LIST_LEN_OFF) as *mut usize).write(len) }
}

#[inline]
pub fn list_set_cap(l: *mut PickleObject, cap: usize) {
    unsafe { ((l as *mut u8).add(LIST_CAP_OFF) as *mut usize).write(cap) }
}

#[inline]
pub fn list_set_data(l: *mut PickleObject, data: *mut *mut PickleObject) {
    unsafe { ((l as *mut u8).add(LIST_DATA_OFF) as *mut *mut *mut PickleObject).write(data) }
}

#[inline]
pub fn map_len(m: *const PickleObject) -> usize {
    unsafe { ((m as *const u8).add(MAP_LEN_OFF) as *const usize).read() }
}

#[inline]
pub fn map_cap(m: *const PickleObject) -> usize {
    unsafe { ((m as *const u8).add(MAP_CAP_OFF) as *const usize).read() }
}

#[inline]
pub fn map_entries(m: *const PickleObject) -> *mut MapEntry {
    unsafe { ((m as *const u8).add(MAP_ENTRIES_OFF) as *const *mut MapEntry).read() }
}

#[inline]
pub fn map_set_len(m: *mut PickleObject, len: usize) {
    unsafe { ((m as *mut u8).add(MAP_LEN_OFF) as *mut usize).write(len) }
}

#[inline]
pub fn map_set_cap(m: *mut PickleObject, cap: usize) {
    unsafe { ((m as *mut u8).add(MAP_CAP_OFF) as *mut usize).write(cap) }
}

#[inline]
pub fn map_set_entries(m: *mut PickleObject, entries: *mut MapEntry) {
    unsafe { ((m as *mut u8).add(MAP_ENTRIES_OFF) as *mut *mut MapEntry).write(entries) }
}

/// An open-addressing hash-map entry.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MapEntry {
    pub key: *mut PickleObject,
    pub value: *mut PickleObject,
}

/// Total byte size of the *object* (header + payload) for a builtin.
#[inline]
pub fn builtin_string_total(n: usize) -> usize {
    crate::heap::align8(STR_BYTES_OFF + n)
}

/// An object -> raw buffer deallocator hook used during sweep.
pub type RawReleaser = unsafe extern "C" fn(*mut PickleObject, *const crate::object::DescriptorTable);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offsets_are_8aligned() {
        for off in [STR_LEN_OFF, STR_BYTES_OFF, LIST_LEN_OFF, LIST_CAP_OFF, LIST_DATA_OFF] {
            assert_eq!(off % 8, 0);
        }
        assert_eq!(std::mem::size_of::<MapEntry>(), 16);
    }
}