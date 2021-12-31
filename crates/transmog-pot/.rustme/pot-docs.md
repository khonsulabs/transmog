[Transmog](https://github.com/khonsulabs/transmog) implementation of the [Pot](https://github.com/khonsulabs/pot) format.

![transmog-pot forbids unsafe code](https://img.shields.io/badge/unsafe-forbid-success)
[![crate version](https://img.shields.io/crates/v/transmog-pot.svg)](https://crates.io/crates/transmog-pot)
[![Documentation for `main` branch](https://img.shields.io/badge/docs-main-informational)](https://khonsulabs.github.io/transmog/main/transmog_pot/)

This crate provides a [`Format`][format] trait implementation using the [`Pot`][pot-type] type:

```rust
use transmog::{Format, OwnedDeserializer};
use transmog_pot::Pot;

let pot = Pot::default();
let serialized = pot.serialize(&42_u64).unwrap();
let deserialized: u64 = pot.deserialize_owned(&serialized).unwrap();
assert_eq!(deserialized, 42);
```

[pot-type]: https://khonsulabs.github.io/transmog/main/transmog_pot/struct.Pot.html
[format]: $format$
[transmog-async]: $transmog-async$
[transmog-bincode]: $transmog-bincode$
[transmog-cbor]: $transmog-cbor$
[transmog-json]: $transmog-json$
[transmog-pot]: $transmog-pot$
[transmog-versions]: $transmog-versions$
