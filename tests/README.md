# Tests

Cross-crate test suites. Crates also ship their own unit and integration
tests (e.g. `compiler/tests/checker_smoke.rs`, `compiler/tests/parser_smoke.rs`).

```
tests/
    pickle/        # PickleScript suites, run via `pickle test tests/pickle`
    compiler/      # end-to-end front-end suites (source -> errors/IR)
    runtime/       # GC, object model, builtins under stress
    integration/   # full `pickle build` -> execute pipelines
```

Language suites:

```
pickle test tests/pickle
```

Directory targets only discover `*_test.pkl` and `*.test.pkl` files; a file
explicitly named on the command line runs regardless of its name:

```
pickle test tests/pickle/arithmetic_test.pkl
```

Filter by test name substring (`--filter`/`-f`) — useful while iterating:

```
pickle test tests/pickle --filter "commutes"
```

Filter by tag: put `#[tag("name")]` (repeatable) on a test item, then pass
`--tag` (repeatable; a test runs when it carries any requested tag):

```
pickle test tests/pickle --tag slow
pickle test tests/pickle --tag math --tag integration
```

`--tag` and `--filter` combine with AND semantics.

Run everything with:

```
cargo test --workspace
```