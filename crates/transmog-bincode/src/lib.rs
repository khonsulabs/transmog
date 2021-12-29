#![doc = include_str!("./.crate-docs.md")]
#![forbid(unsafe_code)]
#![warn(
    clippy::cargo,
    missing_docs,
    // clippy::missing_docs_in_private_items,
    clippy::pedantic,
    future_incompatible,
    rust_2018_idioms,
)]
#![allow(
    clippy::missing_errors_doc, // TODO clippy::missing_errors_doc
    clippy::option_if_let_else,
)]

use std::io::{Read, Write};

pub use bincode;
use bincode::{
    config::{
        AllowTrailing, BigEndian, Bounded, FixintEncoding, Infinite, LittleEndian, NativeEndian,
        RejectTrailing, VarintEncoding, WithOtherEndian, WithOtherIntEncoding, WithOtherLimit,
        WithOtherTrailing,
    },
    DefaultOptions, Options,
};
use serde::{ser::Error, Deserialize, Serialize};
pub use transmog;
use transmog::Format;

/// Bincode implementor of [`Format`] with default options.
#[derive(Clone)]
#[must_use]
pub struct Bincode {
    limit: Option<u64>,
    endian: Endian,
    integer_encoding: IntegerEncoding,
    reject_trailing_bytes: bool,
}

#[derive(Clone, Copy)]
enum Endian {
    Little,
    Big,
    Native,
}

#[derive(Clone, Copy)]
enum IntegerEncoding {
    Fixed,
    Variable,
}

impl Default for Bincode {
    /// Returns a `Bincode` instance initialized using the equivalent of [`DefaultOptions`].
    fn default() -> Self {
        Self {
            limit: None,
            endian: Endian::Little,
            integer_encoding: IntegerEncoding::Variable,
            reject_trailing_bytes: true,
        }
    }
}

impl Bincode {
    /// Returns a `Bincode` instance initialized using the equivalent settings
    /// that [`bincode::serialize`], [`bincode::deserialize`],
    /// [`bincode::serialize_into`], and [`bincode::deserialize_from`] use. See
    /// [`bincode::config`](mod@bincode::config) for more information.
    pub fn legacy_default() -> Self {
        Self::default()
            .fixed_integer_encoding()
            .allow_trailing_bytes()
    }

    /// Configures no byte limit. See [`Infinite`] for more information.
    pub fn no_limit(mut self) -> Self {
        self.limit = None;
        self
    }

    /// Configures bincode to restrict encoding and decoding to `byte_limit`. See [`Bounded`] for more information.
    pub fn limit(mut self, byte_limit: u64) -> Self {
        self.limit = Some(byte_limit);
        self
    }

    /// Configures big-endian encoding. See [`BigEndian`] for more information.
    pub fn big_endian(mut self) -> Self {
        self.endian = Endian::Big;
        self
    }

    /// Configures little-endian encoding. See [`LittleEndian`] for more information.
    pub fn little_endian(mut self) -> Self {
        self.endian = Endian::Little;
        self
    }

    /// Configures native-endian encoding. See [`NativeEndian`] for more information.
    pub fn native_endian(mut self) -> Self {
        self.endian = Endian::Native;
        self
    }

    /// Configures variable length integer encoding. See [`VarintEncoding`] for more information.
    pub fn variable_integer_encoding(mut self) -> Self {
        self.integer_encoding = IntegerEncoding::Variable;
        self
    }

    /// Configures fixed length integer encoding. See [`FixintEncoding`] for more information.
    pub fn fixed_integer_encoding(mut self) -> Self {
        self.integer_encoding = IntegerEncoding::Fixed;
        self
    }

    /// Configures Bincode to allow trailing bytes when deserializing. See [`AllowTrailing`] for more information.
    pub fn allow_trailing_bytes(mut self) -> Self {
        self.reject_trailing_bytes = false;
        self
    }

    /// Configures Bincode to reject trailing bytes when deserializing. See [`RejectTrailing`] for more information.
    pub fn reject_trailing_bytes(mut self) -> Self {
        self.reject_trailing_bytes = true;
        self
    }
}

