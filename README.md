# iso-4217

[![CI](https://github.com/mechiru/iso-4217/workflows/CI/badge.svg)](https://github.com/mechiru/iso-4217/actions?query=workflow:CI)
[![Rust Documentation](https://docs.rs/iso-4217/badge.svg)](https://docs.rs/iso-4217)
[![Latest Version](https://img.shields.io/crates/v/iso-4217.svg)](https://crates.io/crates/iso-4217)

This library provides enumeration of [ISO-4217](https://en.wikipedia.org/wiki/ISO_4217).

```toml
[dependencies]
iso-4217 = "0.1"
```

## Example
```rust
use std::convert::TryFrom;
use iso_4217::*;

let usd: CurrencyCode = TryFrom::try_from("USD").unwrap();
assert_eq!(usd, CurrencyCode::USD);

assert_eq!(usd.alpha(), "USD");
assert_eq!(usd.num(), 840);
assert_eq!(usd.digit(), Some(2));
assert_eq!(usd.name(), "United States dollar");
```

## License
Licensed under either of [Apache License, Version 2.0](./LICENSE-APACHE) or [MIT license](./LICENSE-MIT) at your option.
