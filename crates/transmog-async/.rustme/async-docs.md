Asynchronous access to a Transmog-encoded item stream.

![transmog-async forbids unsafe code](https://img.shields.io/badge/unsafe-forbid-success)
[![crate version](https://img.shields.io/crates/v/transmog-async.svg)](https://crates.io/crates/transmog-async)
[![Documentation for `main` branch](https://img.shields.io/badge/docs-main-informational)](https://khonsulabs.github.io/transmog/main/transmog_async/)


This crate enables you to asynchronously read from a Transmog-encoded
stream, or write transmog-encoded values. Most serialization format do not
natively support serializing and deserializing in an asynchronous
environment.

Transmog works around that on the receive side by buffering received bytes
until a full element's worth of data has been received, and only then
calling into the underlying [`Format`][format]. To make this work, it relies on the
sender to prefix each encoded element with its encoded size.

On the write side, Transmog buffers the serialized values, and
asynchronously sends the resulting bytestream.

This crate has been adapted from
[`async-bincode`](https://github.com/jonhoo/async-bincode) to generically
support the [`Format`][format] trait.

[format]: $format$
