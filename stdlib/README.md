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
builtins (`print`, `println`, `len`, `abs`, ...) directly in the resolver.