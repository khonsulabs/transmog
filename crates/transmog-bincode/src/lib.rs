use std::io::{Read, Write};

pub use bincode;
use serde::{ser::Error, Deserialize, Serialize};
use transmog::Format;

#[derive(Clone)]
pub struct Bincode;

impl<T> Format<T> for Bincode
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    type Error = bincode::Error;

    fn serialized_size(&self, value: &T) -> Result<Option<usize>, Self::Error> {
        bincode::serialized_size(value).and_then(|size| {
            usize::try_from(size)
                .map(Some)
                .map_err(bincode::Error::custom)
        })
    }

    fn serialize(&self, value: &T) -> Result<Vec<u8>, Self::Error> {
        bincode::serialize(value)
    }

    fn serialize_into<W: Write>(&self, value: &T, writer: W) -> Result<(), Self::Error> {
        bincode::serialize_into(writer, value)
    }

    fn deserialize(&self, data: &[u8]) -> Result<T, Self::Error> {
        bincode::deserialize(data)
    }

    fn deserialize_from<R: Read>(&self, reader: R) -> Result<T, Self::Error> {
        bincode::deserialize_from(reader)
    }
}

#[test]
fn format_tests() {
    transmog::test_util::test_format(Bincode);
}
