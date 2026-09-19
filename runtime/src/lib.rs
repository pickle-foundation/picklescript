//! `pickle-runtime`
//!
//! Static library implementing the garbage collector, object model, builtin
//! types, console bridge, and process entry point for compiled PickleScript
//! programs. See `docs/07-runtime.md` and `docs/05-memory.md`.
//!
//! The ABI is a fixed set of `pickle_*` `extern "C"` symbols plus the
//! compiler-emitted class descriptor table; the runtime provides `main` and
//! calls the compiler's `pickle_main`.

#![allow(dead_code)]
#![allow(clippy::manual_c_str_literals)]

mod console;
mod gc;
mod heap;
mod layout;
mod list;
mod map;
mod object;
mod panic;
mod shadow;
mod strings;
mod test;
mod trace;

/// Descriptor for the builtin string (informational; name used by printers).
const BUILTIN_DESCRIPTORS: &[object::ClassDescriptor] = &[
    string_desc(),
    list_desc(),
    map_desc(),
];

const fn string_desc() -> object::ClassDescriptor {
    object::ClassDescriptor {
        name_ptr: b"string\0".as_ptr(),
        name_len: 6,
        flags: 0,
        slot_count: 0,
        mask_words: 0,
        managed_mask: std::ptr::null(),
        finalizer: object::builtin_nop_finalizer,
    }
}

const fn list_desc() -> object::ClassDescriptor {
    object::ClassDescriptor {
        name_ptr: b"List\0".as_ptr(),
        name_len: 4,
        flags: 0,
        slot_count: 0,
        mask_words: 0,
        managed_mask: std::ptr::null(),
        finalizer: object::builtin_nop_finalizer,
    }
}

const fn map_desc() -> object::ClassDescriptor {
    object::ClassDescriptor {
        name_ptr: b"Map\0".as_ptr(),
        name_len: 3,
        flags: 0,
        slot_count: 0,
        mask_words: 0,
        managed_mask: std::ptr::null(),
        finalizer: object::builtin_nop_finalizer,
    }
}

/// One-time runtime bring-up: install the panic guard, initialise the
/// collector, register the builtin descriptors, and register this thread.
/// Idempotent; safe to call more than once.
#[no_mangle]
pub extern "C" fn pickle_runtime_init() -> u32 {
    panic::install_panic_hook();
    gc::init();
    let g = gc::gc_mut();
    if g.descriptors.len() < BUILTIN_DESCRIPTORS.len() {
        for d in BUILTIN_DESCRIPTORS {
            g.register_class(*d);
        }
    }
    0
}

/// Register `count` compiler-emitted class descriptors (a `[ClassDescriptor;
/// count]` table in the generated object). Returns the next free class id.
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn pickle_runtime_register_class_table(
    table: *const object::ClassDescriptor,
    count: u32,
) -> u32 {
    if table.is_null() || count == 0 {
        return gc::gc_mut().descriptors.len() as u32;
    }
    unsafe {
        let g = gc::gc_mut();
        if g.descriptors.len() < BUILTIN_DESCRIPTORS.len() {
            for d in BUILTIN_DESCRIPTORS {
                g.register_class(*d);
            }
        }
        for i in 0..count as usize {
            g.register_class(table.add(i).read());
        }
        g.descriptors.len() as u32
    }
}

/// Tear down the collector. Usually not needed at exit, but available.
#[no_mangle]
pub extern "C" fn pickle_runtime_shutdown() {
    gc::shutdown();
}

// The compiler-emitted program entry. Resolved at link time.
#[cfg(all(feature = "entry", not(test)))]
unsafe extern "C" {
    fn pickle_main();
}

// Process entry point: bootstrap the runtime, run the program, flush output.
#[cfg(all(feature = "entry", not(test)))]
#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    unsafe {
        pickle_runtime_init();
        pickle_main();
        console::pickle_flush();
        pickle_runtime_shutdown();
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_cycle_alloc_mark_sweep() {
        let _guard = crate::gc::test_begin();
        pickle_runtime_init();

        let cls = {
            let g = gc::gc_mut();
            g.register_class(object::ClassDescriptor {
                name_ptr: b"Hero\0".as_ptr(),
                name_len: 4,
                flags: 0,
                slot_count: 1,
                mask_words: 1,
                managed_mask: &[1u32] as *const u32,
                finalizer: object::builtin_nop_finalizer,
            })
        };

        // Rooted: a Hero pointing at a string.
        let hero = gc::pickle_gc_alloc(48, cls);
        let name = {
            let s = strings::string_from_bytes(b"Alice".as_ptr(), 5, gc::gc_mut());
            unsafe { (*hero).set_slot(0, s) };
            s
        };

        // Unrooted: garbage to sweep.
        let _trash = strings::string_from_bytes(b"junk".as_ptr(), 4, gc::gc_mut());

        let root: *mut *mut object::PickleObject = Box::into_raw(Box::new(hero));
        let handle = gc::pickle_gc_root_add(root);
        gc::pickle_gc_collect(true);

        unsafe {
            assert!(!(*hero).is_marked());
            assert_eq!((*hero).slot(0), name, "rooted string must survive");
        }

        // The list + map builtins survive a full cycle too.
        let list = list::list_new(0, gc::gc_mut());
        list::list_push(list, name);
        let map = map::map_new(0, gc::gc_mut());
        let key = strings::string_from_bytes(b"class".as_ptr(), 5, gc::gc_mut());
        let val = strings::string_from_bytes(b"warrior".as_ptr(), 7, gc::gc_mut());
        map::map_set(map, key, val);
        assert_eq!(map::map_get(map, key), val);

        let hero_root: *mut *mut object::PickleObject = Box::into_raw(Box::new(hero));
        let list_root: *mut *mut object::PickleObject = Box::into_raw(Box::new(list));
        let map_root: *mut *mut object::PickleObject = Box::into_raw(Box::new(map));
        let rh = gc::pickle_gc_root_add(hero_root);
        let rl = gc::pickle_gc_root_add(list_root);
        let rm = gc::pickle_gc_root_add(map_root);
        gc::pickle_gc_collect(true);

        assert_eq!(list::list_get(list, 0), name);
        assert_eq!(map::map_get(map, key), val);

        gc::pickle_gc_root_drop(handle);
        gc::pickle_gc_root_drop(rh);
        gc::pickle_gc_root_drop(rl);
        gc::pickle_gc_root_drop(rm);
        unsafe {
            drop(Box::from_raw(root));
            drop(Box::from_raw(hero_root));
            drop(Box::from_raw(list_root));
            drop(Box::from_raw(map_root));
        }
    }

    #[test]
    fn register_class_table_assigns_ids_after_builtins() {
        let _guard = crate::gc::test_begin();
        pickle_runtime_init();
        const D: object::ClassDescriptor = object::ClassDescriptor {
            name_ptr: b"State\0".as_ptr(),
            name_len: 5,
            flags: 0,
            slot_count: 0,
            mask_words: 0,
            managed_mask: std::ptr::null(),
            finalizer: object::builtin_nop_finalizer,
        };
        let start = pickle_runtime_register_class_table(&D as *const _, 1);
        assert!(start >= 3, "user classes come after the builtins");
        let g = gc::gc_mut();
        assert_eq!(g.class_name(0).map(|b| b.to_vec()), Some(b"string".to_vec()));
        assert_eq!(g.class_name(start - 1).map(|b| b.to_vec()), Some(b"State".to_vec()));
    }
}