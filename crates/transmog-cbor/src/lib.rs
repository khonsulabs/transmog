#![doc = include_str!("./.crate-docs.md")]
#![forbid(unsafe_code)]
#![warn(
    clippy::cargo,
    missing_docs,
    // clippy::missing_docs_in_private_items,
    clippy::pedantic,
    future_incompatible,
    rust_2018_idioms,
)]
#![allow(
    clippy::missing_errors_doc, // TODO clippy::missing_errors_doc
    clippy::option_if_let_else,
)]

use std::io::{Read, Write};

pub use ciborium;
use serde::{Deserialize, Serialize};
use transmog::Format;

/// CBOR implementor of [`Format`].
#[derive(Clone, Default)]
pub struct Cbor;

impl<T> Format<T> for Cbor
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    type Error = Error;

    fn serialize_into<W: Write>(&self, value: &T, writer: W) -> Result<(), Self::Error> {
        ciborium::ser::into_writer(value, writer).map_err(Error::from)
    }

    fn deserialize_from<R: Read>(&self, reader: R) -> Result<T, Self::Error> {
        ciborium::de::from_reader(reader).map_err(Error::from)
    }
}

/// CBOR serialization and deserialization errors.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// A serialization-related error.
    #[error("serialization error: {0}")]
    Serialization(#[from] ciborium::ser::Error<std::io::Error>),
    /// A deserialization-related error.
    #[error("serialization error: {0}")]
    Deserialization(#[from] ciborium::de::Error<std::io::Error>),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Serialization(ciborium::ser::Error::Io(err))
    }
}

#[test]
fn format_tests() {
    transmog::test_util::test_format(&Cbor);
}
