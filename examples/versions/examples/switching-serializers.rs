//! Demonstrates using transmog to migrate from unversioned data in one
//! format to versioned data in another format.

use std::{
    fmt::Debug,
    io::{BufReader, Read},
};

use serde::{Deserialize, Serialize};
use transmog_bincode::bincode;
use transmog_pot::pot;
use transmog_versions::{self, UnknownVersion, Versioned};

#[derive(Copy, Clone, Debug)]
enum Versions {
    Legacy = 0,
    Current = 1,
}

impl Versioned for Versions {
    fn version(&self) -> u64 {
        *self as u64
    }
}

impl TryFrom<u64> for Versions {
    type Error = UnknownVersion;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Legacy),
            1 => Ok(Self::Current),
            other => Err(UnknownVersion(other)),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Eq, PartialEq, Debug)]
pub struct User {
    id: u32,
    name: String,
}

impl User {
    /// Converts the user to bytes suitable for storage.
    pub fn to_vec(&self) -> Result<Vec<u8>, pot::Error> {
        let mut serialized = Vec::new();
        // This example uses Write to build the payload without extra data
        // copying, but it's a little more verbose. To see a simpler API, refer
        // to `wrap()` and see its usage in `versioned-serde.rs`.
        transmog_versions::write_header(&Versions::Current, &mut serialized)?;
        pot::to_writer(self, &mut serialized)?;

        Ok(serialized)
    }

    fn deserialize_versioned<R: Read>(
        version: u64,
        data: BufReader<R>,
    ) -> Result<Self, transmog_versions::Error<SerializerErrors>> {
        match Versions::try_from(version)? {
            Versions::Legacy => bincode::deserialize_from(data).map_err(SerializerErrors::Bincode),
            Versions::Current => pot::from_reader(data).map_err(SerializerErrors::Pot),
        }
        .map_err(transmog_versions::Error::Format)
    }
}

fn main() -> anyhow::Result<()> {
    let original_user = User {
        id: 42,
        name: String::from("ecton"),
    };
    // To simulate loading a file that was previously stored in some arbitrary
    // format, we're starting with a plain-encoded bincode user record.
    let originally_stored_data = bincode::serialize(&original_user)?;

    // If we pass the bincode-encoded file into
    let deserialized_user =
        transmog_versions::decode(&originally_stored_data[..], User::deserialize_versioned)?;
    assert_eq!(original_user, deserialized_user);

    // And, when we write out our new version, it will be wrapped by
    // `transmog` with the current version information.
    let new_data = deserialized_user.to_vec()?;
    let deserialized_user = transmog_versions::decode(&new_data[..], User::deserialize_versioned)?;
    assert_eq!(original_user, deserialized_user);

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
