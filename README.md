# safe-bigmath

[![Crates.io](https://img.shields.io/crates/v/safe-bigmath.svg)](https://crates.io/crates/safe-bigmath)
[![Docs](https://docs.rs/safe-bigmath/badge.svg)](https://docs.rs/safe-bigmath)
[![CI](https://github.com/sam0x17/safe-bigmath/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/sam0x17/safe-bigmath/actions/workflows/ci.yaml)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Safe, non-panicking numeric primitives built on pure-Rust `num-bigint`. `safe-bigmath` gives you:

- `SafeInt`: arbitrary-precision integers with ergonomic operator overloads.
- `SafeDec<D>`: fixed-scale decimals backed by arbitrary-precision `SafeInt`; the const generic `D` sets how many decimal places are stored exactly.
- Parsing helpers for turning strings into safe numeric values.
- No hidden panics: division returns `Option`, parsing reports structured errors.
- `std` by default; disable default features for `no_std` + `alloc` (works on `wasm32-unknown-unknown`).

## Quick start

```toml
[dependencies]
safe-bigmath = "0.2"

# Optional: go `no_std` + `alloc`
# safe-bigmath = { version = "0.2", default-features = false }
```

### Safe integers

```rust
use safe_bigmath::SafeInt;

let a = SafeInt::from(10);
let b = SafeInt::from(3);

assert_eq!((&a / &b).unwrap(), SafeInt::from(3)); // division is fallible
assert_eq!(&a + &b, SafeInt::from(13));
assert_eq!(SafeInt::from(5) / SafeInt::from(0), None); // no panic on zero div
```

### Fixed-scale decimals

```rust
use safe_bigmath::SafeDec;

let price: SafeDec<2> = "12.50".parse().unwrap();
let qty: SafeDec<2> = "3.00".parse().unwrap();
let total = price * qty;

assert_eq!(total.to_string(), "37.50");
```

### Pow of ratios with scaling

Compute `(x / (x + dx))^(w1 / w2)` scaled to perquintill:

```rust
use safe_bigmath::SafeInt;

let x = SafeInt::from(21_000_000_000_000_000u64);
let dx = SafeInt::from(7_000_000_000_000_000u64);
let w1 = SafeInt::from(600_000_000_000_000_000u128);
let w2 = SafeInt::from(400_000_000_000_000_000u128);
let perquintill = SafeInt::from(1_000_000_000_000_000_000u128);

let result = SafeInt::pow_ratio_scaled(&x, &(x.clone() + dx), &w1, &w2, 0, &perquintill).unwrap();
assert_eq!(result, SafeInt::from(649_519_052_838_328_985u128));
```

## Feature flags

- `std` (on by default): enables `std` support for downstream crates; disable default features for `no_std` + `alloc`.

## Lencode encoding/decoding

`SafeInt` and `SafeDec<D>` implement `lencode::Encode` and `lencode::Decode`. The format is
stable and `no_std`-friendly, and always uses little-endian byte order for the payload.

`SafeInt` encoding is zigzag over the signed magnitude, then a compact header:

- Header byte layout (from MSB to LSB): `V S LLLLLL`
  - `V` (bit 7): variant. `0` = varint path, `1` = bytes path.
  - `S` (bit 6): size mode (varint path only). `0` = small (value fits in 6 bits),
    `1` = large (next `L` bytes are the little-endian payload).
  - `L` (bits 0..5): when `S=0`, the value itself; when `S=1`, the payload length in bytes.
- Varint path (V=0):
  - Small: one byte total, supports zigzag values in `[0, 63]`.
  - Large: `1 + L` bytes total, supports zigzag payloads up to 63 bytes.
- Bytes path (V=1):
  - Encodes the zigzag payload as `Vec<u8>` using lencode (varint length + payload, with
    optional compression), allowing arbitrary magnitudes.

Range note: the varint path covers signed values in `[-2^503, 2^503 - 1]`. Larger magnitudes
use the bytes path automatically.

`SafeDec<D>` encodes exactly like its underlying scaled `SafeInt` (the raw integer at scale `D`).

## Supported targets

- `std` targets (default).
- `no_std` targets with `alloc` via `--no-default-features`.
- `wasm32-unknown-unknown` (CI builds and test-compiles both `--no-default-features` and `--all-features`).

## Testing

```bash
cargo test --workspace
cargo test --workspace --no-default-features
cargo test --workspace --all-features
cargo test --target wasm32-unknown-unknown --no-default-features --no-run
cargo test --target wasm32-unknown-unknown --all-features --no-run
```

## License

MIT © sam0x17
