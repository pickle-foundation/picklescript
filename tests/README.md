# Tests

Cross-crate test suites. Crates also ship their own unit and integration
tests (e.g. `compiler/tests/checker_smoke.rs`, `compiler/tests/parser_smoke.rs`).

```
tests/
    compiler/      # end-to-end front-end suites (source -> errors/IR)
    runtime/       # GC, object model, builtins under stress
    integration/   # full `pickle build` -> execute pipelines
```

Run everything with:

```
cargo test --workspace
```