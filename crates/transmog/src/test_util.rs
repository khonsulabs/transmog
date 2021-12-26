#[macro_export]
macro_rules! define_format_test_suite {
    ($format:expr) => {
        #[cfg(test)]
        mod format_tests {
            use super::*;
            use $crate::Format;

            #[test]
            fn basic() {
                let format = $format;
                let serialized_to_vec = format.serialize(&1_u64).unwrap();
                let deserialized_from_reader: u64 =
                    format.deserialize_from(&serialized_to_vec[..]).unwrap();
                assert_eq!(deserialized_from_reader, 1);

                let mut serialized_to_writer = Vec::new();
                format
                    .serialize_into(&2_u64, &mut serialized_to_writer)
                    .unwrap();
                let deserialized_from_slice: u64 =
                    format.deserialize(&serialized_to_writer).unwrap();
                assert_eq!(deserialized_from_slice, 2);
            }
        }
    };
}
