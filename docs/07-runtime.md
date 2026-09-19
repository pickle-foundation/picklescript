# PickleScript — Runtime Architecture

## Shape

`pickle-runtime` is a static library (`staticlib`) written in Rust,
implementing:

- the garbage collector and object model,
- thread-local GC registration,
- builtin type implementations (string, list, map internals, char),
- I/O and process bridges,
- time, and
- the process entry point and startup sequence.

Compiled PickleScript code imports these behind the compiler's
`runtime_abi` table — the only ABI the compiler needs to know:
`pickle_*` `extern "C"` symbols plus the compiler's own descriptor tables.

## Startup

```
main(argc, argv)                     <- runtime
  -> pickle_runtime_init()           <- parse args? no: called first
  -> initialize GC heaps
  -> register class/descriptor table emitted by compiler
  -> enable shadow stack protocol
  -> call pickle_main()              <- emitted by compiler
  -> flush stdout
  -> pickle_runtime_shutdown()      <- optional
```

`pickle_main` is the compiled program's `fn main()`; implicit `print` at the
end of a program awaits outstanding async tasks before returning (i.e. the
runtime drains the task pool at exit).

## Tasks & scheduling

v1 thread model: cooperative threads mapped 1:1 onto OS threads; each OS
thread maintains its own shadow stack, GC ids, and heap-affiliated data.

- `task { ... }` spawns an OS thread (or a coroutine on the shared pool,
  configurable) executing a `fn (): T`.
- `async fn` lowers to a state machine that yields at each `await`.
- Channels are rendezvous buffers with a bounded queue: `channel(T, cap)`,
  `send` (blocking/`trySend`), `receive`/`tryReceive`/`select`.
- A program-wide work-stealing pool backs `parallel { }` blocks:
  `parallel for (i in jobs) { }`.

GC stops the world: the runtime signals all registered threads, each thread
parks at its next safepoint; v1 safepoints are allocation, call, and channel
operations (places compiled code already spills), plus an explicit
`gc.safepoint()` builtin for long loops.

## Standard runtime services

- **GC**: see memory model (non-moving mark-and-sweep, shadow stack, size
  classes, weak refs, finalizers).
- **Allocator interface**: default allocator (VirtualAlloc arenas on
  Windows, mmap elsewhere) with pluggable global allocator and arena API.
- **Strings**: rope? no — contiguous UTF-8 with cached length; concat &
  interpolation helpers exported for codegen.
- **Numbers-to-text**: fast int/float to string for interpolation.
- **Console**: `print`, `println`, `print(..)`, ANSI if TTY.
- **Error bridge**: `panic(msg)` -> unwind banner + nonzero exit; call-stack
  capture is a debug-mode feature (frame pointers).

## FFI (later milestones)

C ABI export/import announced in `extern` blocks; `libc`-adjacent functions
declared in the standard library. The ABI surface is already stable because
the runtime's internals are `extern "C"`.

## Testing hooks

`pickle test` links the runtime with a test harness: `test fn` bodies are
ruad via reflection-less discovery — the compiler emits a registry of test
functions into the object, the harness runs them, reports pass/fail counts,
and exits nonzero on failure.

## Portability

- Windows: `kernel32`-only runtime linkage; no CRT dependency;
  MinGW (`cc`) or MSVC (`link.exe`) acceptable for object linking.
- Linux/macOS: `mmap`, `pthread` as needed; same `pickle` semantics.
- The runtime has no I/O on the allocation path, no thread heap contention
  during STW collection.