impl<'a> From<&Bincode> for BincodeOptions {
    #[allow(clippy::too_many_lines)]
    fn from(settings: &Bincode) -> Self {
        match (
            settings.limit,
            settings.endian,
            settings.integer_encoding,
            settings.reject_trailing_bytes,
        ) {
            (None, Endian::Little, IntegerEncoding::Fixed, true) => {
                Self::UnlimitedLittleFixintReject(
                    DefaultOptions::default()
                        .reject_trailing_bytes()
                        .with_fixint_encoding()
                        .with_little_endian()
                        .with_no_limit(),
                )
            }
            (None, Endian::Little, IntegerEncoding::Fixed, false) => {
                Self::UnlimitedLittleFixintAllow(
                    DefaultOptions::default()
                        .allow_trailing_bytes()
                        .with_fixint_encoding()
                        .with_little_endian()
                        .with_no_limit(),
                )
            }
            (None, Endian::Little, IntegerEncoding::Variable, true) => {
                Self::UnlimitedLittleVarintReject(
                    DefaultOptions::default()
                        .reject_trailing_bytes()
                        .with_varint_encoding()
                        .with_little_endian()
                        .with_no_limit(),
                )
            }
            (None, Endian::Little, IntegerEncoding::Variable, false) => {
                Self::UnlimitedLittleVarintAllow(
                    DefaultOptions::default()
                        .allow_trailing_bytes()
                        .with_varint_encoding()
                        .with_little_endian()
                        .with_no_limit(),
                )
            }
            (None, Endian::Big, IntegerEncoding::Fixed, true) => Self::UnlimitedBigFixintReject(
                DefaultOptions::default()
                    .reject_trailing_bytes()
                    .with_fixint_encoding()
                    .with_big_endian()
                    .with_no_limit(),
            ),
            (None, Endian::Big, IntegerEncoding::Fixed, false) => Self::UnlimitedBigFixintAllow(
                DefaultOptions::default()
                    .allow_trailing_bytes()
                    .with_fixint_encoding()
                    .with_big_endian()
                    .with_no_limit(),
            ),
            (None, Endian::Big, IntegerEncoding::Variable, true) => Self::UnlimitedBigVarintReject(
                DefaultOptions::default()
                    .reject_trailing_bytes()
                    .with_varint_encoding()
                    .with_big_endian()
                    .with_no_limit(),
            ),
            (None, Endian::Big, IntegerEncoding::Variable, false) => Self::UnlimitedBigVarintAllow(
                DefaultOptions::default()
                    .allow_trailing_bytes()
                    .with_varint_encoding()
                    .with_big_endian()
                    .with_no_limit(),
            ),
            (None, Endian::Native, IntegerEncoding::Fixed, true) => {
                Self::UnlimitedNativeFixintReject(
                    DefaultOptions::default()
                        .reject_trailing_bytes()
                        .with_fixint_encoding()
                        .with_native_endian()
                        .with_no_limit(),
                )
            }
            (None, Endian::Native, IntegerEncoding::Fixed, false) => {
                Self::UnlimitedNativeFixintAllow(
                    DefaultOptions::default()
                        .allow_trailing_bytes()
                        .with_fixint_encoding()
                        .with_native_endian()
                        .with_no_limit(),
                )
            }
            (None, Endian::Native, IntegerEncoding::Variable, true) => {
                Self::UnlimitedNativeVarintReject(
                    DefaultOptions::default()
                        .reject_trailing_bytes()
                        .with_varint_encoding()
                        .with_native_endian()
                        .with_no_limit(),
                )
            }
            (None, Endian::Native, IntegerEncoding::Variable, false) => {
                Self::UnlimitedNativeVarintAllow(
                    DefaultOptions::default()
                        .allow_trailing_bytes()
                        .with_varint_encoding()
                        .with_native_endian()
                        .with_no_limit(),
                )
            }
            (Some(limit), Endian::Little, IntegerEncoding::Fixed, true) => {
                Self::LimitedLittleFixintReject(
                    DefaultOptions::default()
                        .reject_trailing_bytes()
                        .with_fixint_encoding()
                        .with_little_endian()
                        .with_limit(limit),
                )
            }
            (Some(limit), Endian::Little, IntegerEncoding::Fixed, false) => {
                Self::LimitedLittleFixintAllow(
                    DefaultOptions::default()
                        .allow_trailing_bytes()
                        .with_fixint_encoding()
                        .with_little_endian()
                        .with_limit(limit),
                )
            }
            (Some(limit), Endian::Little, IntegerEncoding::Variable, true) => {
                Self::LimitedLittleVarintReject(
                    DefaultOptions::default()
                        .reject_trailing_bytes()
                        .with_varint_encoding()
                        .with_little_endian()
                        .with_limit(limit),
                )
            }
            (Some(limit), Endian::Little, IntegerEncoding::Variable, false) => {
                Self::LimitedLittleVarintAllow(
                    DefaultOptions::default()
                        .allow_trailing_bytes()
                        .with_varint_encoding()
                        .with_little_endian()
                        .with_limit(limit),
                )
            }
            (Some(limit), Endian::Big, IntegerEncoding::Fixed, true) => {
                Self::LimitedBigFixintReject(
                    DefaultOptions::default()
                        .reject_trailing_bytes()
                        .with_fixint_encoding()
                        .with_big_endian()
                        .with_limit(limit),
                )
            }
            (Some(limit), Endian::Big, IntegerEncoding::Fixed, false) => {
                Self::LimitedBigFixintAllow(
                    DefaultOptions::default()
                        .allow_trailing_bytes()
                        .with_fixint_encoding()
                        .with_big_endian()
                        .with_limit(limit),
                )
            }
            (Some(limit), Endian::Big, IntegerEncoding::Variable, true) => {
                Self::LimitedBigVarintReject(
                    DefaultOptions::default()
                        .reject_trailing_bytes()
                        .with_varint_encoding()
                        .with_big_endian()
                        .with_limit(limit),
                )
            }
            (Some(limit), Endian::Big, IntegerEncoding::Variable, false) => {
                Self::LimitedBigVarintAllow(
                    DefaultOptions::default()
                        .allow_trailing_bytes()
                        .with_varint_encoding()
                        .with_big_endian()
                        .with_limit(limit),
                )
            }
            (Some(limit), Endian::Native, IntegerEncoding::Fixed, true) => {
                Self::LimitedNativeFixintReject(
                    DefaultOptions::default()
                        .reject_trailing_bytes()
                        .with_fixint_encoding()
                        .with_native_endian()
                        .with_limit(limit),
                )
            }
            (Some(limit), Endian::Native, IntegerEncoding::Fixed, false) => {
                Self::LimitedNativeFixintAllow(
                    DefaultOptions::default()
                        .allow_trailing_bytes()
                        .with_fixint_encoding()
                        .with_native_endian()
                        .with_limit(limit),
                )
            }
            (Some(limit), Endian::Native, IntegerEncoding::Variable, true) => {
                Self::LimitedNativeVarintReject(
                    DefaultOptions::default()
                        .reject_trailing_bytes()
                        .with_varint_encoding()
                        .with_native_endian()
                        .with_limit(limit),
                )
            }
            (Some(limit), Endian::Native, IntegerEncoding::Variable, false) => {
                Self::LimitedNativeVarintAllow(
                    DefaultOptions::default()
                        .allow_trailing_bytes()
                        .with_varint_encoding()
                        .with_native_endian()
                        .with_limit(limit),
                )
            }
        }
    }
}

