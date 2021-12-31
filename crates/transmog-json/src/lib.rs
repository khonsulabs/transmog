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

use serde::{de::DeserializeOwned, Deserialize, Serialize};
pub use serde_json;
pub use transmog;
use transmog::{BorrowedDeserializer, Format, OwnedDeserializer};

/// Json implementor of [`Format`].
#[derive(Clone, Default)]
#[must_use]
pub struct Json {
    pretty: bool,
}

impl Json {
    /// Returns an instance configured to serialize in a "pretty" format.
    pub fn pretty(mut self) -> Self {
        self.pretty = true;
        self
    }
}

impl<'a, T> Format<'a, T> for Json
where
    T: Serialize,
{
    type Error = Error;

    fn serialize(&self, value: &T) -> Result<Vec<u8>, Self::Error> {
        if self.pretty {
            serde_json::to_vec_pretty(value).map_err(Error::from)
        } else {
            serde_json::to_vec(value).map_err(Error::from)
        }
    }

    fn serialize_into<W: Write>(&self, value: &T, writer: W) -> Result<(), Self::Error> {
        if self.pretty {
            serde_json::to_writer_pretty(writer, value).map_err(Error::from)
        } else {
            serde_json::to_writer(writer, value).map_err(Error::from)
        }
    }
}

impl<'a, T> BorrowedDeserializer<'a, T> for Json
where
    T: Serialize + Deserialize<'a>,
{
    fn deserialize_borrowed(&self, data: &'a [u8]) -> Result<T, Self::Error> {
        serde_json::from_slice(data).map_err(Error::from)
    }
}

impl<T> OwnedDeserializer<T> for Json
where
    T: Serialize + DeserializeOwned,
{
    fn deserialize_owned(&self, data: &[u8]) -> Result<T, Self::Error> {
        serde_json::from_slice(data).map_err(Error::from)
    }
    fn deserialize_from<R: Read>(&self, reader: R) -> Result<T, Self::Error> {
        serde_json::from_reader(reader).map_err(Error::from)
    }
}

#[test]
fn format_tests() {
    transmog::test_util::test_format(&Json::default());
    transmog::test_util::test_format(&Json::default().pretty());
}

/// Errors from [`Json`].
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// An error occurred from parsing `Json`.
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    /// An Io error occurred outside of parsing `Json`.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
