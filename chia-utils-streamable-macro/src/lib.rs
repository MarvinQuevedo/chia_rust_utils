pub mod bytes_utils;
pub mod sized_bytes;

#[macro_export]
macro_rules! chia_streamable {
    ($class:ident { $($field:ident : $field_ty:ty),* }) => {
        use chia_utils_streamable_macro::bytes_utils::bytes_to_sha256;
        impl $class {
            fn to_bytes(&self) -> Vec<u8> {
                let mut bytes = Vec::new();
                $(
                    bytes.extend_from_slice(&self.$field.to_sized_bytes());
                )*
                bytes
            }

            fn from_bytes(data: Vec<u8>) -> Option<$class> {
                let total_size: usize = 0 $(+ <$field_ty as SizedBytes>::SIZE)*;
                if data.len() != total_size {
                    return None;
                }

                let mut offset = 0;
                $(
                    let $field: $field_ty = match <$field_ty as SizedBytes>::from_sized_bytes(&data[offset..offset + <$field_ty as SizedBytes>::SIZE]) {
                        Some(val) => val,
                        None => return None,
                    };
                    offset += <$field_ty as SizedBytes>::SIZE;
                )*

                Some($class {
                    $(
                        $field,
                    )*
                })
            }

            fn name(&self) -> Bytes32  {
                let bytes: Vec<u8> = self.to_bytes();
                let hash_bytes =  bytes_to_sha256(bytes);
                Bytes32::from(hash_bytes)

            }
        }
    };
}