#[allow(clippy::type_complexity)]
enum BincodeOptions {
    UnlimitedLittleVarintReject(
        WithOtherLimit<
            WithOtherEndian<
                WithOtherIntEncoding<
                    WithOtherTrailing<DefaultOptions, RejectTrailing>,
                    VarintEncoding,
                >,
                LittleEndian,
            >,
            Infinite,
        >,
    ),
    UnlimitedLittleVarintAllow(
        WithOtherLimit<
            WithOtherEndian<
                WithOtherIntEncoding<
                    WithOtherTrailing<DefaultOptions, AllowTrailing>,
                    VarintEncoding,
                >,
                LittleEndian,
            >,
            Infinite,
        >,
    ),
    UnlimitedLittleFixintReject(
        WithOtherLimit<
            WithOtherEndian<
                WithOtherIntEncoding<
                    WithOtherTrailing<DefaultOptions, RejectTrailing>,
                    FixintEncoding,
                >,
                LittleEndian,
            >,
            Infinite,
        >,
    ),
    UnlimitedLittleFixintAllow(
        WithOtherLimit<
            WithOtherEndian<
                WithOtherIntEncoding<
                    WithOtherTrailing<DefaultOptions, AllowTrailing>,
                    FixintEncoding,
                >,
                LittleEndian,
            >,
            Infinite,
        >,
    ),
    UnlimitedBigVarintReject(
        WithOtherLimit<
            WithOtherEndian<
                WithOtherIntEncoding<
                    WithOtherTrailing<DefaultOptions, RejectTrailing>,
                    VarintEncoding,
                >,
                BigEndian,
            >,
            Infinite,
        >,
    ),
    UnlimitedBigVarintAllow(
        WithOtherLimit<
            WithOtherEndian<
                WithOtherIntEncoding<
                    WithOtherTrailing<DefaultOptions, AllowTrailing>,
                    VarintEncoding,
                >,
                BigEndian,
            >,
            Infinite,
        >,
    ),
    UnlimitedBigFixintReject(
        WithOtherLimit<
            WithOtherEndian<
                WithOtherIntEncoding<
                    WithOtherTrailing<DefaultOptions, RejectTrailing>,
                    FixintEncoding,
                >,
                BigEndian,
            >,
            Infinite,
        >,
    ),
    UnlimitedBigFixintAllow(
        WithOtherLimit<
            WithOtherEndian<
                WithOtherIntEncoding<
                    WithOtherTrailing<DefaultOptions, AllowTrailing>,
                    FixintEncoding,
                >,
                BigEndian,
            >,
            Infinite,
        >,
    ),
    UnlimitedNativeVarintReject(
        WithOtherLimit<
            WithOtherEndian<
                WithOtherIntEncoding<
                    WithOtherTrailing<DefaultOptions, RejectTrailing>,
                    VarintEncoding,
                >,
                NativeEndian,
            >,
            Infinite,
        >,
    ),
    UnlimitedNativeVarintAllow(
        WithOtherLimit<
            WithOtherEndian<
                WithOtherIntEncoding<
                    WithOtherTrailing<DefaultOptions, AllowTrailing>,
                    VarintEncoding,
                >,
                NativeEndian,
            >,
            Infinite,
        >,
    ),
    UnlimitedNativeFixintReject(
        WithOtherLimit<
            WithOtherEndian<
                WithOtherIntEncoding<
                    WithOtherTrailing<DefaultOptions, RejectTrailing>,
                    FixintEncoding,
                >,
                NativeEndian,
            >,
            Infinite,
        >,
    ),
    UnlimitedNativeFixintAllow(
        WithOtherLimit<
            WithOtherEndian<
                WithOtherIntEncoding<
                    WithOtherTrailing<DefaultOptions, AllowTrailing>,
                    FixintEncoding,
                >,
                NativeEndian,
            >,
            Infinite,
        >,
    ),
    LimitedLittleVarintReject(
        WithOtherLimit<
            WithOtherEndian<
                WithOtherIntEncoding<
                    WithOtherTrailing<DefaultOptions, RejectTrailing>,
                    VarintEncoding,
                >,
                LittleEndian,
            >,
            Bounded,
        >,
    ),
    LimitedLittleVarintAllow(
        WithOtherLimit<
            WithOtherEndian<
                WithOtherIntEncoding<
                    WithOtherTrailing<DefaultOptions, AllowTrailing>,
                    VarintEncoding,
                >,
                LittleEndian,
            >,
            Bounded,
        >,
    ),
    LimitedLittleFixintReject(
        WithOtherLimit<
            WithOtherEndian<
                WithOtherIntEncoding<
                    WithOtherTrailing<DefaultOptions, RejectTrailing>,
                    FixintEncoding,
                >,
                LittleEndian,
            >,
            Bounded,
        >,
    ),
    LimitedLittleFixintAllow(
        WithOtherLimit<
            WithOtherEndian<
                WithOtherIntEncoding<
                    WithOtherTrailing<DefaultOptions, AllowTrailing>,
                    FixintEncoding,
                >,
                LittleEndian,
            >,
            Bounded,
        >,
    ),
    LimitedBigVarintReject(
        WithOtherLimit<
            WithOtherEndian<
                WithOtherIntEncoding<
                    WithOtherTrailing<DefaultOptions, RejectTrailing>,
                    VarintEncoding,
                >,
                BigEndian,
            >,
            Bounded,
        >,
    ),
    LimitedBigVarintAllow(
        WithOtherLimit<
            WithOtherEndian<
                WithOtherIntEncoding<
                    WithOtherTrailing<DefaultOptions, AllowTrailing>,
                    VarintEncoding,
                >,
                BigEndian,
            >,
            Bounded,
        >,
    ),
    LimitedBigFixintReject(
        WithOtherLimit<
            WithOtherEndian<
                WithOtherIntEncoding<
                    WithOtherTrailing<DefaultOptions, RejectTrailing>,
                    FixintEncoding,
                >,
                BigEndian,
            >,
            Bounded,
        >,
    ),
    LimitedBigFixintAllow(
        WithOtherLimit<
            WithOtherEndian<
                WithOtherIntEncoding<
                    WithOtherTrailing<DefaultOptions, AllowTrailing>,
                    FixintEncoding,
                >,
                BigEndian,
            >,
            Bounded,
        >,
    ),
    LimitedNativeVarintReject(
        WithOtherLimit<
            WithOtherEndian<
                WithOtherIntEncoding<
                    WithOtherTrailing<DefaultOptions, RejectTrailing>,
                    VarintEncoding,
                >,
                NativeEndian,
            >,
            Bounded,
        >,
    ),
    LimitedNativeVarintAllow(
        WithOtherLimit<
            WithOtherEndian<
                WithOtherIntEncoding<
                    WithOtherTrailing<DefaultOptions, AllowTrailing>,
                    VarintEncoding,
                >,
                NativeEndian,
            >,
            Bounded,
        >,
    ),
    LimitedNativeFixintReject(
        WithOtherLimit<
            WithOtherEndian<
                WithOtherIntEncoding<
                    WithOtherTrailing<DefaultOptions, RejectTrailing>,
                    FixintEncoding,
                >,
                NativeEndian,
            >,
            Bounded,
        >,
    ),
    LimitedNativeFixintAllow(
        WithOtherLimit<
            WithOtherEndian<
                WithOtherIntEncoding<
                    WithOtherTrailing<DefaultOptions, AllowTrailing>,
                    FixintEncoding,
                >,
                NativeEndian,
            >,
            Bounded,
        >,
    ),
}

