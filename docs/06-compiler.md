# PickleScript — Compiler Architecture

Implemented in Rust, organized as a workspace.

## Pipeline

```
Source files
   |
   v
Lexer -------------------- tokens ----------> Parser
   |                                             |
   |                                             v
   |                                           AST (untyped)
   |                                             |
   |                                             v
   |                                    Resolver (names, scopes,
   |                                     modules, visibility)
   |                                             |
   |                                             v
   |                                    Type Checker (bidirectional
   |                                     inference, interfaces,
   |                                     nullability)
   |                                             |
   |                                             v
   |                                    Semantic IR (AstLowered:
   |                                     typed, monomorphic-ready,
   |                                     inlined-able)
   |                                             |
   |                                             v
   |                                   Mono (generics -> concrete
   |                                    instantiations)
   |                                             |
   |                                             v
   |                                    PickleIR (SSA-ish, explicit
   |                                     GC safepoints, shadow-stack
   |                                     slots, call lowering)
   |                                             |
   |                                             v
   |                                    Cranelift backend
   |                                      (module, functions, data)
   |                                             |
   |                                             v
   |                                    cranelift-object -> COFF/ELF
   |                                             |
   |                                             v
   |                                    Linker driver (cc / lld)
   |                                             |
   |                                             v
   |                                    Native executable  (+ lib runtime)
   v
```

## Crates

| Crate              | Role                                                               |
|--------------------|--------------------------------------------------------------------|
| `pickle-compiler`  | lexer, parser, AST, resolver, type checker, PickleIR, Cranelift backend, formatter |
| `pickle-runtime`   | GC, object model, builtins, stdio, entry point, exported C ABI; builds as a staticlib |
| `pickle-cli`       | the `pickle` tool: project loading, subcommands, build orchestration, REPL driver |

`pickle-cli` depends on `pickle-compiler`; the object file produced by the
compiler is linked against `pickle-runtime`.

## Front end

- **Diag**: source files, byte spans, severities; errors annotate code with
  a caret and a hint. Diagnostics stream to a colored reporter (TTY) or
  plain text (CI).
- **Lexer**: maximal-munch tokenizer, string/triple-string/raw/interpolated
  fragments, numberization, keyword table. Newlines are significant tokens
  (statement boundaries) plus a separate "continues" rule computed in the
  parser, so multi-line expressions still work.
- **Parser**: recursive descent, precedence climbing for binary operators.
  Produces an untyped AST with spans. Fails fast with recovery at the next
  declaration boundary so one session reports many errors.
- **Resolver**: module graph → import graph; scope trees; binding of every
  identifier; visibility checks (private/protected); duplicate detection;
  method override validation; generic parameter scopes.
- **Type checker**: bottom-up inference with expected-type propagation,
  signature unification across calls, interface conformance checks, option
  flattening, `is`/`as` validation, operator method lookup, monomorphization
  requests.

## Middle end

- **AstLowered**: typed AST with resolved symbols, implicit conversions
  materialized, desugaring of `??`, `?.`, ranges, lambdas -> closures,
  `+` on strings -> concat calls, pattern matching -> branch lowering.
- **Mono**: template instantiations for generic classes/functions and
  interfaces-as-generic-bounds; dedupes instantiations; sets concrete
  layouts.
- **PickleIR**: high-level typed IR with:
  - structural terms (blocks, calls, loads/stores, allocs with class ids)
  - SSA values
  - explicit GC safepoints and shadow-stack slot descriptors
  - invariants: every managed value either lives in a shadow slot or is
    spilled before any call/alloc.

## Back end

Cranelift (streaming, fast, register-allocated, no C dependency) > native
machine code. `cranelift-module` + `cranelift-object` emit COFF/ELF. A
future `llvm` feature switch will target LLVM for profile-guided and
aggressive optimization, sharing PickleIR.

The runtime is linked as a static library; the generated object exports
`pickle_main`, the runtime provides `main`, and the linker driver
(`cc`, or MSVC link / rust-lld fallback) produces the executable. No
semicolons, no headers, no Makefiles — `pickle build` does all of the above.

## Compiler phases as commands

`pickle` subcommands map to pipeline stages so failing stages are debuggable:

- `pickle build`   — full pipeline to executable.
- `pickle run`     — build (or JIT in dev) then execute.
- `pickle fmt`     — parse -> print canonical source (formatter).
- `pickle check`   — parse + resolve + typecheck only (fast CI gate).
- `pickle ast`     — dump AST.
- `pickle ir`      — dump typed IR.
- `pickle test`    — build tests + run.
- `pickle repl`    — incremental front end + JIT.

## Error philosophy

Errors are: precise (span-accurate), kinded (syntax/type/name/…), and
actionable (hint). The compiler never emits a wall of cascading errors for
one root cause; after an error in a declaration it resynchronizes at the
next declaration.