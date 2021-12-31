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

use std::{
    fmt::{Debug, Display},
    io::{Read, Write},
};

/// A serialization format.
pub trait Format<'a, T>: Send + Sync {
    /// The error type this format produces.
    type Error: From<std::io::Error> + Debug + Display;

    /// Return the number of bytes `value` will need to be serialized, or None
    /// if pre-measuring is not implemented for this format.
    #[allow(unused_variables)]
    fn serialized_size(&self, value: &T) -> Result<Option<usize>, Self::Error> {
        Ok(None)
    }

    /// Serialize `value` into a `Vec<u8>`.
    fn serialize(&self, value: &T) -> Result<Vec<u8>, Self::Error> {
        let mut bytes = if let Some(serialized_size) = self.serialized_size(value)? {
            Vec::with_capacity(serialized_size)
        } else {
            Vec::new()
        };
        self.serialize_into(value, &mut bytes)?;
        Ok(bytes)
    }

    /// Serialize `value` into `writer`.
    fn serialize_into<W: Write>(&self, value: &T, writer: W) -> Result<(), Self::Error>;
}

/// A deserializer that borrows data when possible.
pub trait BorrowedDeserializer<'a, T>: Format<'a, T> {
    /// Deserialize `T` from `data`, borrowing when possible.
    fn deserialize_borrowed(&self, data: &'a [u8]) -> Result<T, Self::Error>;
}

/// A deserializer that does not attempt to borrow data when deserializing.
pub trait OwnedDeserializer<T>: Format<'static, T> {
    /// Deserialize `T` from `data`.
    fn deserialize_owned(&self, data: &[u8]) -> Result<T, Self::Error> {
        self.deserialize_from(data)
    }

    /// Deserialize `T` from `reader`.
    fn deserialize_from<R: Read>(&self, reader: R) -> Result<T, Self::Error>;
}

/// Utilities for testing formats. Requires feature `test-util`.
#[cfg(any(test, feature = "test-util"))]
pub mod test_util;

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct U64BEFormat;

    impl<'a> Format<'a, u64> for U64BEFormat {
        type Error = std::io::Error;

        fn serialize_into<W: Write>(&self, value: &u64, mut writer: W) -> Result<(), Self::Error> {
            writer.write_all(&value.to_be_bytes())
        }

        fn serialized_size(&self, _value: &u64) -> Result<Option<usize>, Self::Error> {
            Ok(Some(std::mem::size_of::<u64>()))
        }
    }

    impl OwnedDeserializer<u64> for U64BEFormat {
        fn deserialize_from<R: Read>(&self, mut reader: R) -> Result<u64, Self::Error> {
            let mut bytes = [0_u8; 8];
            reader.read_exact(&mut bytes)?;
            Ok(u64::from_be_bytes(bytes))
        }
    }

    #[test]
    fn basic_format() {
        test_util::test_format(&U64BEFormat);
    }
}
