//! WIP. Please ignore.
//!
//! This file is just a playground exploring how traits might work that can power a proc macro.
//!
//! ```rust,ignore
//! #[derive(Versions)]
//! #[versions(strategy = Pot)]
//! enum Versions {
//!     #[versions(strategy = Bincode)]
//!     V0(UserV0),
//!     // implied strategy Pot
//!     V1(UserV1),
//!     // implied strategy Pot
//!     Current(User),
//! }
//! ```

use std::{
    fmt::Debug,
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};
use transmog::{Format, OwnedDeserializer};
use transmog_bincode::{bincode, Bincode};
use transmog_pot::{pot, Pot};
use transmog_versions::UnknownVersion;

#[derive(Debug)]
struct Versions;

/// Our first version of the User structure.
#[derive(Serialize, Deserialize, Default, Eq, PartialEq, Debug)]
pub struct UserV0 {
    id: u8,
    first_name: String,
    last_name: String,
}

impl From<UserV0> for User {
    fn from(legacy: UserV0) -> Self {
        Self {
            id: u32::from(legacy.id),
            name: format!("{} {}", legacy.first_name, legacy.last_name),
        }
    }
}

// Pair of types: Serializer, Type
// A list of these pairs = a sequence of structure versions
// To deserialize

// impl Versioned for Versions {
//     fn version(&self) -> u64 {
//         match self {
//             Versions::V0(_) => 0,
//             Versions::V1(_) => 1,
//             Versions::Current(_) => 2,
//         }
//     }
// }

impl Format<'static, User> for Versions {
    type Error = transmog_versions::Error<SerializerErrors>;

    fn serialize(&self, value: &User) -> Result<Vec<u8>, Self::Error> {
        Pot::default()
            .serialize(value)
            .map(|data| transmog_versions::wrap(&2, data))
            .map_err(SerializerErrors::from)
            .map_err(transmog_versions::Error::Format)
    }

    fn serialize_into<W: Write>(&self, value: &User, mut writer: W) -> Result<(), Self::Error> {
        transmog_versions::write_header(&2, &mut writer)?;
        Pot::default()
            .serialize_into(value, writer)
            .map_err(SerializerErrors::from)
            .map_err(transmog_versions::Error::Format)
    }
}

impl OwnedDeserializer<User> for Versions {
    fn deserialize_owned(&self, data: &[u8]) -> Result<User, Self::Error> {
        let (version, data) = transmog_versions::unwrap_version(data);
        match version {
            0 => <Bincode as OwnedDeserializer<UserV0>>::deserialize_owned(
                &Bincode::legacy_default(),
                data,
            )
            .map(User::from)
            .map_err(SerializerErrors::from)
            .map_err(transmog_versions::Error::Format),
            1 => <Pot as OwnedDeserializer<UserV0>>::deserialize_owned(&Pot::default(), data)
                .map(User::from)
                .map_err(SerializerErrors::from)
                .map_err(transmog_versions::Error::Format),
            2 => <Pot as OwnedDeserializer<User>>::deserialize_owned(&Pot::default(), data)
                .map(User::from)
                .map_err(SerializerErrors::from)
                .map_err(transmog_versions::Error::Format),
            other => Err(transmog_versions::Error::UnknownVersion(UnknownVersion(
                other,
            ))),
        }
    }

    fn deserialize_from<R: Read>(&self, reader: R) -> Result<User, Self::Error> {
        transmog_versions::decode(reader, |version, reader| match version {
            0 => <Bincode as OwnedDeserializer<UserV0>>::deserialize_from(
                &Bincode::legacy_default(),
                reader,
            )
            .map(User::from)
            .map_err(SerializerErrors::from)
            .map_err(transmog_versions::Error::Format),
            1 => <Pot as OwnedDeserializer<UserV0>>::deserialize_from(&Pot::default(), reader)
                .map(User::from)
                .map_err(SerializerErrors::from)
                .map_err(transmog_versions::Error::Format),
            2 => <Pot as OwnedDeserializer<User>>::deserialize_from(&Pot::default(), reader)
                .map(User::from)
                .map_err(SerializerErrors::from)
                .map_err(transmog_versions::Error::Format),
            other => Err(transmog_versions::Error::UnknownVersion(UnknownVersion(
                other,
            ))),
        })
    }
}

#[derive(Serialize, Deserialize, Default, Eq, PartialEq, Debug)]
pub struct User {
    id: u32,
    name: String,
}

fn main() -> anyhow::Result<()> {
    let original_user = UserV0 {
        id: 42,
        first_name: String::from("Jane"),
        last_name: String::from("Smith"),
    };
    let current_user = User {
        id: 42,
        name: String::from("Jane Smith"),
    };
    // To simulate the encoding
    let v0_data = bincode::serialize(&original_user)?;
    // Then we adopted transmog, still using the V0 structure, but using transmog to wrap it
    let v1_data = transmog_versions::wrap(&1, pot::to_vec(&original_user)?);
    // And then we updated the structure to a new version
    let current_data = Versions.serialize(&current_user)?;

    // If we pass the bincode-encoded file into
    assert_eq!(current_user, Versions.deserialize_owned(&v0_data)?);
    assert_eq!(current_user, Versions.deserialize_owned(&v1_data)?);
    assert_eq!(current_user, Versions.deserialize_owned(&current_data)?);

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum SerializerErrors {
    #[error("pot error: {0}")]
    Pot(#[from] pot::Error),
    #[error("bincode error: {0}")]
    Bincode(#[from] bincode::Error),
}

#[test]
fn runs() {
    main().unwrap();
}
