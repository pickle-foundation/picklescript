//! Builtin `Map<K, V>` support (v1: string keys).
//!
//! Layout per `docs/05-memory.md`: `{ header, len, cap, entries: *mut Entry }`
//! with `Entry { key, value }` pairs; open addressing with linear probing and
//! a power-of-two table. The collector scans live entries as managed refs.

use crate::gc::Gc;
use crate::layout::{
    map_cap, map_entries, map_len, map_set_cap, map_set_entries, map_set_len, MapEntry,
    MAP_OBJECT_SIZE,
};
use crate::object::{PickleObject, PICKLE_CLASS_MAP};
use crate::strings::{string_bytes_len, string_bytes_ptr, string_cmp};

const INITIAL_CAP: usize = 16;
const MAX_LOAD: f64 = 0.7;

/// FNV-1a over a string's bytes.
fn fnv1a(bytes: *const u8, len: usize) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    unsafe {
        for i in 0..len {
            h ^= *bytes.add(i) as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
    }
    h
}

fn find_slot(
    entries: *mut MapEntry,
    cap: usize,
    key: *const PickleObject,
    key_hash: u64,
) -> (*mut MapEntry, bool) {
    // Returns (empty-or-match slot, found)
    unsafe {
        let mask = cap - 1;
        let mut idx = (key_hash as usize) & mask;
        let mut first_tomb: *mut MapEntry = std::ptr::null_mut();
        let key_str = |e: *mut MapEntry| (*e).key;
        loop {
            let slot = entries.add(idx);
            let k = key_str(slot);
            if k.is_null() {
                // Empty: probe `first_tomb` if we passed any.
                let target = if first_tomb.is_null() { slot } else { first_tomb };
                return (target, false);
            }
            // Tombstone check (key == the unique tombstone sentinel).
            if std::ptr::eq(k, crate::object::nil_sentinel()) {
                if first_tomb.is_null() {
                    first_tomb = slot;
                }
            } else if string_cmp(k, key) == 0 {
                return (slot, true);
            }
            idx = (idx + 1) & mask;
        }
    }
}

fn needs_grow(entries: *mut MapEntry, cap: usize, _len: usize) -> bool {
    let occupied = entries_count(entries, cap);
    (occupied as f64 + 1.0) >= (cap as f64 * MAX_LOAD)
}

fn entries_count(entries: *mut MapEntry, cap: usize) -> usize {
    unsafe {
        let mut n = 0usize;
        for i in 0..cap {
            let k = entries.add(i).read().key;
            if !k.is_null() && !std::ptr::eq(k, crate::object::nil_sentinel()) {
                n += 1;
            }
        }
        n
    }
}

fn rehash(old_entries: *mut MapEntry, old_cap: usize, new_cap: usize, gc: &mut Gc) -> *mut MapEntry {
    unsafe {
        let fresh = gc.heap.raw_alloc(new_cap * std::mem::size_of::<MapEntry>());
        std::ptr::write_bytes(fresh, 0, new_cap * std::mem::size_of::<MapEntry>());
        let fresh = fresh as *mut MapEntry;
        for i in 0..old_cap {
            let mut e = old_entries.add(i).read();
            if !e.key.is_null() && !std::ptr::eq(e.key, crate::object::nil_sentinel()) {
                let key_hash = fnv1a(string_bytes_ptr(e.key), string_bytes_len(e.key));
                let (slot, _) = find_slot(fresh, new_cap, e.key, key_hash);
                std::ptr::swap(slot, &mut e);
            }
        }
        crate::heap::raw_free(old_entries as *mut u8);
        fresh
    }
}

/// Create an empty map.
pub fn map_new(cap: usize, gc: &mut Gc) -> *mut PickleObject {
    let cap = cap.max(INITIAL_CAP).next_power_of_two();
    let obj = gc.alloc(MAP_OBJECT_SIZE as u32, PICKLE_CLASS_MAP);
    unsafe {
        map_set_len(obj, 0);
        map_set_cap(obj, cap);
        let entries = gc.heap.raw_alloc(cap * std::mem::size_of::<MapEntry>());
        std::ptr::write_bytes(
            entries,
            0,
            cap * std::mem::size_of::<MapEntry>(),
        );
        map_set_entries(obj, entries as *mut MapEntry);
    }
    obj
}

