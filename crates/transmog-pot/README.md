# transmog-pot

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
[format]: https://docs.rs/transmog/*/transmog/trait.Format.html
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
