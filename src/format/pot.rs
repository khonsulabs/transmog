use std::io::{Read, Write};

use serde::{Deserialize, Serialize};

use crate::format::Format;

pub struct Pot;

impl<T> Format<T> for Pot
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    type Error = pot::Error;

    fn serialize(value: &T) -> Result<Vec<u8>, Self::Error> {
        pot::to_vec(value)
    }

    fn serialize_into<W: Write>(value: &T, writer: W) -> Result<(), Self::Error> {
        pot::to_writer(value, writer)
    }

    fn deserialize(data: &[u8]) -> Result<T, Self::Error> {
        pot::from_slice(data)
    }

    fn deserialize_from<R: Read>(reader: R) -> Result<T, Self::Error> {
        pot::from_reader(reader)
    }
}
