use std::io::{Read, Write};

#[cfg(feature = "pot")]
mod pot;
#[cfg(feature = "pot")]
pub use self::pot::*;
#[cfg(feature = "bincode")]
mod bincode;
#[cfg(feature = "bincode")]
pub use self::bincode::*;

pub trait Format<T> {
    type Error;

    fn serialize(value: &T) -> Result<Vec<u8>, Self::Error>;
    fn serialize_into<W: Write>(value: &T, writer: W) -> Result<(), Self::Error>;
    fn deserialize(data: &[u8]) -> Result<T, Self::Error>;
    fn deserialize_from<R: Read>(reader: R) -> Result<T, Self::Error>;
}
