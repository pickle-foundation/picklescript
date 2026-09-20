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
`xs.pop()`, `for (x in xs)` iteration, and `print`/`println` on a list
(prints `List(len=N)`). Scalar elements are boxed at the runtime boundary;
strings/lists and other managed values pass through as pointers. Lists of
`char` and string byte-indexing/iteration are lowered: `List<char>` goes
through the scalar-list path (see the `char` bullet in `06-compiler.md`), and
`s[i]` / `for (c in s)` are byte-addressed reads typed `byte` per the locked
string model (`04-types.md`). `print(c)` on a `char` writes the scalar as
UTF-8 and `print(b)` on a `byte` writes the raw byte, so non-ASCII text
round-trips through the copy loop `for (c in s) { out += "{c}" }`.

## I/O model

File/streams expose `read(n)? -> bytes`, `write(bytes)`, `flush()`,
`close()`. Bytes are `List<byte>` in v1 with a `string` conversion builtin
for text. Stdout/stderr are `Stream` values; `print` routes to stdout.

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