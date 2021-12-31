# Transmog

Universal data serialization utilities for Rust.

![Transmog forbids unsafe code](https://img.shields.io/badge/unsafe-forbid-success)
[![crate version](https://img.shields.io/crates/v/transmog.svg)](https://crates.io/crates/transmog)
[![Live Build Status](https://img.shields.io/github/workflow/status/khonsulabs/transmog/Tests/main)](https://github.com/khonsulabs/transmog/actions?query=workflow:Tests)
[![HTML Coverage Report for `main` branch](https://khonsulabs.github.io/transmog/coverage/badge.svg)](https://khonsulabs.github.io/transmog/coverage/)
[![Documentation for `main` branch](https://img.shields.io/badge/docs-main-informational)](https://khonsulabs.github.io/transmog/main/transmog/)

Rust has a vibrant ecosystem chock full of serialization crates. Many crates
implement a common set of traits via [Serde](https://serde.rs), but other crates
can not or chose not to support Serde.

At the end of the day, however, most serialization formats can be interacted
with in a generic fashion. The [`Format`][format] trait aims to be the universal
serialization trait for any crate that can serialize from a `std::io::Read` and
deserialize from a `std::io::Write`.

## Status of this project

We are currently at the experimentation phase of creating this ecosystem. All
constructive criticism, format requests, and questions are welcome on [Github
Issues](https://github.com/khonsulabs/transmog/issues/new). We are looking to
use this crate as a strategy of offering versioned data support in
[BonsaiDb](https://github.com/khonsulabs/bonsaidb) as well as customizable
serialization support for [`Fabruic`](https://github.com/khonsulabs/fabruic).

## Serialization format support

We accept pull requests for any moderately stable serialization API.

- [`Bincode`](https://crates.io/crates/bincode) via [`transmog-bincode`][transmog-bincode]
- CBOR via [`transmog-cbor`][transmog-cbor], powered by
  [`Ciborium`](https://crates.io/crates/ciborium).
- JSON via [`transmog-json`][transmog-json], powered by
  [`serde_json`](https://crates.io/crates/serde_json).
- [`Pot`](https://crates.io/crates/pot) via [`transmog-pot`][transmog-pot]

## Utilities for migrating data structures

Sometimes a breaking change is unavoidable. Perhaps, you've decided a different
format is better for your situation. Or, you refactored your structure so much
that serde's built-in attributes aren't enough to help.
[`transmog-versions`][transmog-versions] to the rescue!

The [`transmog-versions`][transmog-versions] crate provides APIs that allow you
to treat your currently stored data as "version 0" and provide the logic for
handling loading each version of data.

Plans to add a derive macro to remove even more boilerplate code is planned.

## Serializing/Deserializing from a `futures::Stream`

The [`transmog-async`][transmog-async] crate is a fork of
[`async-bincode`](https://crates.io/crates/async-bincde), altered to support the
[`Format`][format] trait.

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
