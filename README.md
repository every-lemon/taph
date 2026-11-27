# Taph

A Rust-inspired, statically typed language planned for native compilation.

## Status

|                                      |                                          |
| ------------------------------------ | ---------------------------------------- |
| Stage                                | Language-design preview                  |
| Compiler                             | There is no working compiler release yet |
| Performance and compatibility claims | None until measured                      |

## Example

Proposed syntax. It cannot be run today.

```taph
fn sum(limit: i64): i64 {
  let total = 0;
  for (let i = 0; i < limit; i = i + 1) {
    total = total + i;
  }
  return total;
}
fn main(): i64 {
  print(i64ToString(sum(100)));
  return 0;
}
```

## Design

| Goal            | Detail                                                           |
| --------------- | ---------------------------------------------------------------- |
| Rust-inspired    | Braces, functions and expressions; not Rust-compatible          |
| Inferred types  | Static types with optional annotations                           |
| Safe by default | Explicit conversions, defined arithmetic, bounds-checked access  |
| Native output   | Rust compiler, LLVM ahead-of-time code for Linux amd64 and arm64 |
| Measured claims | Reproducible benchmarks before any speed claim                   |

## Documentation

|                           |                                              |
| ------------------------- | -------------------------------------------- |
| Language specification    | [docs/LANGUAGE.md](docs/LANGUAGE.md)         |
| Compiler architecture     | [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) |
| Roadmap                   | [docs/ROADMAP.md](docs/ROADMAP.md)           |
| Testing and benchmarks    | [docs/TESTING.md](docs/TESTING.md)           |
| Examples (not executable) | [examples](examples)                         |

Planned layout: `crates/` compiler and runtime, `tests/` conformance, `benchmarks/`, `packaging/`.

## Contributing

Discuss changes through issues and pull requests. Mark proposals as proposals; do not describe unimplemented syntax as supported.

## License

Apache-2.0. Programs compiled with Taph are not covered by this license. Third-party components keep their own licenses.
