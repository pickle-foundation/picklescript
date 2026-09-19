# PickleScript — Memory Model

## Default: garbage collected

Normal PickleScript code never mentions memory.

```
class Account {
    name: string
    transactions: List<int>
}

fn main() {
    let a = Account("Ada")
    let b = a                // shared reference
    let copy = Object.clone(a) // only if you ask
}
```

- Assignments copy *values* (structs, primitives) and share *references*
  (classes, strings, collections).
- Objects are allocated on a GC heap, moved in/out of functions by
  reference. No ownership, no lifetimes, no let/const/borrow ceremony in
  safe code.
- Locals, fields, collection elements are all managed uniformly.
- Strings are immutable and UTF-8; substring/slice operations copy.

## Collector

The v1 collector is a **non-moving, stop-the-world, mark-and-sweep GC**
with the following properties:

- Threads report live references through a **shadow stack** managed by the
  compiler: the function prologue allocates a fixed-size slot frame,
  registers it with the runtime's thread-local GC data, and codegen spills
  every live managed pointer into its slot around each GC safe point
  (allocation, function call, channel send, await).
- Allocation is a bump/small-region allocation over a free-list triaged by
  size class.
- Sweeping returns dead objects to the free list; consecutive free blocks
  coalesce.
- Weak references exist (`weak T`) but do not keep objects alive.
- Finalizers (`deinit`) run during sweep, off the allocation path, in
  unspecified order; they must not allocate.
- The GC is **incremental-izable** later; v1 reserves a stop-the-world
  pause for simplicity and predictability.

## Safe vs unsafe

```
let account = Account("Ada")          // safe: GC

unsafe {
    let ptr: *Account = &account      // raw address
    ptr->branch = "NYC"
    let selfPtr: *selfObject = ...    // whatever you need
}
```

Inside `unsafe { }` you get:

- `*T` raw pointers (taking `&` address-of, dereferencing `*p`, field access
  `p->field`, index `p[i]`, arithmetic `p + n`).
- `&T` references (`fn update(v: &int)`) for out/inout style parameters —
  the referenced object is rooted on the shadow stack for the call's
  duration.
- Direct allocation through `alloc(T)`, `allocArray`, and allocator hooks:
  `withAllocator(a) { ... }`.
- Calling foreign (C) functions — not in v1.

Rules enforced by the type checker inside `unsafe`:
- A raw pointer derived from a managed object must not outlive a GC point in
  a way the compiler can see; the compiler inserts implicit roots for live
  referents of `&T` params.
- Casting between pointer kinds is explicit.

## Custom allocators and arenas

For engine-style code:

```
arena Arena<SceneData> {           // region allocation
    ...
}

let scratch = Arena()
let item = scratch.alloc(Vec<Collider>())
// freed as a whole when scratch goes out of scope
```

Design: `Arena<T>` is a class in the stdlib; custom *global* allocators are
registered through the runtime (`SetGlobalAllocator`) and participate in GC
as non-moving spaces in v1. Manual heap (`malloc`/`free`) is
`unsafe`-gated.

## Pointers and references — precise rules

| Syntax          | Meaning                                  |
|-----------------|------------------------------------------|
| `&expr`         | address of a value (stack or managed)    |
| `*T`            | raw pointer type (unsafe only)           |
| `ptr->field`    | field through raw pointer                |
| `*ptr`          | dereference                              |
| `T&`? — no, `&T` | reference as a parameter (rooted)       |
| `p[n]`          | index through pointer (unsafe)           |

`*T` and `&T` do not own memory. There is no manual free in safe code.

## GC and the C side

The runtime exports a C ABI (`extern "C"`) surface:

- `pickle_gc_alloc`, `pickle_gc_alloc_string`
- `pickle_gc_register_thread` / `pickle_gc_unregister_thread`
- `pickle_gc_collect(force: bool)`
- `pickle_allocator_set` (custom allocator)

The compiler generates code against this ABI; a future FFI layer reuses the
same surface.

## Object layout

```
struct PickleObjectHeader {
    next:      *mut PickleObject,  // GC free-list/live-list link
    class_id:  u32,
    flags:     u32,                // mark, etc.
    size:      u32,                // total object bytes
    _pad:      u32,
}   // 24 bytes

struct PString  { header, len: usize, bytes: [u8] }
struct PList    { header, len, cap, data: *mut void }  // element-kind tag in class table
struct PMap     { header, entries: *mut Entry, len, cap }
```

Class descriptors are emitted by the compiler into the generated object
image; each carries `field_count`, a managed-field bitmask, and a name, so
the collector marks exactly the pointer-bearing fields and no others.

## Safety guarantees summarized

- Safe code: no use-after-free, no double-free, no null deref (options
  checked statically), no buffer overrun in bounds-checked indexing.
- Unsafe code: the developer is responsible for the same invariants the GC
  would have preserved; mismatches are caught best-effort in debug builds
  (`-D` enables poison + bounds checks everywhere).
- All heap allocation paths are GC safe points by construction.