impl<T> Format<T> for Bincode
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    type Error = bincode::Error;

    fn serialized_size(&self, value: &T) -> Result<Option<usize>, Self::Error> {
        BincodeOptions::from(self).serialized_size(value)
    }

    fn serialize(&self, value: &T) -> Result<Vec<u8>, Self::Error> {
        BincodeOptions::from(self).serialize(value)
    }

    fn serialize_into<W: Write>(&self, value: &T, writer: W) -> Result<(), Self::Error> {
        BincodeOptions::from(self).serialize_into(value, writer)
    }

    fn deserialize(&self, data: &[u8]) -> Result<T, Self::Error> {
        BincodeOptions::from(self).deserialize(data)
    }

    fn deserialize_from<R: Read>(&self, reader: R) -> Result<T, Self::Error> {
        BincodeOptions::from(self).deserialize_from(reader)
    }
}

impl<T> Format<T> for BincodeOptions
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    type Error = bincode::Error;

    fn serialized_size(&self, value: &T) -> Result<Option<usize>, Self::Error> {
        match self {
            BincodeOptions::UnlimitedLittleVarintReject(options) => options.serialized_size(value),
            BincodeOptions::UnlimitedLittleVarintAllow(options) => options.serialized_size(value),
            BincodeOptions::UnlimitedLittleFixintReject(options) => options.serialized_size(value),
            BincodeOptions::UnlimitedLittleFixintAllow(options) => options.serialized_size(value),
            BincodeOptions::UnlimitedBigVarintReject(options) => options.serialized_size(value),
            BincodeOptions::UnlimitedBigVarintAllow(options) => options.serialized_size(value),
            BincodeOptions::UnlimitedBigFixintReject(options) => options.serialized_size(value),
            BincodeOptions::UnlimitedBigFixintAllow(options) => options.serialized_size(value),
            BincodeOptions::UnlimitedNativeVarintReject(options) => options.serialized_size(value),
            BincodeOptions::UnlimitedNativeVarintAllow(options) => options.serialized_size(value),
            BincodeOptions::UnlimitedNativeFixintReject(options) => options.serialized_size(value),
            BincodeOptions::UnlimitedNativeFixintAllow(options) => options.serialized_size(value),
            BincodeOptions::LimitedLittleVarintReject(options) => options.serialized_size(value),
            BincodeOptions::LimitedLittleVarintAllow(options) => options.serialized_size(value),
            BincodeOptions::LimitedLittleFixintReject(options) => options.serialized_size(value),
            BincodeOptions::LimitedLittleFixintAllow(options) => options.serialized_size(value),
            BincodeOptions::LimitedBigVarintReject(options) => options.serialized_size(value),
            BincodeOptions::LimitedBigVarintAllow(options) => options.serialized_size(value),
            BincodeOptions::LimitedBigFixintReject(options) => options.serialized_size(value),
            BincodeOptions::LimitedBigFixintAllow(options) => options.serialized_size(value),
            BincodeOptions::LimitedNativeVarintReject(options) => options.serialized_size(value),
            BincodeOptions::LimitedNativeVarintAllow(options) => options.serialized_size(value),
            BincodeOptions::LimitedNativeFixintReject(options) => options.serialized_size(value),
            BincodeOptions::LimitedNativeFixintAllow(options) => options.serialized_size(value),
        }
        .and_then(|size| {
            usize::try_from(size)
                .map(Some)
                .map_err(bincode::Error::custom)
        })
    }

    fn serialize(&self, value: &T) -> Result<Vec<u8>, Self::Error> {
        match self {
            BincodeOptions::UnlimitedLittleVarintReject(options) => options.serialize(value),
            BincodeOptions::UnlimitedLittleVarintAllow(options) => options.serialize(value),
            BincodeOptions::UnlimitedLittleFixintReject(options) => options.serialize(value),
            BincodeOptions::UnlimitedLittleFixintAllow(options) => options.serialize(value),
            BincodeOptions::UnlimitedBigVarintReject(options) => options.serialize(value),
            BincodeOptions::UnlimitedBigVarintAllow(options) => options.serialize(value),
            BincodeOptions::UnlimitedBigFixintReject(options) => options.serialize(value),
            BincodeOptions::UnlimitedBigFixintAllow(options) => options.serialize(value),
            BincodeOptions::UnlimitedNativeVarintReject(options) => options.serialize(value),
            BincodeOptions::UnlimitedNativeVarintAllow(options) => options.serialize(value),
            BincodeOptions::UnlimitedNativeFixintReject(options) => options.serialize(value),
            BincodeOptions::UnlimitedNativeFixintAllow(options) => options.serialize(value),
            BincodeOptions::LimitedLittleVarintReject(options) => options.serialize(value),
            BincodeOptions::LimitedLittleVarintAllow(options) => options.serialize(value),
            BincodeOptions::LimitedLittleFixintReject(options) => options.serialize(value),
            BincodeOptions::LimitedLittleFixintAllow(options) => options.serialize(value),
            BincodeOptions::LimitedBigVarintReject(options) => options.serialize(value),
            BincodeOptions::LimitedBigVarintAllow(options) => options.serialize(value),
            BincodeOptions::LimitedBigFixintReject(options) => options.serialize(value),
            BincodeOptions::LimitedBigFixintAllow(options) => options.serialize(value),
            BincodeOptions::LimitedNativeVarintReject(options) => options.serialize(value),
            BincodeOptions::LimitedNativeVarintAllow(options) => options.serialize(value),
            BincodeOptions::LimitedNativeFixintReject(options) => options.serialize(value),
            BincodeOptions::LimitedNativeFixintAllow(options) => options.serialize(value),
        }
    }

    fn serialize_into<W: Write>(&self, value: &T, writer: W) -> Result<(), Self::Error> {
        match self {
            BincodeOptions::UnlimitedLittleVarintReject(options) => {
                options.serialize_into(writer, value)
            }
            BincodeOptions::UnlimitedLittleVarintAllow(options) => {
                options.serialize_into(writer, value)
            }
            BincodeOptions::UnlimitedLittleFixintReject(options) => {
                options.serialize_into(writer, value)
            }
            BincodeOptions::UnlimitedLittleFixintAllow(options) => {
                options.serialize_into(writer, value)
            }
            BincodeOptions::UnlimitedBigVarintReject(options) => {
                options.serialize_into(writer, value)
            }
            BincodeOptions::UnlimitedBigVarintAllow(options) => {
                options.serialize_into(writer, value)
            }
            BincodeOptions::UnlimitedBigFixintReject(options) => {
                options.serialize_into(writer, value)
            }
            BincodeOptions::UnlimitedBigFixintAllow(options) => {
                options.serialize_into(writer, value)
            }
            BincodeOptions::UnlimitedNativeVarintReject(options) => {
                options.serialize_into(writer, value)
            }
            BincodeOptions::UnlimitedNativeVarintAllow(options) => {
                options.serialize_into(writer, value)
            }
            BincodeOptions::UnlimitedNativeFixintReject(options) => {
                options.serialize_into(writer, value)
            }
            BincodeOptions::UnlimitedNativeFixintAllow(options) => {
                options.serialize_into(writer, value)
            }
            BincodeOptions::LimitedLittleVarintReject(options) => {
                options.serialize_into(writer, value)
            }
            BincodeOptions::LimitedLittleVarintAllow(options) => {
                options.serialize_into(writer, value)
            }
            BincodeOptions::LimitedLittleFixintReject(options) => {
                options.serialize_into(writer, value)
            }
            BincodeOptions::LimitedLittleFixintAllow(options) => {
                options.serialize_into(writer, value)
            }
            BincodeOptions::LimitedBigVarintReject(options) => {
                options.serialize_into(writer, value)
            }
            BincodeOptions::LimitedBigVarintAllow(options) => options.serialize_into(writer, value),
            BincodeOptions::LimitedBigFixintReject(options) => {
                options.serialize_into(writer, value)
            }
            BincodeOptions::LimitedBigFixintAllow(options) => options.serialize_into(writer, value),
            BincodeOptions::LimitedNativeVarintReject(options) => {
                options.serialize_into(writer, value)
            }
            BincodeOptions::LimitedNativeVarintAllow(options) => {
                options.serialize_into(writer, value)
            }
            BincodeOptions::LimitedNativeFixintReject(options) => {
                options.serialize_into(writer, value)
            }
            BincodeOptions::LimitedNativeFixintAllow(options) => {
                options.serialize_into(writer, value)
            }
        }
    }

    fn deserialize(&self, data: &[u8]) -> Result<T, Self::Error> {
        match self {
            BincodeOptions::UnlimitedLittleVarintReject(options) => options.deserialize(data),
            BincodeOptions::UnlimitedLittleVarintAllow(options) => options.deserialize(data),
            BincodeOptions::UnlimitedLittleFixintReject(options) => options.deserialize(data),
            BincodeOptions::UnlimitedLittleFixintAllow(options) => options.deserialize(data),
            BincodeOptions::UnlimitedBigVarintReject(options) => options.deserialize(data),
            BincodeOptions::UnlimitedBigVarintAllow(options) => options.deserialize(data),
            BincodeOptions::UnlimitedBigFixintReject(options) => options.deserialize(data),
            BincodeOptions::UnlimitedBigFixintAllow(options) => options.deserialize(data),
            BincodeOptions::UnlimitedNativeVarintReject(options) => options.deserialize(data),
            BincodeOptions::UnlimitedNativeVarintAllow(options) => options.deserialize(data),
            BincodeOptions::UnlimitedNativeFixintReject(options) => options.deserialize(data),
            BincodeOptions::UnlimitedNativeFixintAllow(options) => options.deserialize(data),
            BincodeOptions::LimitedLittleVarintReject(options) => options.deserialize(data),
            BincodeOptions::LimitedLittleVarintAllow(options) => options.deserialize(data),
            BincodeOptions::LimitedLittleFixintReject(options) => options.deserialize(data),
            BincodeOptions::LimitedLittleFixintAllow(options) => options.deserialize(data),
            BincodeOptions::LimitedBigVarintReject(options) => options.deserialize(data),
            BincodeOptions::LimitedBigVarintAllow(options) => options.deserialize(data),
            BincodeOptions::LimitedBigFixintReject(options) => options.deserialize(data),
            BincodeOptions::LimitedBigFixintAllow(options) => options.deserialize(data),
            BincodeOptions::LimitedNativeVarintReject(options) => options.deserialize(data),
            BincodeOptions::LimitedNativeVarintAllow(options) => options.deserialize(data),
            BincodeOptions::LimitedNativeFixintReject(options) => options.deserialize(data),
            BincodeOptions::LimitedNativeFixintAllow(options) => options.deserialize(data),
        }
    }

    fn deserialize_from<R: Read>(&self, reader: R) -> Result<T, Self::Error> {
        match self {
            BincodeOptions::UnlimitedLittleVarintReject(options) => {
                options.deserialize_from(reader)
            }
            BincodeOptions::UnlimitedLittleVarintAllow(options) => options.deserialize_from(reader),
            BincodeOptions::UnlimitedLittleFixintReject(options) => {
                options.deserialize_from(reader)
            }
            BincodeOptions::UnlimitedLittleFixintAllow(options) => options.deserialize_from(reader),
            BincodeOptions::UnlimitedBigVarintReject(options) => options.deserialize_from(reader),
            BincodeOptions::UnlimitedBigVarintAllow(options) => options.deserialize_from(reader),
            BincodeOptions::UnlimitedBigFixintReject(options) => options.deserialize_from(reader),
            BincodeOptions::UnlimitedBigFixintAllow(options) => options.deserialize_from(reader),
            BincodeOptions::UnlimitedNativeVarintReject(options) => {
                options.deserialize_from(reader)
            }
            BincodeOptions::UnlimitedNativeVarintAllow(options) => options.deserialize_from(reader),
            BincodeOptions::UnlimitedNativeFixintReject(options) => {
                options.deserialize_from(reader)
            }
            BincodeOptions::UnlimitedNativeFixintAllow(options) => options.deserialize_from(reader),
            BincodeOptions::LimitedLittleVarintReject(options) => options.deserialize_from(reader),
            BincodeOptions::LimitedLittleVarintAllow(options) => options.deserialize_from(reader),
            BincodeOptions::LimitedLittleFixintReject(options) => options.deserialize_from(reader),
            BincodeOptions::LimitedLittleFixintAllow(options) => options.deserialize_from(reader),
            BincodeOptions::LimitedBigVarintReject(options) => options.deserialize_from(reader),
            BincodeOptions::LimitedBigVarintAllow(options) => options.deserialize_from(reader),
            BincodeOptions::LimitedBigFixintReject(options) => options.deserialize_from(reader),
            BincodeOptions::LimitedBigFixintAllow(options) => options.deserialize_from(reader),
            BincodeOptions::LimitedNativeVarintReject(options) => options.deserialize_from(reader),
            BincodeOptions::LimitedNativeVarintAllow(options) => options.deserialize_from(reader),
            BincodeOptions::LimitedNativeFixintReject(options) => options.deserialize_from(reader),
            BincodeOptions::LimitedNativeFixintAllow(options) => options.deserialize_from(reader),
        }
    }
}

