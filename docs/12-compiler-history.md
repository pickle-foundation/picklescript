# Compiler history

Status: implemented (MVP). `pickle history`, `pickle builds`, and `pickle
explain <CODE> --history` are in; the cache lives at
`<git-root>/.pickle/history/snapshots.db` and is regenerable (gitignored).

Compiler history makes the evolution of a codebase queryable without a
server: versions are the git history. For every commit that touches a
`*.pkl` file we compile the file exactly as it existed at that commit, store
a snapshot, and diff consecutive snapshots. Three questions are answered:

- **What changed to a public type over time?** `pickle history` prints the
  versions of each file, newest to oldest, with the surface (public
  functions, classes, fields, methods, constructors, properties, enum
  variants, interface members, and constants) and the dependency lines
  (`import`/`use`) between consecutive versions.
- **When did this code stop (or start) compiling?** `pickle builds` prints
  the most recent build rows — commit, file, `ok`/`ERR`, and the stable
  error codes of the failures.
- **When was this error first produced?** `pickle explain E0351 --history`
  prints the error catalogue entry plus, oldest first, every committed
  version that produced that code.

Nothing here changes the language. It is a renderer and a cache over the
compiler that already exists.

## The snapshot

A snapshot is one JSON object per (commit, file):

```
{"commit":"0f32f41…","short":"0f32f41","date":"2026-09-20 17:59:43 +0300",
 "message":"v2: nullable name, admin field, a typo","file":"app.pkl",
 "hash":"…","ok":false,"codes":["E0308","E0351"],
 "deps":["import a.b as c"],
 "items":[{"kind":"field","name":"User.name","vis":"def","sig":"name: string?"},…]}
```

| field     | meaning |
|-----------|---------|
| `commit`  | full git commit id |
| `short`   | first 7 characters |
| `date`    | commit date (git `%ci`) |
| `message` | commit subject line |
| `file`    | the `.pkl` path in that commit |
| `hash`    | FNV-1a 64 of the file content (hex); keys the cache entry |
| `ok`      | the frontend (lex/parse/resolve/check) plus codegen reported no errors |
| `codes`   | distinct stable error codes produced, sorted |
| `deps`    | the module's `import`/`use` lines, sorted |
| `items`   | the public surface (see below) |

The compiler has no dependencies, so the cache is plain JSONL written
through the compiler's own JSON encoder (`diag::json_string`), and read back
through a small bundled parser. Entries that fail to decode are skipped, and
an item whose `kind` is not one of the known kinds is skipped rather than
guessed. A snapshot whose content hash still matches the file is reused;
only stale or missing entries are recompiled.

One cache file (`.pickle/history/snapshots.db`) holds all snapshots rather
than a file per dimension (`symbols.db`/`dependencies.db`/`builds.db`). A
snapshot carries the surface, the dependencies, and the build status
together, and any content change invalidates the whole snapshot, so three
files would just duplicate every line without adding invalidation
granularity.

## The public surface

The surface is the stable, callable and queryable API of a module —
everything a consumer or a test could rely on. Bodies are intentionally
irrelevant. Members are flattened under their owner so each is diffed
independently:

- functions and `test` functions → `fn`
- classes, structs, enums, interfaces → `class`/`struct`/`enum`/`interface`
  plus `field`/`method`/`ctor`/`prop`/`variant`/`imethod`/`iprop` for their
  members
- module and class constants → `const`
- visibility → `pub`/`prot`/`priv`/`def`

Each item is keyed by `(kind, dotted name)` and rendered to a canonical
signature (parameters, return type, `static`/`const`/`override`/`async`
flags, `extends`/`implements`), so unrelated text changes never show up as a
type change. Surfaces are sorted by `(kind, name)`.

## Rendering

`pickle history [--file F] [--limit N]`:

```
history of app.pkl (2 version(s))
  87cb0cd  2026-09-20 17:59:43 +0300  v1: plain User
  0f32f41  2026-09-20 17:59:43 +0300  v2: nullable name, admin field, a typo
  changed-type  User.<init>  (name: string, age: int) -> (name: string?, age: int)  (made nullable)
  added-type    User.admin   admin: bool
  changed-type  User.name    name: string -> name: string?  (made nullable)
  added-type    broken       broken()
```

Each change row is one of `added-type`/`removed-type`/`changed-type`,
followed by the item name and `before -> after`. A wording label is printed
only when the difference is structural enough to name honestly — `string ->
string?` is "(made nullable)", `string? -> string` is "(no longer
optional)". A change that the plain-text comparison cannot claim is left
unlabeled rather than labelled with a guess.

`pickle builds [--limit N]`:

```
      commit  file                       ok    codes
  0f32f41  app.pkl                       ERR  E0308 E0351
  87cb0cd  app.pkl                       ok
```

`pickle explain E0351 --history` prints the catalogue entry and then the
history of the code:

```
error[E0351] -- undeclared name
…
error[E0351] -- produced by committed versions (oldest first):
  0f32f41  2026-09-20 17:59:43 +0300  v2: nullable name, admin field, a typo
first seen: 0f32f41 (app.pkl)
```

## Fit-later: migration assistance

The purpose of the history is to help both a human and a tool answer "what
breaks if I change `User.name`?" at any earlier version. That needs
cross-file resolution — this repository's fixture added it as a queued item
for the multi-module front end. Deferred:

- rename/signature-change advice over the whole tree (`pickle impact`);
- a `--diff` machine mode; the JSONL cache already contains everything needed,
  but a stable command-line schema was not specced yet;
- pruning the cache (kept forever; histories are small today).