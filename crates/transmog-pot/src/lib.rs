use std::io::{Read, Write};

use serde::{Deserialize, Serialize};

use transmog::Format;

pub use pot;

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

#[cfg(test)]
transmog::define_format_test_suite!(Pot);
