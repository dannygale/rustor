use uuid::Uuid;
use std::hash::Hasher;
use std::collections::hash_map::DefaultHasher;

use crate::object::ObjKey;

pub trait GeneratesKeys {
    fn make_key(&self, data: &[u8]) -> ObjKey;
}

pub struct KeyGen {}

impl GeneratesKeys for KeyGen {
    fn make_key(&self, data: &[u8]) -> ObjKey {
        let mut hasher = DefaultHasher::new();
        hasher.write(data);
        let hash = hasher.finish();

        ObjKey {
            uuid: Uuid::new_v5(&Uuid::NAMESPACE_OID, data),
            hash: hash,
            size: data.len() as u64
        }
    }
}

