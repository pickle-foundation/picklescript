# Advanced error reporting

Status: design notes, not implemented yet.

The compiler's diagnostics today are flat: a message, a location, a source
snippet. That is enough to pinpoint a single mistake in a small file, but for
a program past a few hundred lines it stops being helpful. The most common
follow-up after reading such a diagnostic is to trace the values involved by
hand — walk back to where a variable came from, figure out which step of a
signature change a call site tripped over, or search the tree for every place
a renamed field is referenced. The compiler has to do that tracing anyway
while checking; it just discards the results. This document describes an
error-reporting design that keeps that information and presents it, plus a
few tools built on top of it.

Nothing here changes the language. It adds a rendering layer, a couple of
subcommands, and — for the JSON mode — a serializer over diagnostics that
already exist. `.?.` and `??` appear in examples below as possible future
syntax; neither exists today.

## Diagnostic structure

Every diagnostic carries the pieces below. The first three are what the
diagnostic pipeline already produces; the rest are new.

| field             | origin today         | what it holds |
|-------------------|----------------------|---------------|
| code              | none (message text)  | stable symbol such as `E0308` |
| summary           | diagnostic message   | one line, what was rejected |
| location          | span + source map    | file, line, column, snippet |
| reasoning tree    | none                 | ordered list of typed facts ending in the violated rule |
| related locations | none                 | spans contributing to the error (declarations, prior uses) |
| suggestions       | none                 | candidate fixes with placeholders, optionally machine-applicable |

The `code` field is new and matters for all downstream work. Errors need a
stable symbol so `pickle explain` can look them up, so groups can be keyed on
them, and so CI can match on them without parsing prose. The codes form a
catalogue (`E0xxx` per bucket) rendered as:

```
error[E0420]: cannot access field "length"
```

The renderer's message prefix is a mechanical change; the summary stays on
the same line as the code.

## The reasoning tree

The checker works by walking an expression and resolving each sub-expression
to a typing: it looks up what `user` is, what its field `name` is, what type
that field has, and whether the operation on that type is legal. Each of
those steps is currently evaluated and thrown away. The reasoning tree makes
the steps visible in the same order the checker performed them:

```
error[E0420]: cannot access field "length"

src/users/UserService.pkl:42

let size = user.name.length

user
 └── type: User
      └── field: name
           └── type: string?
                └── value can be none
                     └── field access requires non-null value

suggestions:
1. guard the access:       if (user.name != none) { user.name.length }
2. optional chaining:      user.name?.length
3. or provide a fallback:  user.name ?? ""
```

The tree is a chain of `(span, step)` nodes. Each node records the
sub-expression it reasoned about and the conclusion reached. The conclusion
of one node is the premise of the next, so the last node is the rule that was
violated.

Producing the tree requires two things that already exist in the checker.
First, `check.rs` stores a `HashMap<Span, Ty>` of every sub-expression's
resolved type; that is the source of the per-node conclusions. Second, each
sub-expression's span maps back through `SourceMap` to the snippet to print.
The work is to place the nodes into a per-diagnostic list as the check
unwinds, rather than discarding them. Because the tree is only materialized
when a diagnostic fires, the memory cost outside error paths is negligible —
the chain exists only inside the failing check's local state and is dropped
on success.

The tree is optional. When the reason is obvious, or when nothing was traced
(a parse error, for instance), the renderer omits it.

## Value origins in type mismatches

A mismatch currently reads:

```
expected int, found string
```

accurate, but it does not say where the `string` came from, so the reader
starts hunting. With origins, the same error becomes:

```
error[E0308]: type mismatch

expected:   int
found:      string, from "name" initialized at line 12 (literal "Pickle")

used here:  calculateAge(name)
function:   calculateAge(value: int)
```

The origin is the declaration that produced the offending value. `check.rs`
resolves every identifier to a binding in scope; the binding already carries
its declaring span and, for a literal initializer, the literal expression.
Attaching the origin to the diagnostic is therefore mostly a matter of
carrying the binding's span next to the typed value through the mismatch
path.

Origins are tracked selectively: only for names and literals, and only long
enough to answer a failing unification. The reasoning tree reuses that
mechanism — the origin of the `string` above is exactly the first node of the
chain that would have been printed for the mismatch.

## `pickle explain`

