# PickleScript — Implementation Roadmap

## End goal
The project's north star is for **Pickle to be self-hosted**: one day the
compiler and its toolchain are written in Pickle and compile themselves. Every
phase below is sequenced so the language keeps growing toward that: the front
end and runtime evolve together, `pickle run`/`pickle build` stay working
throughout, and each new feature is landed end-to-end rather than half-built
(see "Guiding principles for sequencing").

## Milestone 0 — Foundations (done in first pass)
- [x] Workspace, crates, docs
- [x] Language design (this repo's `docs/`)

## Milestone 1 — Front end (vertical slice, no codegen)
- [x] Diagnostic/source infrastructure
- [ ] Lexer: full token set + newline/statement hygiene
- [ ] AST + recursive-descent parser with error recovery
- [ ] Resolver: modules, imports, scopes, visibility, override checks
- [ ] Type checker: inference, nullability, interfaces, `as`/`is`
- [ ] `pickle check` / `pickle ast` debug commands

## Milestone 2 — Runtime
- [ ] Object model + headers, class descriptor tables
- [ ] Shadow-stack support (register/unregister frames)
- [ ] Non-moving mark-and-sweep GC, size classes, weak refs, finalizers
- [ ] Strings (concat/interpolate), number-to-text formats
- [ ] Entry point (`main`), stdio bridge, panic path
- [ ] Test harness registry

## Milestone 3 — Back end (native, via Cranelift)
- [ ] PickleIR emit from typed AST
- [ ] Monomorphization of generics & closures
- [ ] Cranelift lowering: loads/stores, calls, allocs, safepoints, shadow
      stack spilling, control flow, operators
- [x] Object emission (COFF/ELF) + linker driver (cc/rust-lld/MSVC)
- [ ] `pickle build` and `pickle run` working end-to-end for:
      functions, control flow, classes, fields, methods, inheritance,
      interfaces, strings, lists, maps, pattern matching, closures
- [ ] Debug builds: bounds checks, poison, backtraces

## Milestone 4 — Tooling
- [ ] `pickle fmt` formatter (AST printer, canonical layout)
- [ ] `pickle test` (discovers `test fn`, runs, reports)
- [ ] `pickle repl` (incremental front end + JIT)
- [ ] `pickle package` (source/binary bundle)
- [ ] `pickle doc` (from `///` comments)
- [ ] `pickle.toml` full schema: deps, targets, options, features

## Milestone 5 — Standard library
- [ ] `std.io`, `std.collections`, `std.text`, `std.math`, `std.time`
- [ ] `std.json`
- [ ] `std.net` (TCP/UDP), `std.http`, `std.crypto`
- [ ] `std.process`, `std.thread` with tasks/channels/parallel
- [ ] `std.fs`, `std.gc` (arenas, weak, stats), `std.test`

## Milestone 6 — Systems programming surface
- [ ] `unsafe` core complete: pointers, refs, custom allocators, arenas
- [ ] C FFI (`extern "C"`, `@[link(...)]`)
- [ ] `extern` module imports from linkable libs

## Milestone 7 — Language hardening
- [ ] Async lowering to state machines; `task`/`channel`/`parallel` runtime
- [ ] Optimizations: inlining, devirtualization, const folding,
      escape analysis of temporaries, string interning
- [ ] Optional LLVM backend feature
- [ ] Incremental GC + generational plan
- [ ] Language reference + spec conformance test suite

## Guiding principles for sequencing
1. Always keep `pickle run` working; land features end-to-end, not
   half-compiled.
2. Front end strictly precedes back end work touching a feature.
3. Runtime and compiler evolve together; the ABI (`pickle_*`) is the
   contract, never broken without a versioned bump.
4. Every milestone ends with a runnable demo under `projects/`.
## Definition of shippable

A release is shippable when the compiler is self-hosting for its front end,
produces native executables with no manual linking steps, passes its own
conformance suite, and `build`, `test`, `fmt`, `doc`, `package`, and `repl`
all work.