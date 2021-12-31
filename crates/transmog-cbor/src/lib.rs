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
use serde::{de::DeserializeOwned, Serialize};
pub use transmog;
use transmog::{Format, OwnedDeserializer};

/// CBOR implementor of [`Format`].
#[derive(Clone, Default)]
pub struct Cbor;

impl<'a, T> Format<'a, T> for Cbor
where
    T: Serialize,
{
    type Error = Error;

    fn serialize_into<W: Write>(&self, value: &T, writer: W) -> Result<(), Self::Error> {
        ciborium::ser::into_writer(value, writer).map_err(Error::from)
    }
}

impl<T> OwnedDeserializer<T> for Cbor
where
    T: Serialize + DeserializeOwned,
{
    fn deserialize_owned(&self, data: &[u8]) -> Result<T, Self::Error> {
        self.deserialize_from(data)
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
