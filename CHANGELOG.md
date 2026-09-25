# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/)
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added
- Spread list arguments (`f(1, ...xs)`), `Range`/`RangeIncl` binary lowering, and stored-range `for (x in r)` iteration — with the port emitter's `Ty.Range` for-in arm matching the Rust lowering byte-for-byte.
- Fixture `tests/pickle/spread_main.pkl` exercising spread-call arity and stored-range loops.

### Added
- Workspace skeleton: `compiler`, `runtime`, `cli` crates.
- Front end: lexer, parser, AST, resolver, type checker.
- `pickle check` and `pickle ast` commands.
- Runtime object model (managed object headers, class descriptor tables).
- Project restructured for open-source release under Apache-2.0.