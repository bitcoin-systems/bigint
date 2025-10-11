# bigint-rs

A small, from-scratch arbitrary-precision integer library written in Rust.

**bigint-rs** provides a clean, ergonomic `BigInt` type implementing basic arithmetic, conversions, and common utilities — no external crates required. It's designed for learning, embedded-friendly usage, and projects that want a lightweight bigint implementation without pulling in large dependencies.

---

## Table of contents

* [Why this crate?](#why-this-crate)
* [Features](#features)
* [Quick start](#quick-start)
* [Examples](#examples)
* [API overview](#api-overview)
* [Design notes & internals](#design-notes--internals)
* [Performance & benchmarks](#performance--benchmarks)
* [Compatibility & `no_std`](#compatibility--no_std)
* [Roadmap](#roadmap)
* [Contributing](#contributing)
* [License](#license)

---

# Why this crate?

There are full-featured bigint crates in the Rust ecosystem (for example, `num-bigint`). This project is intentionally small and self-contained:

* Implemented from scratch to demonstrate algorithms and internals.
* Minimal surface area: easy to audit and reason about.
* Educational: good reference for learning big-integer arithmetic, carrying/borrowing propagation, and implementing arithmetic traits.
* Useful in `no_std` or constrained environments where pulling in large dependencies is undesirable.

> Not intended as a drop-in replacement for high-performance, battle-tested libraries in cryptography or heavy numeric workloads.

# Features

* Signed arbitrary-precision integers (`BigInt`) backed by a `Vec<u32>` limb representation.
* Implementations of the standard arithmetic traits: `Add`, `Sub`, `Mul`, `Div`, `Rem`, `Neg`.
* Comparison traits: `PartialEq`, `Eq`, `PartialOrd`, `Ord`.
* Conversion helpers: `From<i64>`, `TryFrom<&str>`, `to_string()` / `Display`.
* Bitwise operations: `BitAnd`, `BitOr`, `BitXor`, `Shl`, `Shr`.
* Utilities: `abs()`, `sign()`, `is_zero()`, `gcd()`, `pow_mod()`.
* Optional `serde` support behind a `serde` feature (if enabled).
* `no_std` compatible with the `alloc` feature (optional).

# Quick start

Add the crate to your `Cargo.toml` (replace with the published crate name or use `path` during local development):

```toml
[dependencies]
bigint-rs = { path = "../bigint-rs" }
# or if published
# bigint-rs = "0.1"
```

Usage example:

```rust
use bigint_rs::BigInt;

fn main() {
    let a = BigInt::from(1234567890123456789i128);
    let b = BigInt::from(987654321098765432i128);

    let sum = &a + &b;
    println!("{} + {} = {}", a, b, sum);

    let product = &a * &b;
    println!("product has {} decimal digits", product.to_string().len());
}
```

# Examples

Parsing from string and arithmetic:

```rust
use bigint_rs::BigInt;

let a = BigInt::from_str("-100000000000000000000").unwrap();
let b = BigInt::from(42);
let c = a / b; // division
assert_eq!(c.to_string(), "-2380952380952380952");
```

Modular exponentiation (pow mod):

```rust
use bigint_rs::BigInt;

let base = BigInt::from(7);
let exp = BigInt::from(128);
let modu = BigInt::from(1000);
let r = base.pow_mod(&exp, &modu);
println!("7^128 mod 1000 = {}", r);
```

# API overview

> This is a high-level summary. See the crate docs for exhaustive signatures and examples.

## Common types

* `BigInt` — main signed arbitrary-precision integer.

## Constructors / conversions

* `BigInt::zero()` / `BigInt::one()`
* `From<i8|i16|i32|i64|i128|isize>`
* `TryFrom<&str>` — fallible decimal (or hex with `0x`) parsing.
* `from_bytes_be / from_bytes_le` and `to_bytes_be / to_bytes_le` helpers.

## Arithmetic

* `impl Add, Sub, Mul, Div, Rem` for `&BigInt` and `BigInt` combinations.
* `neg()` / unary `-`.
* `div_mod` — compute quotient and remainder together.

## Bitwise

* `BitAnd`, `BitOr`, `BitXor`, `Shl`, `Shr`.

## Utility functions

* `abs()`, `is_zero()`, `sign()`
* `gcd(&self, &other)`
* `pow_mod(base, exponent, modulus)` — modular exponentiation using square-and-multiply.

# Design notes & internals

* **Representation**: signed magnitude using a `Vec<u32>` of limbs (least-significant limb first) and a sign flag.
* **Normalization**: leading zero limbs are stripped after every operation to keep canonical form.
* **Algorithms**:

  * Addition/subtraction: limb-by-limb with carry/borrow.
  * Multiplication: grade-school O(n²) multiplication; algorithm is simple and predictable. (Planned: Karatsuba for larger sizes.)
  * Division: long division algorithm with normalized divisor.
  * Modular exponentiation: binary exponentiation.
* **Safety**: heavily uses safe Rust. Internal helper functions are marked `#[inline]` where helpful.

# Performance & benchmarks

This crate prioritizes clarity over raw performance. The current implementation is suitable for moderate-sized integers (hundreds to low thousands of bits). For cryptographic-sized workloads or high-performance needs, consider `rug` or `num-bigint` which use optimized algorithms and native bindings.

If you want to benchmark:

1. Add a `benches/` directory and use `criterion`.
2. Compare operations: `add`, `mul`, `div`, `pow_mod` across various bit sizes.

# Compatibility & `no_std`

The crate can be built without the Rust standard library if you enable the `alloc` feature and supply an allocator in your environment. When using `no_std`, the `serde` feature is disabled.

# Roadmap

Planned improvements:

* Karatsuba / Toom-Cook multiplication for large operands.
* Optional limb-size feature to switch between `u32` and `u64` limbs (tradeoffs in portability vs speed).
* Constant-time variants for cryptographic use-cases.
* Improved division algorithms and performance tuning.

# Contributing

Contributions are welcome! Suggested workflow:

1. Fork the repo and create a feature branch.
2. Add tests for new features or bug fixes.
3. Run `cargo test` and, if relevant, `cargo bench` (when adding/adjusting algorithms).
4. Open a PR with a clear description and benchmarks if performance is affected.

Please follow idiomatic Rust coding style and add documentation comments for public APIs.

# License

Dual-licensed under MIT OR Apache-2.0 — pick the license that best suits your project.

---

If you'd like, I can:

* Generate the corresponding `Cargo.toml` and module skeleton for the crate.
* Add example unit tests and a small benchmark suite using `criterion`.

Tell me which one you want next and I’ll add it to the repo.
