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

Enums follow the same extern path. An enum value is a managed object
(`PEnum`) whose class id is `PICKLE_CLASS_ENUM` (6; user classes start at 7).
Its payload slot 0 holds the variant tag as a *raw* `i64` (unmanaged, skipped
by the tracer); payload slots 1.. hold the variant's fields, boxed with the
same element rules as `List<T>` elements. Field count is derived at runtime
from the object size (`(size - header)/8 - 1`), so all variants of an enum
get an object sized for their widest variant. The ABI is
`pickle_enum_new(tag, count)` / `pickle_enum_set_field(obj, i, value)` /
`pickle_enum_tag(obj)` / `pickle_enum_field(obj, i)`.

Lowering rules:

- `Enum.Variant(args...)` compiles to `pickle_enum_new` plus one
  `pickle_enum_set_field` per argument, boxing scalar payloads.
- A bare zero-field variant (`Color.Red`) compiles to
  `pickle_enum_new(tag, 0)`.
- `match (s) { case V(p..) -> body ... }` keeps the scrutinee in a managed
  shadow slot (so the GC retains it across arm allocations), reads the tag
  once, and lowers to a `BranchIf` chain comparing `tag == variant_index`.
  Payload bindings unbox each field into fresh slots; `case _` / `case name`
  catch-all arms are a plain branch.
- A chain that reaches its end without a catch-all arm calls
  `pickle_panic_no_match`, which faults with "pickle: match is not
  exhaustive" — the checker does not require exhaustiveness, so misses fail
  loudly at runtime instead of reading garbage.
- Patterns beyond `Variant`/`Wildcard`/`Binding` payload names, and guards,
  bail with "not lowered yet".

Classes use the same object-model extern path. A class instance is a plain
managed object (`PickleObject`) whose runtime `ClassDescriptor` carries the
name (for printing) and a field-validity mask. Descriptors are allocated *at
runtime* rather than baked into the generated binary: `main`'s entry block
opens with one `pickle_class_register(name_ptr, name_len, slot_count, mask)`
void call per class, in registration order, so descriptor id == call site id.
Call sites inside constructors hardcode that id (`7 + emitted index`), so no
id is threaded through the lowering; `pickle_class_register` returns the id
but emitters discard it. `StrAddr` lowerings pull the name bytes from the data
section (`pkl_strdata_N`) — AOT marks those externs as data, JIT lowers them
to symbol addresses; the pointer is never treated as a GC object.

Lowering rules:

- No explicit constructor, no properties, no `static var` fields, no field
  initializers, no `init`/`deinit`, no `Const` members, no generics/extends/
  implements -> the class is *registered*. Anything else bails with
  "… in `{name}` are not lowered yet" so nothing miscompiles silently.
- `TypeName(args...)` compiles to `pkl_<TypeName>_new(args...)`: parameters
  are the instance fields in declaration order, slot 0 of the object is
  `pickle_class_new(id, field_count)`, each field value is boxed per the
  scalar list rules and stored with `pickle_obj_slot_set`. Only non-static
  fields occupy slots; a class with 64+ fields empties the mask.
- Instance methods compile to `pkl_<TypeName>_<m>`, with the receiver passed
  first as a managed pointer (IR param slot 0, declared as `this`); static
  methods compile to `pkl_<TypeName>_sm_<m>` with no receiver. Async/
  override/body-less/generic methods and methods with default/rest params are
  skipped like their top-level counterparts.
- `this` loads the receiver slot. An undefined identifier inside an instance
  method falls back to `this.<name>`; `obj.field` read/write and compound
  assigns (`+=`) lower through `pickle_obj_slot_get`/`pickle_obj_slot_set`
  with scalar unbox/box on the boundary, and a member chain like
  `p.home.x` reads the nested object first via the normal member path.
- `Type.staticMethod(...)` and `instance.method(...)` dispatch through
  `method_ids` (parameterized by class id and method name); `this` is the
  first call argument for instance methods.
- `println`/`print` of a class/struct value lowers to `pickle_print_obj`,
  which prints the class name plus `Class` fields when available.

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