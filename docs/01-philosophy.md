# PickleScript — Language Philosophy

PickleScript is a standalone, native-compiled, general-purpose programming
language. Its design goal is one sentence:

> **Familiar syntax. Modern design. Powerful capabilities.**

It is intentionally a *language*, not a dialect of any other language. It
borrows good ideas where they make the language better, but every feature is
re-derived to fit PickleScript's own grammar and mental model.

## What PickleScript wants to be

PickleScript serves the same broad set of users that would reach for C#, Go,
or Java today — backend service authors, desktop app writers, CLI tool
creators, infrastructure engineers — and it provides a real path into systems
programming for those who want it.

The language is expected to build real software: servers, editors, build
tools, libraries, and other application workloads.

## Core values (ranked)

1. **Readability.** Source is written once and read many times. The grammar
   pushes the happy path; unusual behavior is explicit, never implicit.

2. **Simplicity.** One obvious way to say something. Few keywords. No
   ceremony. A PickleScript file reads top-to-bottom and does what it looks
   like it does.

3. **Consistency.** The same rule applies everywhere. Function calls, method
   calls, constructors, and generics all use the same `(...)` and `<...>`
   shapes. Blocks are always `{ }`. Conditions always use parentheses.

4. **Minimal ceremony.** No `public static void main(String[] args)`.
   Declarations infer types. Everything you don't write is a sensible
   default, and you can always opt into explicitness.

5. **Production capability.** The compiler is real: lexer, parser, semantic
   analysis, typed IR, optimizing native codegen, integrated build tool. The
   standard library covers real workloads: files, network, HTTP, JSON,
   crypto, threading, time.

## Design rules of thumb

- **No semicolons.** Statements are separated by lines and structure, not
  punctuation.
- **One keyword, one job.** `let` is immutable, `var` is mutable, `const` is
  a compile-time constant. `fn` declares a function. `class`, `struct`,
  `enum`, `interface` each mean exactly one thing.
- **Everything is either a value or a reference, and that never changes at
  runtime.** `struct` = value, `class` = reference, primitives are values,
  `string`/`List<T>`/`Map<K,V>` are managed references.
- **Safe by default, powerful on request.** Normal code is garbage
  collected. `unsafe { }` unlocks pointers, raw access, and custom
  allocators — one keywords worth of ceremony for systems work.
- **No header files, no build scripts.** The module system is the build
  system. `pickle build` finds files, resolves imports, compiles, links, and
  produces an executable.
- **Tooling is a feature, not an afterthought.** `build`, `run`, `fmt`,
  `test`, `package`, `doc`, and `repl` are part of the language contract.

## What PickleScript deliberately avoids

- C/C++ build and linker pain (headers, `Makefile`, CMake plumbing).
- Java-style boilerplate (everything inside a class, checked exceptions).
- Go's minimalism taken to the point of hand-writing trivially generic code.
- TypeScript's loose structural type system.
- Rust's borrower ceremony as the *default* for application code (available
  in `unsafe`, where it matters).

## The social contract

PickleScript trusts the developer. It assumes you want correctness, gives you
a strong type system and clear errors, and stays out of your way the rest of
the time.