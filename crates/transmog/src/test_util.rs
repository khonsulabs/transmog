use std::io::ErrorKind;

use crate::Format;

/// Perform a simple set of tests on `format`. These tests are not meant to be
/// exhaustive, and only test the basic functionality of Format. The soundness
/// of each format should be tested within their respective crates.
#[allow(clippy::missing_panics_doc)]
pub fn test_format<F: Format<u64> + Clone>(format: &F) {
    let serialized_to_vec = format.serialize(&1_u64).unwrap();
    if let Some(expected_size) = format.serialized_size(&1_u64).unwrap() {
        assert_eq!(serialized_to_vec.len(), expected_size);
    }

    let deserialized_from_reader: u64 = format.deserialize_from(&serialized_to_vec[..]).unwrap();
    assert_eq!(deserialized_from_reader, 1);

    let mut serialized_to_writer = Vec::new();
    format
        .serialize_into(&2_u64, &mut serialized_to_writer)
        .unwrap();
    let deserialized_from_slice: u64 = format.deserialize(&serialized_to_writer).unwrap();
    assert_eq!(deserialized_from_slice, 2);

    // Test error conversion
    println!(
        "Converted io error: {0}",
        <F::Error as From<std::io::Error>>::from(std::io::Error::from(ErrorKind::UnexpectedEof)),
    );

    // Test the cloned format
    let format = format.clone();
    let deserialized_from_cloned: u64 = format.deserialize(&serialized_to_writer).unwrap();
    assert_eq!(deserialized_from_cloned, 2);

    let serialized_from_cloned = format.serialize(&2).unwrap();
    assert_eq!(serialized_from_cloned, serialized_to_writer);
}
