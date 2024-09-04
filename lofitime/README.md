# lofitime

`lofitime` is a Rust crate that provides wrapper types and conversion utilities
for working with both high-precision (`hifitime`) and low-precision (`chrono`)
time libraries. This crate aims to bridge the gap between these two popular
time-handling libraries in the Rust ecosystem.

## Features

- Wrapper types for `hifitime::Epoch`, `hifitime::Duration`,
  `chrono::DateTime<Utc>`, and `chrono::Duration`
- Low-friction conversions between `hifitime` and `chrono` types
- Implementation of common traits like `Deref`, `DerefMut`, and `Timelike` for
  wrapper types
- Simplified `SimpleDateLike` trait for common date operations

## Todos
- [ ] ✨Seamless✨ conversions between `hifitime` and `chrono` types
- [ ] Upstream into `hifitime`?

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
lofitime = "0.2.0"
```

Then, you can use the wrapper types in your code:

```rust
use lofitime::{HifiEpoch, LofiDateTime};

let hifi_epoch = HifiEpoch(hifitime::Epoch::from_gregorian_utc(2023, 8, 12, 15, 30, 45, 0));
let lofi_datetime: LofiDateTime = hifi_epoch.into();

assert_eq!(lofi_datetime.year(), 2023);
assert_eq!(lofi_datetime.month(), 8);
assert_eq!(lofi_datetime.day(), 12);
```

## Why use lofitime?

- **Interoperability**: Easily convert between `hifitime` and `chrono` types.
- **Simplified API**: Use a consistent interface for both high-precision and
  low-precision time operations.
- **Type Safety**: Wrapper types provide clear distinctions between different
  time representations.
