# Contributing to PickleScript

Thanks for your interest in contributing.

## Ground rules

- **Keep it green.** Every commit compiles with zero warnings;
  `cargo test --workspace` passes.
- **Small, focused changes.** One logical change per commit; use conventional
  commit messages (`feat:`, `fix:`, `chore:`, `docs:`, `refactor:`).
- **Front end before back end.** A feature lands end-to-end per
  [docs/09-roadmap.md](docs/09-roadmap.md); never half-compiled.
- **Do not** use `git add .`, `git commit -am`, `git reset --hard`, or
  `git clean -fd` in this repository.

## Development loop

```
cargo build --workspace     # must be warning-free
cargo test --workspace      # front-end + runtime smoke tests
```

Toolchain notes: Rust stable, 1.98.1. On Windows the linker driver uses
[w64devkit](https://github.com/skeeto/w64devkit) `cc` (no MSVC required).

## Proposing changes

1. Fork the repository and create a branch.
2. Implement your change with tests.
3. Open a pull request describing the change and how it was verified.

By contributing you agree that your contributions are licensed under the
Apache License 2.0 (see [LICENSE](LICENSE)).