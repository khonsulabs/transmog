# transmog-versions

Multi-version/Multi-format support for [Transmog](https://github.com/khonsulabs/transmog/).

![transmog-versions forbids unsafe code](https://img.shields.io/badge/unsafe-forbid-success)
[![crate version](https://img.shields.io/crates/v/transmog-versions.svg)](https://crates.io/crates/transmog-versions)
[![Documentation for `main` branch](https://img.shields.io/badge/docs-main-informational)](https://khonsulabs.github.io/transmog/main/transmog_versions/)

This crate is early in development and experimental. A low-level API has been designed and is demonstrated in these examples:

* [switching-serializers.rs](https://github.com/khonsulabs/transmog/blob/main/examples/versions/examples/switching-serializers.rs): Demonstrates switching between serialization formats.
* [versioned-serde.rs](https://github.com/khonsulabs/transmog/blob/main/examples/versions/examples/versioned-serde.rs): Demonstrates switching between major versions of structures.

A high-level procedural macro is being designed to wrap the low-level API.

## Open-source Licenses

This project, like all projects from [Khonsu Labs](https://khonsulabs.com/), are
open-source. This repository is available under the [MIT License](./LICENSE-MIT)
or the [Apache License 2.0](./LICENSE-APACHE).

To learn more about contributing, please see [CONTRIBUTING.md](./CONTRIBUTING.md).
