# PickleScript — Standard Library Plan

Namespace: everything lives under modules, mirroring the module system.

## Layout

```
use std.io          print, println, readLine, stderr
use std.fs          File, Dir, Path, readFile, writeFile, exists
use std.net         Tcp, Udp, Tls, Host, resolve
use std.http        Client, Request, Response, Server, get, post
use std.json        Json, parse, serialize, stringify
use std.collections List, Map, Set, Deque, Vec, Stack, Queue, OrderedMap, PriorityQueue
use std.math        sin, cos, sqrt, abs, min, max, pow, floor, ceil, random, PI, E, TAU
use std.crypto       sha256, md5, aes, hmac, randBytes, base64, hex
use std.process     run, spawn, capture, exit, env
use std.thread      thread, channel, task, parallel, Mutex, Atomic, Semaphore, Once
use std.time        now, Instant, Duration, sleep, Timer, DateTime
use std.text        String buffer, Unicode helpers, scan/format, regex
use std.gc          collect, stats, Weak, Arena
use std.os          args, exit, tempDir, homeDir, cwd, platform
use std.test        assert, assertEqual, assertThrows, bench
```

## Design principles

1. **The stdlib is written in PickleScript** on top of a thin runtime
   layer (the `pickle_*` runtime ABI). Compiler intrinsics are only where
   the language cannot express something (allocation, atomic ops).
2. Consistent naming: verbs do (`readFile`), nouns are (`File`, `Dir`);
   no `getX()`/`setX()` — properties instead.
3. Errors are values, not exceptions: functions return `T?` or
   `Result<T, Error>`; postfix `?` unwraps an option or panics with
   context. A `try`/`catch`-with-patterns form is reserved for cross-module
   failures and marked experimental.
4. Async versions are `await`-able on the same name as sync (`http.get`
   and `await http.get` share the implementation; extra `Async` suffixes
   are banned style).
5. Collections: uniform interpolation, slicing, and iteration protocol
   (interface `Iterable<T>` -> `Iterator<T>` with `next()`).

## What ships in `std` (self-hosting-driven split)

A lot of the primitive surface already lives below `std` in the runtime
`pickle_*` ABI + compiler intrinsics (`print`/`println`/`flush`, `str()`
conversions, `min`/`max`/`clamp`/`abs`/`range`, box/unbox for `T?`, list
push/pop, map has/keys/values, string cmp/len, `pickle_runtime_reset`).
`std` itself is the **PickleScript-written layer on top of that ABI** — the
modules the self-hosted compiler will actually import.

**The bytes↔string bridge is an intrinsic now:** `bytes(s: string) -> List<byte>`
returns a byte-exact copy of the string's UTF-8 bytes (with NUL bytes
preserved), and `str(xs: List<byte>) -> string` is the exact inverse — the
builtin `str()`, already used for scalar conversion, additionally accepts a
`List<byte>` and reassembles a string from those bytes. `s[i]` and
`for (b in s)` already read the same bytes as `bytes(s)[i]` (both typed
`byte` per the model in `04-types.md`). These compose the `std.text` bridge:
```
let bs = bytes("café")   // List<byte>: c a f é(2 bytes)
let back = str(bs)       // "café"
```

**Must-have (blocks rewriting the compiler in Pickle):**
- `std.text` — mutable string/byte builder, UTF-8 helpers, `format`, and a
  bytes↔string bridge. A compiler reads source text and emits source text.
- `std.collections` — `List`/`Map` (largely present) plus `remove`/`insert`,
  a hash `Set`, and string interning for symbol tables.
- `std.fs` — `readFile`/`writeFile`/`exists` (file I/O). The whole-file
  intrinsics (`read_file`/`write_file`/`file_exists`) shipped first; streams
  and directory listing remain.
- `std.os` — `args`, `exit`, env for the `pickle` driver.
- `std.math` — `pow`/`floor`/`ceil`/`sin`/`cos`/`sqrt`, `PI`/`E`/`TAU`.
- `std.test` — the `expect`/hooks/`bench` layer already steer the `.pkl`
  suites; `std.test` gave them a home.
- `std.time` — `now`/`Instant`/`Duration` for `analyzing N files`-style
  diagnostics.

**Coverage (the real-workload clause of 01-philosophy; often 1.x):**
- `std.json` (parse/serialize — first-class for tooling / a language server).
- `std.net` + `std.http` — typed client/server, streaming bodies.
- `std.process` / `std.thread` — run/spawn/capture, channels, mutexes.
- `std.crypto` — pure-Rust, no OpenSSL (SHA/HMAC/AES/ChaCha20 + base64/hex).
- `std.gc` — stats/collect; arenas are the optional self-host memory answer.

**Deliberately not in `std`:** reflexive sugar owned by the intrinsic/runtime
layer (`print` family), and compiler-internal machinery that belongs in the
compiler itself rather than the standard library.

## Collections quick view

- `List<T>` — resizeable array, indexing, `add/remove/insert`, slicing.
- `Map<K, V>` — hash map, insertion ordered, `[]` access, `has`,
  `keys()`, enumerate `for ((k, v) in map)`.
