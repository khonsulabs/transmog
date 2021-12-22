use std::{
    fmt::Display,
    io::{BufRead, BufReader, Read, Write},
};

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

fn header(version: u64) -> Option<Vec<u8>> {
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

pub fn write_header<V: Versioned, W: Write>(
    versioned: &V,
    mut write: W,
) -> Result<(), std::io::Error> {
    if let Some(header) = header(versioned.version()) {
        write.write_all(&header)?;
    }
    Ok(())
}

pub fn wrap<V: Versioned>(versioned: &V, mut data: Vec<u8>) -> Vec<u8> {
    if let Some(header) = header(versioned.version()) {
        data.reserve(header.len());
        data.splice(0..0, header);
    }

    data
}

pub fn decode<E: Display, T, R: Read, F: FnOnce(u64, BufReader<R>) -> Result<T, Error<E>>>(
    data: R,
    callback: F,
) -> Result<T, Error<E>> {
    let mut buffered = BufReader::with_capacity(13, data);
    let mut peeked_header = buffered.fill_buf()?;

    if peeked_header.starts_with(&MAGIC_CODE[0..4]) {
        let header_start = peeked_header.as_ptr() as usize;
        peeked_header = &peeked_header[4..];

        let version = u64::decode_variable(&mut peeked_header)?;
        let header_end = peeked_header.as_ptr() as usize;
        buffered.consume(header_end - header_start);

        callback(version, buffered)
    } else {
        callback(0, buffered)
    }
}

pub fn unwrap_version(mut data: &[u8]) -> (u64, &[u8]) {
    if data.starts_with(&MAGIC_CODE[0..4]) {
        data = &data[4..];
        if let Ok(version) = u64::decode_variable(&mut data) {
            return (version, data);
        }
    }
    (0, data)
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
    write_header(&0_u64, &mut wrapped_with_0_version).unwrap();
    wrapped_with_0_version.extend(payload);
    decode::<Infallible, _, _, _>(&wrapped_with_0_version[..], |version, mut contained| {
        assert_eq!(version, 0);
        let mut bytes = Vec::new();
        contained.read_to_end(&mut bytes).unwrap();
        assert_eq!(bytes, payload);
        Ok(())
    })
    .unwrap();

    let bytes = wrap(&1_u64, payload.to_vec());
    let (version, unwrapped_bytes) = unwrap_version(&bytes);
    assert_eq!(version, 1);
    assert_eq!(unwrapped_bytes, payload);

    let unwrapped_version = unwrap_version(&payload[..]);
    assert_eq!(unwrapped_version, (0, &payload[..]));
}
