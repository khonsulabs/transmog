#![doc = include_str!("../crate-docs.md")]

use std::{
    fmt::{Debug, Display},
    io::{Read, Write},
};

pub trait Format<T>: Send + Sync {
    type Error: From<std::io::Error> + Debug + Display;

    #[allow(unused_variables)]
    fn serialized_size(&self, value: &T) -> Result<Option<usize>, Self::Error> {
        Ok(None)
    }

    fn serialize(&self, value: &T) -> Result<Vec<u8>, Self::Error> {
        let mut bytes = if let Some(serialized_size) = self.serialized_size(value)? {
            Vec::with_capacity(serialized_size)
        } else {
            Vec::new()
        };
        self.serialize_into(value, &mut bytes)?;
        Ok(bytes)
    }

    fn serialize_into<W: Write>(&self, value: &T, writer: W) -> Result<(), Self::Error>;

    fn deserialize(&self, data: &[u8]) -> Result<T, Self::Error> {
        self.deserialize_from(data)
    }

    fn deserialize_from<R: Read>(&self, reader: R) -> Result<T, Self::Error>;
}

#[cfg(feature = "test-util")]
pub mod test_util;