#[test]
#[allow(clippy::too_many_lines)]
fn format_tests() {
    transmog::test_util::test_format(&Bincode::legacy_default());
    transmog::test_util::test_format(
        &Bincode::default()
            .no_limit()
            .little_endian()
            .variable_integer_encoding()
            .reject_trailing_bytes(),
    );
    transmog::test_util::test_format(
        &Bincode::default()
            .no_limit()
            .little_endian()
            .variable_integer_encoding()
            .allow_trailing_bytes(),
    );
    transmog::test_util::test_format(
        &Bincode::default()
            .no_limit()
            .little_endian()
            .fixed_integer_encoding()
            .reject_trailing_bytes(),
    );
    transmog::test_util::test_format(
        &Bincode::default()
            .no_limit()
            .little_endian()
            .fixed_integer_encoding()
            .allow_trailing_bytes(),
    );
    transmog::test_util::test_format(
        &Bincode::default()
            .no_limit()
            .big_endian()
            .variable_integer_encoding()
            .reject_trailing_bytes(),
    );
    transmog::test_util::test_format(
        &Bincode::default()
            .no_limit()
            .big_endian()
            .variable_integer_encoding()
            .allow_trailing_bytes(),
    );
    transmog::test_util::test_format(
        &Bincode::default()
            .no_limit()
            .big_endian()
            .fixed_integer_encoding()
            .reject_trailing_bytes(),
    );
    transmog::test_util::test_format(
        &Bincode::default()
            .no_limit()
            .big_endian()
            .fixed_integer_encoding()
            .allow_trailing_bytes(),
    );
    transmog::test_util::test_format(
        &Bincode::default()
            .no_limit()
            .native_endian()
            .variable_integer_encoding()
            .reject_trailing_bytes(),
    );
    transmog::test_util::test_format(
        &Bincode::default()
            .no_limit()
            .native_endian()
            .variable_integer_encoding()
            .allow_trailing_bytes(),
    );
    transmog::test_util::test_format(
        &Bincode::default()
            .no_limit()
            .native_endian()
            .fixed_integer_encoding()
            .reject_trailing_bytes(),
    );
    transmog::test_util::test_format(
        &Bincode::default()
            .no_limit()
            .native_endian()
            .fixed_integer_encoding()
            .allow_trailing_bytes(),
    );
    transmog::test_util::test_format(
        &Bincode::default()
            .limit(64)
            .little_endian()
            .variable_integer_encoding()
            .reject_trailing_bytes(),
    );
    transmog::test_util::test_format(
        &Bincode::default()
            .limit(64)
            .little_endian()
            .variable_integer_encoding()
            .allow_trailing_bytes(),
    );
    transmog::test_util::test_format(
        &Bincode::default()
            .limit(64)
            .little_endian()
            .fixed_integer_encoding()
            .reject_trailing_bytes(),
    );
    transmog::test_util::test_format(
        &Bincode::default()
            .limit(64)
            .little_endian()
            .fixed_integer_encoding()
            .allow_trailing_bytes(),
    );
    transmog::test_util::test_format(
        &Bincode::default()
            .limit(64)
            .big_endian()
            .variable_integer_encoding()
            .reject_trailing_bytes(),
    );
    transmog::test_util::test_format(
        &Bincode::default()
            .limit(64)
            .big_endian()
            .variable_integer_encoding()
            .allow_trailing_bytes(),
    );
    transmog::test_util::test_format(
        &Bincode::default()
            .limit(64)
            .big_endian()
            .fixed_integer_encoding()
            .reject_trailing_bytes(),
    );
    transmog::test_util::test_format(
        &Bincode::default()
            .limit(64)
            .big_endian()
            .fixed_integer_encoding()
            .allow_trailing_bytes(),
    );
    transmog::test_util::test_format(
        &Bincode::default()
            .limit(64)
            .native_endian()
            .variable_integer_encoding()
            .reject_trailing_bytes(),
    );
    transmog::test_util::test_format(
        &Bincode::default()
            .limit(64)
            .native_endian()
            .variable_integer_encoding()
            .allow_trailing_bytes(),
    );
    transmog::test_util::test_format(
        &Bincode::default()
            .limit(64)
            .native_endian()
            .fixed_integer_encoding()
            .reject_trailing_bytes(),
    );
    transmog::test_util::test_format(
        &Bincode::default()
            .limit(64)
            .native_endian()
            .fixed_integer_encoding()
            .allow_trailing_bytes(),
    );
}
