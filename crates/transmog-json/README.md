# transmog-json

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

[json-type]: https://khonsulabs.github.io/transmog/main/transmog_json/struct.Json.html
[format]: https://khonsulabs.github.io/transmog/main/transmog/trait.Format.html
[transmog-async]: https://crates.io/crates/transmog-async
[transmog-bincode]: https://crates.io/crates/transmog-bincode
[transmog-cbor]: https://crates.io/crates/transmog-cbor
[transmog-json]: https://crates.io/crates/transmog-json
[transmog-pot]: https://crates.io/crates/transmog-pot
[transmog-versions]: https://crates.io/crates/transmog-versions

## Open-source Licenses

This project, like all projects from [Khonsu Labs](https://khonsulabs.com/), are
open-source. This repository is available under the [MIT License](./LICENSE-MIT)
or the [Apache License 2.0](./LICENSE-APACHE).

To learn more about contributing, please see [CONTRIBUTING.md](./CONTRIBUTING.md).
