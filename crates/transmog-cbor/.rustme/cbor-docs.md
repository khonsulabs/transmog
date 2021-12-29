[Transmog](https://github.com/khonsulabs/transmog) implementation of the CBOR format, powered by the [Ciborium](https://github.com/enarx/ciborium) crate.

![transmog-cbor forbids unsafe code](https://img.shields.io/badge/unsafe-forbid-success)
[![crate version](https://img.shields.io/crates/v/transmog-cbor.svg)](https://crates.io/crates/transmog-cbor)
[![Documentation for `main` branch](https://img.shields.io/badge/docs-main-informational)](https://khonsulabs.github.io/transmog/main/transmog_cbor/)

This crate provides a [`Format`][format] trait implementation using the [`Cbor`][cbor-type] type:

```rust
use transmog::Format;
use transmog_cbor::Cbor;

let cbor = Cbor::default();
let serialized = cbor.serialize(&42_u64).unwrap();
let deserialized: u64 = cbor.deserialize(&serialized).unwrap();
assert_eq!(deserialized, 42);
```

[cbor-type]: $cbor-type$
[format]: $format$
[transmog-async]: $transmog-async$
[transmog-bincode]: $transmog-bincode$
[transmog-cbor]: $transmog-cbor$
[transmog-pot]: $transmog-pot$
[transmog-versions]: $transmog-versions$
