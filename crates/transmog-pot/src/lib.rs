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

pub use pot;
use serde::{Deserialize, Serialize};
pub use transmog;
use transmog::Format;

/// Pot implementor of [`Format`].
#[derive(Clone, Default)]
pub struct Pot(pot::Config);

impl From<pot::Config> for Pot {
    fn from(config: pot::Config) -> Self {
        Self(config)
    }
}

impl<T> Format<T> for Pot
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    type Error = pot::Error;

    fn serialize(&self, value: &T) -> Result<Vec<u8>, Self::Error> {
        self.0.serialize(value)
    }

    fn serialize_into<W: Write>(&self, value: &T, writer: W) -> Result<(), Self::Error> {
        self.0.serialize_into(value, writer)
    }

    fn deserialize(&self, data: &[u8]) -> Result<T, Self::Error> {
        self.0.deserialize(data)
    }

    fn deserialize_from<R: Read>(&self, reader: R) -> Result<T, Self::Error> {
        self.0.deserialize_from(reader)
    }
}

#[test]
fn format_tests() {
    transmog::test_util::test_format(&Pot::default());
    transmog::test_util::test_format(&Pot::from(pot::Config::default().allocation_budget(64)));
}
