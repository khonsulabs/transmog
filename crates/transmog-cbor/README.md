# transmog-cbor

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

[cbor-type]: https://khonsulabs.github.io/transmog/main/transmog_cbor/struct.Cbor.html
[format]: https://khonsulabs.github.io/transmog/main/transmog/trait.Format.html
[transmog-async]: https://crates.io/crates/transmog-async
[transmog-bincode]: https://crates.io/crates/transmog-bincode
[transmog-cbor]: https://crates.io/crates/transmog-cbor
[transmog-pot]: https://crates.io/crates/transmog-pot
[transmog-versions]: https://crates.io/crates/transmog-versions

## Open-source Licenses

This project, like all projects from [Khonsu Labs](https://khonsulabs.com/), are
open-source. This repository is available under the [MIT License](./LICENSE-MIT)
or the [Apache License 2.0](./LICENSE-APACHE).

To learn more about contributing, please see [CONTRIBUTING.md](./CONTRIBUTING.md).
