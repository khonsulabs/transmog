use std::io::{Read, Write};

pub use ciborium;
use serde::{Deserialize, Serialize};
use transmog::Format;

#[derive(Clone)]
pub struct Ciborium;

impl<T> Format<T> for Ciborium
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    type Error = Error;

    fn serialize_into<W: Write>(&self, value: &T, writer: W) -> Result<(), Self::Error> {
        ciborium::ser::into_writer(value, writer).map_err(Error::from)
    }

    fn deserialize_from<R: Read>(&self, reader: R) -> Result<T, Self::Error> {
        ciborium::de::from_reader(reader).map_err(Error::from)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("serialization error: {0}")]
    Serialization(#[from] ciborium::ser::Error<std::io::Error>),
    #[error("serialization error: {0}")]
    Deserialization(#[from] ciborium::de::Error<std::io::Error>),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Serialization(ciborium::ser::Error::Io(err))
    }
}

#[test]
fn format_tests() {
    transmog::test_util::test_format(Ciborium);
}