- `Set<T>` — hash set.
- `Deque<T>` — double-ended queue.
- `Vec<T>` — SIMD-friendly fixed-capacity buffer (math-heavy code).

Literals:
```
let xs = [1, 2, 3]
let m  = { "a": 1, "b": 2 }
let s  = {1, 2, 3}       // Set when element type is monomorphic
let bs = [byte] { 1, 2 }
```

Compiled list operations today: array literals `[a, b, c]`, `xs[i]` reads
and assignments (including `+=`-style compounds), `len(xs)`, `xs.push(v)` /
`xs.pop()`, `xs.remove(i)` (remove by index, returns the element),
`xs.insert(i, v)`, and `xs.sort()` (in-place ascending for
`List<int/float/byte/char/bool/string>` — element comparison happens on the
boxed values), `for (x in xs)` iteration, and `print`/`println` on a list
(prints `List(len=N)`). Scalar elements are boxed at the runtime boundary;
strings/lists and other managed values pass through as pointers. Lists of
`char` and string byte-indexing/iteration are lowered: `List<char>` goes
through the scalar-list path (see the `char` bullet in `06-compiler.md`), and
`s[i]` / `for (c in s)` are byte-addressed reads typed `byte` per the locked
string model (`04-types.md`). `print(c)` on a `char` writes the scalar as
UTF-8 and `print(b)` on a `byte` writes the raw byte, so non-ASCII text
round-trips through the copy loop `for (c in s) { out += "{c}" }`.

**String buffer building is compiled.** `s[i] = v` writes a byte of a string
(copy-on-write: the value is replaced, other aliases keep the old bytes) where
`v` is a `byte`-typed value, an `int` literal in `0..=255`, or an ASCII `char`
literal — plus `+=`/`-=`-style compounds over the current byte (`s[i] += 1`
lower-cases/shifts). Targets are variables and field slots: locals,
`this.x`, bare instance fields (`text[i] = ...` inside a method), bare static
fields (`sbuf[i] = ...`), and `obj.field[i] = ...`. Together with `s[i]`
reads, a compiler can build a string byte-by-byte without a `List<byte>`
staging buffer. Not lowered yet: writes through `&T` parameters (still
rejected), and nested targets like `m[k][j] = ...` (strings are
copy-on-write); `Type.staticField[i] = ...` stores through the type name are
also out.

Compiled map operations today: `{ "a": 1 }` literals, `m[k]` reads and
assignments (including `+=`-style compounds), `m.has(k)`, `m.get(k) -> V?`
and `m.remove(k) -> V?` (both return the value as an option — `none` when the
key is absent — so `?` / `?.` / `??` / `if (let some(v) = ...)` compose
directly), and `m.keys()` / `m.values()` (fresh `List`s). Keys are
`string`-typed only in v1.

## I/O model

File/streams expose `read(n)? -> bytes`, `write(bytes)`, `flush()`,
`close()`. Bytes are `List<byte>` in v1 with a `string` conversion builtin
for text. Stdout/stderr are `Stream` values; `print` routes to stdout.

The whole-file layer of `std.fs` is already shipped as intrinsics on the
string (opaque-byte) ABI, the first runtime piece the self-hosted compiler
needs to load source:

- `read_file(path) -> string?` — reads a whole file byte-exact as `string`,
  or `none` when the file cannot be read.
- `write_file(path, text) -> bool` — overwrites `path` with `text`'s bytes.
- `file_exists(path) -> bool` — metadata probe (works for directories too).

Round-trips are byte-exact (no UTF-8 re-validation). Missing so far are the
buffered `Stream` model, byte-list (`List<byte>`) reads, `delete`, and
directory listing — those wait on a `List<byte>`↔`string` bridge.

## Networking & HTTP

`std.net` wraps OS sockets with TCP/UDP + TLS (rustls via runtime crate, or
no external C libs). `std.http` is a typed client/server on top: requests,
responses, streaming bodies, URL params, multipart.

## Crypto & math

Pure-Rust implementations, no OpenSSL dependency: SHA-2/3, HMAC, AES-128/256
(modes GCM/CTR/CBC), ChaCha20-Poly1305, curve arithmetic for signatures
later. Math: transcendental + vector/repr helpers.

## Testing

`test fn name()` or `test("desc", ...)`/`it` suites in any module; `describe`
groups nest, and `beforeAll`/`beforeEach`/`afterEach`/`afterAll` hooks run
per group. `expect(value).matcher(...)` (e.g. `.toBe`, `.toEqual`,
`.toContain`, `.toBeGreaterThan`) gives readable failure output. Assertions:
`assert(cond, msg?)`; `std.test` provides asserts and timing for longer-term
plans. `pickle test` discovers `*_test.pkl`/`*.test.pkl` files and runs them,
reporting per-module, per-group results.

## Release plan

`std.io`, `std.collections`, `std.text`, `std.math`, `std.time`, `std.json`
ship with 1.0; networking, crypto, HTTP, processes, threads in the 1.x line
as they stabilize. Each module is exercised by the compiler's own test suite
before it is declared stable.