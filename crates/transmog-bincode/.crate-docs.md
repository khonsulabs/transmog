[Transmog](https://github.com/khonsulabs/transmog) implementation of the [Bincode][bincode] format.

![transmog-bincode forbids unsafe code](https://img.shields.io/badge/unsafe-forbid-success)
[![crate version](https://img.shields.io/crates/v/transmog-bincode.svg)](https://crates.io/crates/transmog-bincode)
[![Documentation for `main` branch](https://img.shields.io/badge/docs-main-informational)](https://khonsulabs.github.io/transmog/main/transmog_bincode/)

This crate provides a [`Format`][format] trait implementation using the [`Bincode`][bincode-type] type:

```rust
use transmog::Format;
use transmog_bincode::Bincode;

let bincode = Bincode::default();
let serialized = bincode.serialize(&42_u64).unwrap();
let deserialized: u64 = bincode.deserialize(&serialized).unwrap();
assert_eq!(deserialized, 42);
```

`Bincode::default()` returns an instance configured to be equivalent to using
[`bincode::DefaultOptions`](https://docs.rs/bincode/latest/bincode/config/struct.DefaultOptions.html).
If you're working with existing data that used the global
serialization/deserialization methods, use `Bincode::legacy_default()` instead:

```rust
use transmog::Format;
use transmog_bincode::Bincode;

let bincode = Bincode::legacy_default();
let serialized = bincode.serialize(&42_u64).unwrap();
let deserialized: u64 = bincode.deserialize(&serialized).unwrap();
assert_eq!(deserialized, 42);
```

[Bincode][bincode-type] offers all configuration options [bincode][bincode] exposes.

[format]: https://khonsulabs.github.io/transmog/main/transmog/trait.Format.html
[bincode]: https://github.com/bincode-org/bincode
[bincode-type]: https://khonsulabs.github.io/transmog/main/transmog_cbor/struct.Cbor.html
[transmog-async]: https://crates.io/crates/transmog-async
[transmog-bincode]: https://crates.io/crates/transmog-bincode
[transmog-cbor]: https://crates.io/crates/transmog-cbor
[transmog-pot]: https://crates.io/crates/transmog-pot
[transmog-versions]: https://crates.io/crates/transmog-versions
