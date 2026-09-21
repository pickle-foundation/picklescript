# PickleScript Standard Library

Source libraries for the standard library (`std.*`). Written in PickleScript,
with compiler intrinsics only where the runtime ABI (`pickle_*`) needs a thin
wrapper.

Seeds (see `docs/08-stdlib.md`, office `port/` tracker):

- `text/` — string/byte builder, UTF-8 helpers, format; first seed
  `text/lexer.pkl` (the compiler lexer's scanner; validated, runs on the whole
  battery — see `tools/diff/port/`)
- `collections/` — `List`/`Map` wrappers plus `Set`, `Deque`, `Vec`
- `fs/`, `os/`, `math/`, `time/`, `test/` — planned (see the plan doc)

Status: first seed written and validated (`text/lexer.pkl`);
word its canonical token dump matches the Rust lexer byte-for-byte on all
battery files (44/44), and its battery `main` regenerates `tools/diff/port/`.
The compiler registers builtins (`print`, `println`, `len`, `abs`, ...)
directly in the resolver, and the runtime ABI ships the foundation layer as
intrinsics: collection verbs (`List` push/pop/insert/remove/sort, `Map`
has/get/remove/keys/values/`[]`/entries), whole-file I/O
(`read_file`/`write_file`/`file_exists`/`delete`/`mkdir`/`list_dir`), the
buffered `Stream` layer, the `bytes()`/`str(List<byte>)` bridge, and the math
builtins (`min`/`max`/`clamp`/`range`/`abs`). `std.*` modules will be thin
pickle wrappers over these plus genuinely new code.