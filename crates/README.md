# All Crates

- [`transmog`](./transmog): Base crate that defines the [`Format`][format] trait.

## Formats


- [`transmog-bincode`](./transmog-bincode): [`Bincode`](https://crates.io/crates/bincode) format support.
- [`transmog-cbor`](./transmog-cbor): CBOR format support, powered by
  [`Ciborium`](https://crates.io/crates/ciborium).
- [`transmog-json`](./transmog-json): JSON format support, powered by
  [`serde_json`](https://crates.io/crates/serde_json).
- [`transmog-pot`](./transmog-pot): [`Pot`](https://crates.io/crates/pot) format support.

## Utilities

- [`transmog-async`](./transmog-async): Utilities for reading from/writing to
  `futures::Stream`s.
- [`transmog-versions`](./transmog-versions): Utilities for migrating between
  formats and/or major versions of data structures.

[format]: https://khonsulabs.github.io/transmog/main/transmog/trait.Format.html