A reasoning tree is too wide for tooltips, and it should not clutter every
build log. The diagnostics stay compact, and the expanded walkthrough is
moved behind a dedicated command that reads the error catalogue:

```
pickle explain E0420
```

The catalogue is a value in the compiler holding, per code: the rule text
("field access requires a non-null value"), the reasoning-chain template, and
one or two worked examples. `explain` prints those. It is analogous to
`rustc --explain`, and it doubles as the documentation for the error codes,
so edge cases do not need to live in the manual.

## Memory lifetime errors

Use-after-free becomes expressible once `#[manualAlloc]` and the allocation
primitives from `docs/05-memory.md` are in play. Reporting that class of bug
means presenting a timeline, not a fact:

```
error[E0501]: object lifetime conflict

user:    created at line 20
         released at line 55 (manualAlloc)
         used at     line 60  ← invalid

20 | let user = User()
...
55 | release(user)
...
60 | println(user.name)
```

Mechanism: for each function, record for manual objects the three interesting
spans — creation, release, and every use that follows a release on the same
control-flow path. That is a lightweight liveness pass over the function's
blocks: for each `manualAlloc`-owned binding, walk the block list in order
and flag the first use that comes after a release. No borrow-checker-level
machinery is required; the pass can be a dedicated stage in `check.rs` or a
lint that runs after checking. The exact trace semantics depend on the
allocation and release forms the manual-ownership design settles on, so this
diagnostic is specced here but implemented together with that design.

## Grouping errors

When a refactor goes wrong, the compiler can produce dozens of diagnostics
that are really one problem:

- missing imports (23)
- changed function signature (12)
- invalid types (10)

Grouping reduces a flat list of 50 to 4 headlines. The grouping key is the
root cause the checker kept tripping over — the code plus the item it was
resolving (the missing import name, the function whose signature changed).
Each diagnostic carries that key from the moment it is created; grouping is
then a stable sort by (key, location) in the renderer. No post-processing
over message text is involved.

## Suggestions

The checker already knows several candidate fixes at the point of an error;
it just does not print them. Two examples:

- a non-void function with a path that returns nothing → "add `return 0`" or
  "change `-> int` to `-> void`";
- a concrete mismatch where one side is a subtype or there is an obvious
  conversion → "did you mean to call `parseInt`?" or "implement the
  interface".

Suggestions are typed edits: a replacement span plus replacement text (or
"insert at span"). They are printed under the diagnostic, never applied. If
an applying mode ever exists it is a distinct `--fix` step that must be gated
and reversible.

## JSON mode

For IDEs and CI, `pickle check --json` emits one JSON object per diagnostic:

```
{ "code": "E0308", "file": "User.pkl", "line": 42,
  "expected": "User", "found": "Guest",
  "reason": ["Guest does not implement User"],
  "suggestions": ["Implement the interface", "Convert the type"] }
```

The top-level fields are stable. `reason` and `suggestions` start out
omitted when absent, so consuming tools written against the minimal schema
keep working as the richer diagnostics land. The work is a serializer over
the diagnostic struct in `compiler/src/diag.rs` plus a `--json` flag on the
existing `check` subcommand in `cli/src/main.rs`.

## Impact analysis

`pickle impact` takes a name (a class, a field, a function) and reports every
usage of it across the source tree, whether a rename or signature change
would break each usage, and which files and tests are involved. In effect it
exposes the two things `check.rs` and `resolve.rs` already compute — name
resolution and the `HashMap<Span, Ty>` — as a query.

This is the most deferred item here. It needs cross-file resolution to be
useful: the current front end processes a single module, and impact analysis
over one file is not worth a subcommand. It is listed so its dependency (a
resolution index) is not built twice.

## Implementation order

Each step builds on the previous one:

1. Thread a reasoning chain and value origins through `check.rs`, and append
   them to the diagnostic when it fires.
2. Extend `diag.rs` with the `code` field and the reasoning/related/suggest
   renderers.
3. Add the error catalogue and `pickle explain` to the CLI.
4. Group diagnostics by their (code, root item) key in the renderer.
5. Add the `--json` serializer.
6. Implement E0501 together with the manual-allocation design.
7. `pickle impact` after cross-file resolution exists.

Caveats: the syntax suggested in examples (`?.`, `??`) does not exist and
must be designed separately if it is wanted; lifetime reporting waits on the
allocation model; impact analysis waits on multi-file resolution.