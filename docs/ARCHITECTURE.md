# Compiler architecture

Proposed implementation: Rust 2024, LLVM AOT backend, separate small runtime. Pin a supported LLVM major when the backend spike is accepted; record that choice before merging implementation. The spike must compare maintained Rust binding options and cross-target support; no unsupported binding is adopted solely to claim speed.

## Pipeline

UTF-8 source → lexer with byte spans → parser/AST → module resolution → name/type checking → typed intermediate representation → control-flow/ownership lowering → LLVM IR verification → object file → system linker → native executable.

All diagnostics use stable identifiers, source spans and human-readable output; `--json` provides machine-readable diagnostics. Compilation stops before code generation if frontend errors exist. Reproducible builds must avoid absolute source paths and nondeterministic iteration in emitted artifacts.

The runtime handles bounds/overflow traps, UTF-8 strings, reference counting and basic output. Runtime layouts and symbols are private until a versioned ABI is specified. Do not expose an FFI that could bypass language safety checks in the first milestone.

## Planned command interfaces

- `taph check path.tph [--json]`: parse, resolve and type-check without producing an executable.
- `taph build path.tph -o program [--target TARGET] [-O0|-O2]`: produce a checked native executable; default target is the host, default optimization is O0.
- `taph run path.tph -- [arguments]`: build in the configured cache and run with inherited standard streams; never implicitly download dependencies.
- `taph fmt --check [paths]`: non-mutating source-format validation; formatting without `--check` is explicit.

Initial native targets are `x86_64-unknown-linux-gnu` and `aarch64-unknown-linux-gnu`. Cross compilation requires an explicitly configured matching linker/sysroot. A successful cross-build is not execution qualification on the target.

## Repository layout

`crates/taph-cli`, `crates/taph-frontend`, `crates/taph-ir`, `crates/taph-codegen`, `crates/taph-runtime`; `tests/conformance`, `tests/diagnostics`, `benchmarks`, `examples`, `docs`, `packaging`. These directories are planned; empty placeholder crates are not shipped as a compiler.

No telemetry, network downloads or shell command interpolation belongs in compilation. Invoke the linker using argument arrays. Preserve license attribution for LLVM and every linked dependency.
