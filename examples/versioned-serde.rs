//! Demonstrates using transmog to migrate between old versions of data
//! structures to a current version.

use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use transmog::version::{self, ConstVersioned, UnknownVersion};

trait Serializable: Serialize + Sized + ConstVersioned {
    fn to_vec(&self) -> Result<Vec<u8>, pot::Error>;
}

impl<T> Serializable for T
where
    T: Serialize + Sized + ConstVersioned,
{
    fn to_vec(&self) -> Result<Vec<u8>, pot::Error> {
        // This example uses the simpler "wrap" API, which wraps an existing
        // Vec<u8> of data. It requires an extra copy of data, which can be
        // avoided when using the encode() API. For an example of that API, see
        // `switching-serializers.rs`.
        Ok(version::wrap(self, pot::to_vec(self)?))
    }
}

/// Our first version of the User structure.
#[derive(Serialize, Deserialize, Default, Eq, PartialEq, Debug)]
pub struct UserV0 {
    id: u8,
    first_name: String,
    last_name: String,
}

impl ConstVersioned for UserV0 {
    const VERSION: u64 = 0;
}

/// Our current version of the User structure.
#[derive(Serialize, Deserialize, Default, Eq, PartialEq, Debug)]
pub struct User {
    id: u32,
    name: String,
}

impl ConstVersioned for User {
    const VERSION: u64 = 1;
}

impl User {
    fn deserialize(data: &[u8]) -> Result<Self, version::Error<pot::Error>> {
        let (version, data) = version::unwrap_version(data);
        match version {
            UserV0::VERSION => pot::from_slice::<UserV0>(data).map(Self::from),
            Self::VERSION => pot::from_slice(data),
            other => return Err(version::Error::UnknownVersion(UnknownVersion(other))),
        }
        .map_err(version::Error::Other)
    }
}

impl From<UserV0> for User {
    fn from(legacy: UserV0) -> Self {
        Self {
            id: u32::from(legacy.id),
            name: format!("{} {}", legacy.first_name, legacy.last_name),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let original_user = UserV0 {
        id: 42,
        first_name: String::from("Jane"),
        last_name: String::from("Smith"),
    };

    // Serialize the original version of the user record.
    let originally_stored_data = original_user.to_vec()?;

    // Decode the user, getting the new version as part of the process.
    let upgraded_user = User::deserialize(&originally_stored_data).unwrap();
    assert_eq!(User::from(original_user), upgraded_user);

    Ok(())
}

#[test]
fn runs() {
    main().unwrap();
}
