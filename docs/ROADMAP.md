# Ordered milestones

1. **Language contract:** settle grammar, type rules, integer/string semantics, module rules and the no-cycle memory restriction. Exit: positive/negative examples cover every rule; no unresolved language choices block the frontend.
2. **Frontend and diagnostics:** Rust workspace, lexer, parser, module resolution and type checker. Exit: `taph check` passes conformance/error-location tests and rejects unsupported JavaScript syntax explicitly.
3. **Verified native backend spike:** assess and pin LLVM/bindings, lower typed control flow, verify IR, link a minimal program. Exit: compile/run on native Linux amd64 and arm64 environments; compare O0/O2 semantic results.
4. **Runtime and safety:** checked arithmetic, reference counting, arrays, UTF-8 strings and basic output. Exit: trap, lifetime, memory-leak and error-path tests pass; recursive ownership graphs are rejected.
5. **Developer tooling:** command UX, formatter, JSON diagnostics and documentation. Exit: deterministic output and clean-checkout onboarding checks.
6. **Benchmarks and release qualification:** publish workload sources, versions, hardware, repeated measurements and limitations. Exit: independently reproducible results, signed source/binary packages and platform matrix. Versioning is independent of KagNode application 5.0.x releases.

This first repository delivery ends after documentation and tracking are published. Subsequent milestones require real implementation and must not be marked complete from planning documents alone.
