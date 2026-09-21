# PickleScript Standard Library

Source libraries for the standard library (`std.*`). Written in PickleScript,
with compiler intrinsics only where the runtime ABI (`pickle_*`) needs a thin
wrapper.

Planned modules (see `docs/08-stdlib.md`):

- `core/` — strings, math, collections, errors (foundation)
- `filesystem/`, `system/`
- `networking/`, `http/`, `json/`, `crypto/`
- `database/` — mysql, sqlite, postgres bridges (native extensions)

Status: Milestone 5. Not yet compiled — the compiler currently registers
builtins (`print`, `println`, `len`, `abs`, ...) directly in the resolver, and
the runtime ABI ships the foundation layer as intrinsics: collection verbs
(`List` push/pop/insert/remove/sort, `Map` has/get/remove/keys/values/`[]`/
entries), whole-file I/O (`read_file`/`write_file`/`file_exists`/`delete`/
`mkdir`/`list_dir`), the `bytes()`/`str(List<byte>)` bridge, and the math
builtins (`min`/`max`/`clamp`/`range`/`abs`). `std.*` modules will be thin
pickle wrappers over these plus genuinely new code (see `docs/08-stdlib.md`).