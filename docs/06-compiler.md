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
- Patterns beyond `Variant`/`Wildcard`/`Binding` payload names bail with
  "not lowered yet". A guarded arm lowers into a `guard_in` block between the
  tag-check target and the body: payload bindings (or the binding-name for a
  guarded catch-all) are live there, the guard evaluates to a bool, and a
  `BranchIf` on the guard sends the value to the body or falls through to the
  next arm chain. Guard-false is not a match failure: the search continues (a
  guarded catch-all keeps the chain live, and a chain that exhausts every arm
  falls into the `pickle_panic_no_match` trap). Guards may use `&&`/`||`,
  which lower through the ordinary short-circuit `logic()` path.
- Numeric promotion at mixed float/int operands (`r > 0`, `f * i`) inserts an
  `itof` (i64 -> f64) conversion instruction on the int side before the binop,
  so the JIT and AOT always see same-width operands.

Classes use the same object-model extern path. A class instance is a plain
managed object (`PickleObject`) whose runtime `ClassDescriptor` carries the
name (for printing) and a field-validity mask. Descriptors are allocated *at
runtime* rather than baked into the generated binary: `main`'s entry block
opens with one `pickle_class_register(name_ptr, name_len, slot_count, mask,
finalizer)` void call per class, in registration order, so descriptor id == call
site id. Call sites inside constructors hardcode that id (`8 + emitted index`),
so no id is threaded through the lowering; `pickle_class_register` returns the
id but emitters discard it. `StrAddr` lowerings pull the name bytes from the
data section (`pkl_strdata_N`) — AOT marks those externs as data, JIT lowers
them to symbol addresses; the pointer is never treated as a GC object. The
`finalizer` argument is the address of `pkl_<TypeName>_deinit`, or `0`.

Lowering rules:

- A `deinit { ... }` lowers to `pkl_<TypeName>_deinit(this) -> unit`: slot 0 is
  `this`, the body runs like a constructor body (unit result, `this` live), and
  the function's address is passed to `pickle_class_register` through a
  `FuncAddr` const (`addrof fn#N`, typed `int` so the raw code pointer never
  enters the GC trace frame). A class without `deinit` passes `0`.
- Generics/extends/implements are not lowered: a class using them bails with
  "… in `{name}` are not lowered yet" so nothing miscompiles silently.
- `TypeName(args...)` compiles to `pkl_<TypeName>_new(args...)`: slot 0 of the
  object is `pickle_class_new(id, field_count)`, then each field value is boxed
  per the scalar list rules and stored with `pickle_obj_slot_set`. Only
  non-static fields occupy slots; a class with 64+ fields empties the mask.
- Constructor parameters are the explicit `constructor(...)`'s parameters when
  one is declared; otherwise they are the instance fields *without*
  initializers (declaration order). Fields with initializers (`var x: int = 0`)
  and `init { ... }` blocks run during construction: field initializers in
  declaration order, then the explicit constructor body (an implicit one has
  none), then any `init` blocks. `this` is live throughout, so initializers can
  read earlier fields and the body can assign any member. An initialized field
  therefore needs no constructor argument, but still occupies a slot.
- A named constructor (`constructor.name(params) { this(...) }`) lowers to a
  receiver-less factory `pkl_<TypeName>_nc_<name>(params) -> Ptr`. Its body must
  be exactly one `this(args)` delegation to the primary constructor; the factory
  evaluates the delegation arguments (option-wrapped like any call), calls
  `pkl_<TypeName>_new`, and returns its pointer. `TypeName.name(args)` dispatches
  to that factory; a named constructor creates no cell and no per-class state.
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
- Instance properties compile to `pkl_<TypeName>_<p>_get` and
  `pkl_<TypeName>_<p>_set` functions. A getter takes `this` in slot 0 and
  returns the property type; a setter takes `this` and a `value` parameter and
  returns unit. `obj.prop` reads dispatch the getter and `obj.prop = v` writes
  dispatch the setter; a bare property name inside an instance method falls
  back to `this.<prop>` through the getter. Getters/setters use `=> expr` or
  `{ ... }` bodies and may reference fields and methods through `this`.
  Compound assignment to a property bails with "compound assignment to a
  property is not lowered yet". Static properties compile to receiver-less
  `pkl_<TypeName>_sm_<p>_get`/`_set` functions dispatched through a separate
  map keyed by class id and property name (so instance dispatch can never read
  a static accessor), and the checker requires them to be reached through the
  type name: `Type.prop` reads call the getter, `Type.prop = v` writes call the
  setter, a static getter body may reference other statics via `Type.other`,
  and instance receivers cannot reach static properties (or vice versa).
  Statics with a getter but no setter are read-only.
