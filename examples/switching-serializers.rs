//! Demonstrates using dataversion to migrate from unversioned data in one
//! format to versioned data in another format.

use std::{fmt::Debug, io::Write};

use dataversion::Versioned;
use serde::{Deserialize, Serialize};

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
    type Error = dataversion::UnknownVersion;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Legacy),
            1 => Ok(Self::Current),
            other => Err(dataversion::UnknownVersion(other)),
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
        // This example uses the more efficient Write-based API. See
        // `versioned-serde.rs` for an example that uses the simpler `wrap()`
        // API.
        dataversion::encode(Versions::Current, &mut serialized, |write| {
            self.serialize_into(write)
        })?;

        Ok(serialized)
    }

    fn serialize_into<W: Write + Debug>(&self, write: &mut W) -> Result<(), pot::Error> {
        let mut serializer = pot::ser::Serializer::new(write)?;
        self.serialize(&mut serializer)?;
        Ok(())
    }

    fn deserialize_versioned(
        version: u64,
        data: &[u8],
    ) -> Result<Self, dataversion::Error<SerializerErrors>> {
        match Versions::try_from(version)? {
            Versions::Legacy => bincode::deserialize(data).map_err(SerializerErrors::Bincode),
            Versions::Current => pot::from_slice(data).map_err(SerializerErrors::Pot),
        }
        .map_err(dataversion::Error::Other)
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
        dataversion::decode(&originally_stored_data, User::deserialize_versioned)?;
    assert_eq!(original_user, deserialized_user);

    // And, when we write out our new version, it will be wrapped by
    // `dataversion` with the current version information.
    let new_data = deserialized_user.to_vec()?;
    let deserialized_user = dataversion::decode(&new_data, User::deserialize_versioned)?;
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
