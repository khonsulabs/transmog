use std::{
    fmt::{Debug, Display},
    io::{Read, Write},
};

#[cfg(feature = "pot")]
mod pot;
#[cfg(feature = "pot")]
pub use self::pot::*;
#[cfg(feature = "bincode")]
mod bincode;
#[cfg(feature = "bincode")]
pub use self::bincode::*;

pub trait Format<T>: Send + Sync {
    type Error: From<std::io::Error> + Debug + Display;

    fn serialize(&self, value: &T) -> Result<Vec<u8>, Self::Error>;
    fn serialize_into<W: Write>(&self, value: &T, writer: W) -> Result<(), Self::Error>;
    fn deserialize(&self, data: &[u8]) -> Result<T, Self::Error>;
    fn deserialize_from<R: Read>(&self, reader: R) -> Result<T, Self::Error>;
}
