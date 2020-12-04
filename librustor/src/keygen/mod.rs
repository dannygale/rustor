use uuid::Uuid;
use std::hash::Hasher;
use std::collections::hash_map::DefaultHasher;

use crate::object::{ObjKey, Manifest};
use crate::RResult;

pub trait GeneratesKeys {
    fn make_key(&self, data: &[u8]) -> RResult<ObjKey>;
}

pub struct KeyGen {}

impl GeneratesKeys for KeyGen {
    fn make_key(&self, data: &[u8]) -> RResult<ObjKey> {
        let mut hasher = DefaultHasher::new();
        hasher.write(data);
        let hash = hasher.finish();

        Ok(ObjKey {
            manifest: Manifest::default(),
            uuid: Uuid::new_v5(&Uuid::NAMESPACE_OID, data),
            hash: hash,
            size: data.len() as u64
        })
    }
}

