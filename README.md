# safe-math

[![Crates.io](https://img.shields.io/crates/v/safe-math.svg)](https://crates.io/crates/safe-math)
[![Docs](https://docs.rs/safe-math/badge.svg)](https://docs.rs/safe-math)
[![CI](https://github.com/sam0x17/safe-math/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/sam0x17/safe-math/actions/workflows/ci.yaml)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Safe, non-panicking numeric primitives built on pure-Rust `num-bigint`. `safe-math` gives you:

- `SafeInt`: arbitrary-precision integers with ergonomic operator overloads.
- `SafeDec<D>`: fixed-scale decimals backed by arbitrary-precision `SafeInt`; the const generic `D` sets how many decimal places are stored exactly.
- Parsing helpers for turning strings into safe numeric values.
- No hidden panics: division returns `Option`, parsing reports structured errors.
- `no_std` by default with an opt-in `std` feature; compiles for wasm (`wasm32-unknown-unknown`).

## Quick start

```toml
[dependencies]
safe-math = "0.0.1"

# Optional: enable std support
# safe-math = { version = "0.0.1", features = ["std"] }
```

### Safe integers

```rust
use safe_math::SafeInt;

let a = SafeInt::from(10);
let b = SafeInt::from(3);

assert_eq!((&a / &b).unwrap(), SafeInt::from(3)); // division is fallible
assert_eq!(&a + &b, SafeInt::from(13));
assert_eq!(SafeInt::from(5) / SafeInt::from(0), None); // no panic on zero div
```

### Fixed-scale decimals

```rust
use safe_math::SafeDec;

let price: SafeDec<2> = "12.50".parse().unwrap();
let qty: SafeDec<2> = "3.00".parse().unwrap();
let total = price * qty;

assert_eq!(total.to_string(), "37.50");
```

### Pow of ratios with scaling

Compute `(x / (x + dx))^(w1 / w2)` scaled to perquintill:

```rust
use safe_math::SafeInt;

let x = SafeInt::from(21_000_000_000_000_000u64);
let dx = SafeInt::from(7_000_000_000_000_000u64);
let w1 = SafeInt::from(600_000_000_000_000_000u128);
let w2 = SafeInt::from(400_000_000_000_000_000u128);
let perquintill = SafeInt::from(1_000_000_000_000_000_000u128);

let result = SafeInt::pow_ratio_scaled(&x, &(x.clone() + dx), &w1, &w2, 0, &perquintill).unwrap();
assert_eq!(result, SafeInt::from(649_519_052_838_328_985u128));
```

## Feature flags

- `std` (off by default): enables `std` support for downstream crates. Default build is `no_std` + `alloc`.

## Supported targets

- `no_std` targets with `alloc` (default).
- `wasm32-unknown-unknown` (CI builds and test-compiles both `--no-default-features` and `--all-features`).
- Any `std` target via the `std` feature.

## Testing

```bash
cargo test --workspace --no-default-features
cargo test --workspace --all-features
cargo test --target wasm32-unknown-unknown --no-default-features --no-run
cargo test --target wasm32-unknown-unknown --all-features --no-run
```

## License

MIT © sam0x17
