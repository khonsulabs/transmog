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
use serde::{de::DeserializeOwned, Deserialize, Serialize};
pub use transmog;
use transmog::{BorrowedDeserializer, Format, OwnedDeserializer};

/// Pot implementor of [`Format`].
#[derive(Clone, Default)]
pub struct Pot(pot::Config);

impl From<pot::Config> for Pot {
    fn from(config: pot::Config) -> Self {
        Self(config)
    }
}

impl<'a, T> Format<'a, T> for Pot
where
    T: Serialize,
{
    type Error = pot::Error;

    fn serialize(&self, value: &T) -> Result<Vec<u8>, Self::Error> {
        self.0.serialize(value)
    }

    fn serialize_into<W: Write>(&self, value: &T, writer: W) -> Result<(), Self::Error> {
        self.0.serialize_into(value, writer)
    }
}

impl<'a, T> BorrowedDeserializer<'a, T> for Pot
where
    T: Serialize + Deserialize<'a>,
{
    fn deserialize_borrowed(&self, data: &'a [u8]) -> Result<T, Self::Error> {
        self.0.deserialize(data)
    }
}

impl<T> OwnedDeserializer<T> for Pot
where
    T: Serialize + DeserializeOwned,
{
    fn deserialize_owned(&self, data: &[u8]) -> Result<T, Self::Error> {
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

#[test]
fn borrowed_deserialization() {
    use std::borrow::Cow;
    #[derive(Serialize, Deserialize)]
    struct Test<'a> {
        #[serde(borrow)]
        value: Cow<'a, str>,
    }
    let pot = Pot::default();
    let value: Test<'static> = Test {
        value: Cow::Owned(String::from("hello")),
    };
    let serialized = pot.serialize(&value).unwrap();
    let pot = Pot::default();
    let deserialized: Test<'_> = pot.deserialize_borrowed(&serialized).unwrap();
    assert_eq!(deserialized.value, "hello");
    assert!(matches!(deserialized.value, Cow::Borrowed(_)));
}
