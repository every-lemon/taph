# Taph 0.1 language contract — proposed

This is a design baseline, not an implemented feature list. Changes require a specification PR and matching conformance cases.

## Values and types

`let` introduces a mutable binding; `const` introduces an immutable binding. Types are inferred locally and must remain consistent. Functions have annotated parameter and return types in 0.1. There is no implicit `any`, undefined value, nullable union, structural truthiness or implicit string/number conversion.

Initial types: `bool`, signed `i64`, IEEE-754 `f64`, UTF-8 `string`, `void`, homogeneous `Array<T>`, and named records with fixed field types. Integer literals default to `i64`; literals containing a decimal point or exponent default to `f64`. Conversions require explicit built-ins and reject out-of-range values. `+` concatenates strings only when both operands are strings.

Records and arrays use reference identity; assignments share their managed object. Strings are immutable. References cannot form cycles in 0.1: the compiler rejects recursive record graphs and there are no closures, reference-bearing sum types or pointers. Automatic reference counting releases managed values deterministically. This restriction must be enforced rather than silently leaking cycles. A tracing collector or ownership system is a later design proposal, not part of the first runtime.

## Syntax and control flow

Use braces, mandatory semicolons, `//` and `/* */` comments, `fn name(arg: Type): Type`, `return`, `if`/`else`, `while`, `for (let i = 0; condition; update)`, `break` and `continue`. Conditions require `bool`. Evaluation and argument order are left to right. Logical `&&` and `||` short-circuit. Assignment is a statement, not a truthy expression. Block scope has no hoisting. Function recursion is supported.

Records are declared with `record Name { field: Type; }` and constructed with `Name { field: expression }`. Array literals must have one element type; empty arrays need an annotation. Array indexing is zero-based and bounds checked; `.length` returns `i64`. UTF-8 strings do not support byte-style integer indexing in 0.1. No classes, prototypes, optional chaining, async/await or dynamic evaluation.

## Arithmetic, errors and resource behavior

Integer overflow, division by zero, invalid conversion and out-of-bounds access produce a runtime trap with nonzero exit status and source location when available. Debug and release builds preserve these semantics; optimizers cannot treat user input as unchecked undefined behavior. Floating-point behavior follows IEEE-754 and preserves NaN/infinity; no fast-math by default.

There are no catchable exceptions in 0.1. Recoverable I/O uses explicit status results in the minimal runtime API. Runtime exhaustion terminates with a diagnostic. No claim of hard real-time execution is made.

## Modules, runtime and entrypoint

Use `import { name } from "./module.tph";` with explicit relative module paths, and `export fn` or `export record`. Reject import cycles for 0.1. Module initialization follows dependency order. The program entry is `fn main(): i64`; its result becomes the process exit code, constrained to 0–255. Missing or duplicate entrypoints are compile errors.

The initial runtime exposes `print(string): void` and explicit numeric-to-string functions. File/network/process APIs, FFI and a public package registry are deferred. Implementing new runtime functionality requires a separate capability and error contract.

## Example

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
