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
   |                                    rustc (native system linker)
   |                                             |
   |                                             v
   |                                    Native executable  (+ lib runtime)
   v
```

## Crates

| Crate              | Role                                                               |
|--------------------|--------------------------------------------------------------------|
| `pickle-compiler`  | lexer, parser, AST, resolver, type checker, PickleIR, Cranelift backend, formatter |
| `pickle-runtime`   | GC, object model, builtins, stdio, entry point, exported C ABI; linked as an rlib by `pickle build` |
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
machine code. `cranelift-module` + `cranelift-object` emit a single
relocatable COFF/ELF object for a whole module. A future `llvm` feature
switch will target LLVM for profile-guided and aggressive optimization,
sharing PickleIR.

`pickle build` emits the object, compiles `pickle-runtime` as an rlib with
`cargo`, and links them with `rustc` acting as the linker driver (so the
install's native system linker handles layout and CRT bootstrap). The object
exports `pickle_main` and imports the runtime's `pickle_*` helpers; a small
generated shim provides the process `main`, boots the runtime
(`pickle_runtime_init`), calls `pickle_main`, flushes output, and shuts the
collector down.

Two details keep the AOT path self-contained:

- float `%` lowers to a `pickle_fmod` defined *inside* the generated object
  as `a - floor(a / b) * b` CLIF, so the binary has no dependency on a
  `libm` `fmod` symbol;
- symbol references are carried between lowering phases as Cranelift
  test-case names, which the object backend rejects; emission remaps them to
  user names (namespace 0 = functions, 1 = data) before writing the object.
  Symbols the object *defines itself* (like the `pickle_fmod` binding above)
  must be registered in the same symbol maps, or their references keep a
  test-case name and emission panics.

Every other runtime helper is a plain imported `pickle_*` symbol resolved at
link time against the runtime rlib (the JIT resolves the same names through
its symbol table). That includes the list slice ABI added later: `pickle_box_*` /
`pickle_unbox_*` for scalar list elements and `pickle_list_new` / `len` /
`get` / `set` / `push` / `pop` for `List<T>`, which flow through this shared
extern path with no extra object glue. The map slice ABI follows the same
path: `pickle_map_new` / `set` / `get_boxed` / `has` / `len` / `keys` /
`values`.

One design note on `Map<K, V>`: to keep object reads cheap and GC-safe, an
index read passes an explicit *default* value down to `pickle_map_get_boxed`
(the value type's zero — `0`/`0.0`/`false`, an interned empty string, or
null). The runtime returns it unchanged when the key is absent, so the
emitter never unboxes a null pointer. v1 accepts `string` keys only; the
type checker enforces this at literal, index, and method sites.

No semicolons, no headers, no Makefiles — `pickle build <file>` does all of
the above.

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