- Static fields (`static var`) compile to runtime cells keyed by
  `(class id, static slot)`: `pickle_static_get`/`pickle_static_set` hold one
  managed pointer per field, and each cell is registered as a GC root so its
  pointee survives collection. Every static field is initialized once at
  program start by the synthesized `pkl_static_init` function — the declared
  value, or the field type's default zero/null — which `main`'s entry block
  calls right after class registration. `Type.field` reads/writes and compound
  `op=` go through the cell (scalars box/unbox on the boundary), and a bare
  static-field name inside the class's own static method resolves to the same
  cell. The checker enforces direction: a static field is reached through the
  type name, an instance field only through an instance.
- `const` members are compile-time values: the emitter inlines their
  initializer at every read (`Type.NAME`, or the bare name inside the class
  body), evaluating it in the declaring class's scope so one constant may
  reference another. No cell or static-init entry is created; the emitted code
  is the same as writing the literal at the use site. Assigning a constant and
  reaching one through an instance are rejected by the checker, and a cyclic
  constant is a codegen error ("cyclic `const` initialization").
- `char` values are boxed/unboxed exactly like the other scalars: the runtime
  provides a dedicated `char` box class (id 6), so `char` fields, `List<char>`
  elements, and `char` map *values* store `pickle_box_char`/`pickle_unbox_char`
  pointers and load them back through the scalar list path. `char` is an i32
  scalar at the IR boundary; only the low byte is meaningful for ASCII, and the
  printed form (raw or boxed) is that single byte. An absent map key of type
  `char` reads back the zero value `'\0'`.
- `s[i]` on a `string` lowers to `pickle_str_get` (a bounds-checked byte read
  widening to the `char` `i32` ABI; out-of-range panics). `for (c in s)`
  iterates a string through the same path (`pickle_str_len` bound +
  `pickle_str_get` per trip), char-binding the raw scalar with no box. The
  checker rejects too-few-argument constructor calls, so a miscounted
  synthesized-ctor call reports "expected N argument(s), found M" instead of
  reaching a codegen crash.
- `println`/`print` of a class/struct value lowers to `pickle_print_obj`,
  which prints the class name plus `Class` fields when available.

Options lower as a managed pointer with no dedicated allocation:

- `Option<T>` (`T?`) occupies a `ptr` slot/return; `none` is a null constant.
  `some(v)` is `v` itself when `v` is already managed (class/struct/enum/
  string/`List`/`Map`/nested option) and a `pickle_box_<scalar>` pointer when
  `v` is a primitive. A boxed value is a class, so it is never boxed twice.
- The checker does not rewrite the lifted expression's type, so the emitter
  lifts by *value* type: a value whose natural type is a scalar flowing into a
  `ptr` boundary — annotated/typed `let`/`const`, `return`, a function tail
  expression, a call argument at a `T?` parameter, a ctor/field store, or a
  property setter — is boxed once; managed values pass through. `none`/managed
  values are never boxed.
- `a ?? b` evaluates `a`, null-tests it, and yields `a`'s unboxed value when
  present or the already-evaluated `b` otherwise. Its type is the inner type,
  so it assigns to `T`, not `T?`.
- Postfix `?` unwraps: it null-tests and calls `pickle_panic_none_unwrap`
  ("pickle: unwrapped none value") on `none`, otherwise unboxes/returns the
  value. (`!` is logical not.)
- `obj?.member` evaluates the receiver once, null-tests it, reads the field or
  dispatches the property getter on the present branch (lifting a scalar
  member back into an option), and yields `none` on the absent branch. The
  member's declared type — a field slot's type or the property table's `ty` —
  drives the re-lift.

Casts lower for the statically-known subset:

- `x as float` / `x as int` emit `itof` / `ftoi` (the latter saturating, so
  NaN/out-of-range clamps instead of trapping). `char` is not numeric, so
  `char`/`int` casts are rejected by the checker.
- Casting to an option — `x as? T`, or `as T?` — always yields `T?` and never
  panics: a present value is converted (numeric) and boxed, `none` stays
  `none`. The checker types `as?` as `T?`.
- `x as T` where `x: T?` asserts presence: it unwraps (panicking on `none`
  through `pickle_panic_none_unwrap`) then converts.
- `x is T` is a presence test when `x: T?` and `T` is `x`'s inner type;
  otherwise it folds statically (`T is T` -> true, `int is float` -> false).
- Class/interface `is`/`as` across an inheritance edge bails ("not lowered
  yet"): `extends`/`implements` are not lowered, so there is no runtime
  hierarchy to test against yet. The checker accepts such casts; only codegen
  rejects them.

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