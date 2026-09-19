# PickleScript

**Familiar syntax. Modern design. Powerful capabilities.**

PickleScript is a standalone, native-compiled, general-purpose programming
language for games, backend services, desktop apps, CLI tools, developer
tooling, infrastructure software, and — when you need it — systems
programming.

## Quick tour

```
module hello

fn main() {
    let name = "Hero"
    print("Hello, {name}!")
}
```

## Build from source

```
cargo build --release
```

This produces `pickle` (the CLI) and `pickle_runtime` (the runtime static
library used to link executables).

## Use

```
pickle new myapp
cd myapp
pickle run
pickle build           # -> native executable
pickle test
pickle fmt
pickle repl
```

## Project layout

```
project/
    pickle.toml        # project manifest
    src/
        main.pkl       # entry point (fn main)
        player.pkl
```

`pickle build` discovers `src/*.pkl`, resolves `module`/`import` edges,
compiles them to machine code, links against the runtime, and produces an
executable. No headers. No makefiles. No CMake.

## Language

- C-family syntax, braces, parentheses around conditions, no semicolons.
- `let` / `var` / `const` bindings.
- Classes, structs, enums, interfaces, generics, single inheritance.
- Garbage collected by default; `unsafe` gives pointers and custom
  allocators.
- Pattern matching, optionals (`T?`, `none`), string interpolation,
  closures, `async`/`await`, tasks, channels.
- Integrated toolchain: build / run / fmt / test / package / doc / repl.

## Design & docs

See `docs/`:

1. [Philosophy](docs/01-philosophy.md)
2. [Syntax](docs/02-syntax.md)
3. [Class system](docs/03-classes.md)
4. [Type system](docs/04-types.md)
5. [Memory model](docs/05-memory.md)
6. [Compiler architecture](docs/06-compiler.md)
7. [Runtime architecture](docs/07-runtime.md)
8. [Standard library plan](docs/08-stdlib.md)
9. [Implementation roadmap](docs/09-roadmap.md)

## Repository layout

```
compiler/          # pickle-compiler: lexer, parser, type checker, IR, codegen, formatter
runtime/           # pickle-runtime: GC, object model, builtins, entry point
cli/               # pickle-cli: the `pickle` tool
stdlib/            # std.* source libraries (PickleScript)
docs/              # language & compiler design
rfcs/              # language design proposals
examples/          # example projects
tests/             # cross-crate test suites
benchmarks/        # compiler/runtime benchmarks
tools/             # build & release tooling
scripts/           # CI / dev scripts
```

This project is intentionally not a toy: the compiler is written in Rust,
emits native machine code, and targets real application workloads.

## License

Apache License 2.0. See [LICENSE](LICENSE) and [NOTICE](NOTICE).