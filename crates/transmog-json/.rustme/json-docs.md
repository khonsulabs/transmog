[Transmog](https://github.com/khonsulabs/transmog) implementation of the JSON format, powered by the [serde_json](https://github.com/serde-rs/json) crate.

![transmog-json forbids unsafe code](https://img.shields.io/badge/unsafe-forbid-success)
[![crate version](https://img.shields.io/crates/v/transmog-json.svg)](https://crates.io/crates/transmog-json)
[![Documentation for `main` branch](https://img.shields.io/badge/docs-main-informational)](https://khonsulabs.github.io/transmog/main/transmog_json/)

This crate provides a [`Format`][format] trait implementation using the [`Json`][json-type] type:

```rust
use transmog::{Format, OwnedDeserializer};
use transmog_json::Json;

let json = Json::default();
let serialized = json.serialize(&42_u64).unwrap();
let deserialized: u64 = json.deserialize_owned(&serialized).unwrap();
assert_eq!(deserialized, 42);
```

[json-type]: $json-type$
[format]: $format$
[transmog-async]: $transmog-async$
[transmog-bincode]: $transmog-bincode$
[transmog-cbor]: $transmog-cbor$
[transmog-json]: $transmog-json$
[transmog-pot]: $transmog-pot$
[transmog-versions]: $transmog-versions$
