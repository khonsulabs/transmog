use std::{fmt::Display, io::Write};

use ordered_varint::Variable;

const MAGIC_CODE: &[u8] = b"DVer";

pub trait ConstVersioned {
    const VERSION: u64;
}

pub trait Versioned {
    fn version(&self) -> u64;
}

impl<T: ConstVersioned> Versioned for T {
    fn version(&self) -> u64 {
        T::VERSION
    }
}

impl Versioned for u64 {
    fn version(&self) -> u64 {
        *self
    }
}

pub fn header(version: u64) -> Option<Vec<u8>> {
    if version > 0 {
        let mut header = Vec::with_capacity(13);
        header.extend(MAGIC_CODE);
        version
            .encode_variable(&mut header)
            .expect("version too large");
        Some(header)
    } else {
        None
    }
}

pub fn footer() -> &'static [u8] {
    MAGIC_CODE
}

pub fn encode<
    E: From<std::io::Error>,
    V: Versioned,
    W: Write,
    F: FnOnce(&mut W) -> Result<(), E>,
>(
    versioned: V,
    mut write: W,
    callback: F,
) -> Result<(), E> {
    if let Some(header) = header(versioned.version()) {
        write.write_all(&header)?;
        callback(&mut write)?;
        write.write_all(footer())?;
        Ok(())
    } else {
        callback(&mut write)
    }
}

pub fn wrap<V: Versioned>(versioned: &V, data: &mut Vec<u8>) {
    if let Some(header) = header(versioned.version()) {
        data.reserve(header.len() + footer().len());
        data.splice(0..0, header);
        data.extend(footer());
    }
}

pub fn decode<E: Display, R, F: FnOnce(u64, &[u8]) -> Result<R, Error<E>>>(
    data: &[u8],
    callback: F,
) -> Result<R, Error<E>> {
    if data.starts_with(MAGIC_CODE) && data.ends_with(MAGIC_CODE) {
        let (_magic_code, mut remaining) = data.split_at(MAGIC_CODE.len());
        let version = u64::decode_variable(&mut remaining)?;
        callback(version, &remaining[..remaining.len() - MAGIC_CODE.len()])
    } else {
        callback(0, data)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error<E: Display> {
    /// An unknown version was encountered.
    #[error("{0}")]
    UnknownVersion(#[from] UnknownVersion),
    /// An io error occurred
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Other(E),
}

#[derive(thiserror::Error, Debug)]
#[error("unknown version: {0}")]
pub struct UnknownVersion(pub u64);

#[test]
fn basic_tests() {
    use std::convert::Infallible;
    let payload = b"hello world";
    let mut wrapped_with_0_version = Vec::new();
    encode::<std::io::Error, _, _, _>(0_u64, &mut wrapped_with_0_version, |out| {
        out.extend(payload);
        Ok(())
    })
    .unwrap();
    decode::<Infallible, _, _>(&wrapped_with_0_version, |version, contained| {
        assert_eq!(version, 0);
        assert_eq!(contained, payload);
        Ok(())
    })
    .unwrap();

    let mut wrapped_with_1_version = Vec::new();
    encode::<std::io::Error, _, _, _>(1_u64, &mut wrapped_with_1_version, |out| {
        out.extend(payload);
        Ok(())
    })
    .unwrap();
    decode::<Infallible, _, _>(&wrapped_with_1_version, |version, contained| {
        assert_eq!(version, 1);
        assert_eq!(contained, payload);
        Ok(())
    })
    .unwrap();
}