/// Insert `key -> value`. Returns the previous value, if any.
pub fn map_set(map: *mut PickleObject, key: *const PickleObject, value: *mut PickleObject) -> *mut PickleObject {
    let gc = crate::gc::gc_mut();
    unsafe {
        let key_hash = fnv1a(string_bytes_ptr(key), string_bytes_len(key));
        if needs_grow(map_entries(map), map_cap(map), map_len(map)) {
            let new_cap = map_cap(map) * 2;
            let fresh = rehash(map_entries(map), map_cap(map), new_cap, gc);
            map_set_entries(map, fresh);
            map_set_cap(map, new_cap);
        }
        let (slot, found) = find_slot(map_entries(map), map_cap(map), key, key_hash);
        let old = (*slot).value;
        (*slot).key = key as *mut PickleObject;
        (*slot).value = value;
        if !found {
            map_set_len(map, map_len(map) + 1);
        }
        old
    }
}

/// Look up `key`; returns null when absent.
pub fn map_get(map: *const PickleObject, key: *const PickleObject) -> *mut PickleObject {
    unsafe {
        let key_hash = fnv1a(string_bytes_ptr(key), string_bytes_len(key));
        // Probe with a small guard: always within cap steps.
        let cap = map_cap(map);
        let mask = cap - 1;
        let mut idx = (key_hash as usize) & mask;
        for _ in 0..cap {
            let slot = map_entries(map).add(idx);
            let k = (*slot).key;
            if k.is_null() {
                return std::ptr::null_mut();
            }
            if !std::ptr::eq(k, crate::object::nil_sentinel()) && string_cmp(k, key) == 0 {
                return (*slot).value;
            }
            idx = (idx + 1) & mask;
        }
        std::ptr::null_mut()
    }
}

/// Remove `key`. Returns the value if present, else null.
pub fn map_remove(map: *mut PickleObject, key: *const PickleObject) -> *mut PickleObject {
    unsafe {
        let key_hash = fnv1a(string_bytes_ptr(key), string_bytes_len(key));
        let cap = map_cap(map);
        let mask = cap - 1;
        let mut idx = (key_hash as usize) & mask;
        for _ in 0..cap {
            let slot = map_entries(map).add(idx);
            let k = (*slot).key;
            if k.is_null() {
                return std::ptr::null_mut();
            }
            if !std::ptr::eq(k, crate::object::nil_sentinel()) && string_cmp(k, key) == 0 {
                let v = (*slot).value;
                (*slot).key = crate::object::nil_sentinel() as *mut PickleObject;
                (*slot).value = std::ptr::null_mut();
                map_set_len(map, map_len(map) - 1);
                return v;
            }
            idx = (idx + 1) & mask;
        }
        std::ptr::null_mut()
    }
}

/// Number of live entries.
pub fn map_len_of(map: *const PickleObject) -> usize {
    map_len(map)
}

/// Materialize the live keys (or values) as a fresh `List`.
fn map_snapshot(map: *const PickleObject, want_keys: bool) -> *mut PickleObject {
    let gc = crate::gc::gc_mut();
    let l = crate::list::list_new(0, gc);
    let cap = map_cap(map);
    let entries = map_entries(map);
    unsafe {
        for i in 0..cap {
            let e = &*entries.add(i);
            if e.key.is_null() || std::ptr::eq(e.key, crate::object::nil_sentinel()) {
                continue;
            }
            let val = if want_keys { e.key } else { e.value };
            crate::list::list_push(l, val);
        }
    }
    l
}

// ---- Exported ABI ----------------------------------------------------------
// Thin `extern "C"` wrappers over the internal Rust APIs. Keys must already be
// string objects; scalar values must be boxed first (`boxscalar::pickle_box_*`)
// before being stored, and unboxed after reading.

/// Create an empty map; `cap` is a requested capacity hint (a power of two).
#[no_mangle]
pub extern "C" fn pickle_map_new(cap: usize) -> *mut PickleObject {
    let gc = crate::gc::gc_mut();
    map_new(cap, gc)
}

/// Insert `key -> value`. The previous value (if any) is dropped.
#[no_mangle]
pub extern "C" fn pickle_map_set(
    map: *mut PickleObject,
    key: *const PickleObject,
    value: *mut PickleObject,
) {
    let _ = map_set(map, key, value);
}

/// Look up `key`; when absent, returns `default` instead of null so compiled
/// code never unboxes a null pointer.
#[no_mangle]
pub extern "C" fn pickle_map_get_boxed(
    map: *const PickleObject,
    key: *const PickleObject,
    default: *mut PickleObject,
) -> *mut PickleObject {
    let got = map_get(map, key);
    if got.is_null() {
        default
    } else {
        got
    }
}

/// Whether `key` is present.
#[no_mangle]
pub extern "C" fn pickle_map_has(map: *const PickleObject, key: *const PickleObject) -> bool {
    !map_get(map, key).is_null()
}

