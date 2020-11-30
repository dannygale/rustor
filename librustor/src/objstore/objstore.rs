use uuid::Uuid;
use std::io::Error;

use crate::blockstore::blockstore::{ BlockStore };//, BlockDevice };
use crate::object::ObjKey;
use crate::keystore::KeyStore;
use crate::keygen::{KeyGen, GeneratesKeys};
use crate::freelist::{ FreeList}; //, VecFreeList };

pub type ObjectID = Uuid;

/// An ObjectStore is the top-level interface to put, get, or delete stored data
pub trait StoresObjects {
    fn put(&mut self, data: &[u8]) -> Result<ObjectID, Error>;
    fn get(&mut self, uuid: ObjectID) -> Result<Option<Vec<u8>>, Error>;
    fn delete(&mut self, uuid: ObjectID) -> Result<Option<ObjectID>, Error>;
}

pub struct ObjectStore<'a> {
    blockstore: &'a dyn BlockStore,
    freelist: &'a dyn FreeList,
    keygen: KeyGen,
    keystore: &'a dyn KeyStore<ObjKey>,
}

impl StoresObjects for ObjectStore<'_> {
    fn put(&mut self, data: &[u8]) -> Result<ObjectID, Error> {
        let mut key = self.keygen.make_key(data);
        key.manifest = self.freelist.allocate(key.size)?;
        self.blockstore.write(data, &key);
        self.keystore.set(key.uuid, key);

        Ok(key.uuid)
    }
    fn get(&mut self, uuid: ObjectID) -> Result<Option<Vec<u8>>, Error> {
        if let Some(key) = self.keystore.get(&uuid) {
            let data = self.blockstore.read(&key);
        } else {
            return Ok(None);
        }

        return Ok(None);
    }
    fn delete(&mut self, uuid: ObjectID) -> Result<Option<ObjectID>, Error> {

    }
}

