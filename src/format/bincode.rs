use std::io::{Read, Write};

use serde::{Deserialize, Serialize};

use crate::format::Format;

pub struct Bincode;

impl<T> Format<T> for Bincode
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    type Error = bincode::Error;

    fn serialize(value: &T) -> Result<Vec<u8>, Self::Error> {
        bincode::serialize(value)
    }

    fn serialize_into<W: Write>(value: &T, writer: W) -> Result<(), Self::Error> {
        bincode::serialize_into(writer, value)
    }

    fn deserialize(data: &[u8]) -> Result<T, Self::Error> {
        bincode::deserialize(data)
    }

    fn deserialize_from<R: Read>(reader: R) -> Result<T, Self::Error> {
        bincode::deserialize_from(reader)
    }
}