/// Remove `key`; returns the removed value (a boxed scalar or managed
/// pointer) when present, else null — exactly the `T?` option representation,
/// so compiled code can hand the result straight to an option-typed slot.
#[no_mangle]
pub extern "C" fn pickle_map_remove(
    map: *mut PickleObject,
    key: *const PickleObject,
) -> *mut PickleObject {
    map_remove(map, key)
}

/// Number of live entries.
#[no_mangle]
pub extern "C" fn pickle_map_len(map: *const PickleObject) -> usize {
    map_len_of(map)
}

/// All live keys as a fresh `List` (string pointers).
#[no_mangle]
pub extern "C" fn pickle_map_keys(map: *const PickleObject) -> *mut PickleObject {
    map_snapshot(map, true)
}

/// All live values as a fresh `List` (boxed/pointer values).
#[no_mangle]
pub extern "C" fn pickle_map_values(map: *const PickleObject) -> *mut PickleObject {
    map_snapshot(map, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> std::sync::MutexGuard<'static, ()> {
        crate::gc::test_begin()
    }

    #[test]
    fn insert_lookup_remove() {
        let _guard = setup();
        let gc = crate::gc::gc_mut();
        let m = map_new(0, gc);
        let k1 = crate::strings::string_from_bytes(b"alpha".as_ptr(), 5, gc);
        let k2 = crate::strings::string_from_bytes(b"beta".as_ptr(), 4, gc);
        let v1 = crate::strings::int64_to_string(11);
        let v2 = crate::strings::int64_to_string(22);
        assert!(map_get(m, k1).is_null());
        assert!(map_set(m, k1, v1).is_null());
        assert_eq!(map_get(m, k1), v1);
        assert!(map_get(m, k2).is_null());
        assert!(map_set(m, k2, v2).is_null());
        assert_eq!(map_get(m, k2), v2);
        assert_eq!(map_len_of(m), 2);
        // Overwrite returns old.
        let v3 = crate::strings::int64_to_string(33);
        assert_eq!(map_set(m, k1, v3), v1);
        assert_eq!(map_get(m, k1), v3);
        // Remove.
        assert_eq!(map_remove(m, k2), v2);
        assert!(map_get(m, k2).is_null());
        assert_eq!(map_len_of(m), 1);
    }

    #[test]
    fn many_keys_grow() {
        let _guard = setup();
        let gc = crate::gc::gc_mut();
        let m = map_new(0, gc);
        let mut expected: Vec<*mut PickleObject> = Vec::new();
        for i in 0..500usize {
            let k = crate::strings::int64_to_string(i as i64);
            let v = crate::strings::int64_to_string((i * 7) as i64);
            expected.push(v);
            map_set(m, k, v);
        }
        for (i, v) in expected.iter().enumerate() {
            let k = crate::strings::int64_to_string(i as i64);
            assert_eq!(map_get(m, k), *v, "key {i}");
        }
        assert_eq!(map_len_of(m), 500);
    }

    #[test]
    fn abi_roundtrip_boxed_ints() {
        let _guard = setup();
        crate::pickle_runtime_init();
        let m = crate::map::pickle_map_new(0);
        let k1 = crate::strings::string_from_bytes(b"alpha".as_ptr(), 5, crate::gc::gc_mut());
        let k2 = crate::strings::string_from_bytes(b"beta".as_ptr(), 4, crate::gc::gc_mut());
        crate::map::pickle_map_set(m, k1, crate::boxscalar::pickle_box_i64(7));
        crate::map::pickle_map_set(m, k2, crate::boxscalar::pickle_box_i64(-3));
        assert!(crate::map::pickle_map_has(m, k1));
        let missing = crate::strings::string_from_bytes(b"gamma".as_ptr(), 5, crate::gc::gc_mut());
        assert!(!crate::map::pickle_map_has(m, missing));
        assert_eq!(crate::map::pickle_map_len(m), 2);
        let default = crate::boxscalar::pickle_box_i64(0);
        let v1 = crate::map::pickle_map_get_boxed(m, k1, default);
        assert_eq!(crate::boxscalar::pickle_unbox_i64(v1), 7);
        let missing = crate::strings::string_from_bytes(b"gamma".as_ptr(), 5, crate::gc::gc_mut());
        assert_eq!(
            crate::boxscalar::pickle_unbox_i64(crate::map::pickle_map_get_boxed(m, missing, default)),
            0
        );
        let keys = crate::map::pickle_map_keys(m);
        assert_eq!(crate::list::pickle_list_len(keys), 2);
        let vals = crate::map::pickle_map_values(m);
        assert_eq!(crate::list::pickle_list_len(vals), 2);
    }
}