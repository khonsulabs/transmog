//! Pot adaptor for Transmog.

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

pub use pot;
use serde::{Deserialize, Serialize};
use transmog::Format;

/// Pot implementor of [`Format`].
#[derive(Clone)]
pub struct Pot;

impl<T> Format<T> for Pot
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    type Error = pot::Error;

    fn serialize(&self, value: &T) -> Result<Vec<u8>, Self::Error> {
        pot::to_vec(value)
    }

    fn serialize_into<W: Write>(&self, value: &T, writer: W) -> Result<(), Self::Error> {
        pot::to_writer(value, writer)
    }

    fn deserialize(&self, data: &[u8]) -> Result<T, Self::Error> {
        pot::from_slice(data)
    }

    fn deserialize_from<R: Read>(&self, reader: R) -> Result<T, Self::Error> {
        pot::from_reader(reader)
    }
}

#[test]
fn format_tests() {
    transmog::test_util::test_format(&Pot);
}
