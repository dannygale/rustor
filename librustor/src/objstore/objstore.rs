use uuid::Uuid;

use crate::RResult;
use crate::blockstore::blockstore::{ BlockStore };//, BlockDevice };
use crate::object::ObjKey;
use crate::keystore::KeyStore;
use crate::keygen::{KeyGen, GeneratesKeys};
use crate::freelist::{ FreeList}; //, VecFreeList };

#[allow(unused_imports)]
use log::{trace, debug, info, warn, error};

pub type ObjectID = Uuid;

/// An ObjectStore is the top-level interface to put, get, or delete stored data
pub trait ObjectStore {
    fn put(&mut self, data: &[u8]) -> RResult<ObjectID>;
    fn get(&mut self, uuid: ObjectID) -> RResult<Option<Vec<u8>>>;
    fn delete(&mut self, uuid: ObjectID) -> RResult<Option<ObjectID>>;
}

pub struct BasicObjectStore<'a> {
    blockstore: &'a mut dyn BlockStore,
    freelist: &'a mut dyn FreeList,
    keygen: KeyGen,
    keystore: &'a mut dyn KeyStore<ObjKey>,
}


impl<'a> BasicObjectStore<'a> {
    pub fn new(
        blockstore: &'a mut dyn BlockStore, 
        freelist: &'a mut dyn FreeList,
        keygen: KeyGen,
        keystore: &'a mut dyn KeyStore<ObjKey>
        ) -> Self {
        Self { blockstore, freelist, keygen, keystore }
    }
}

impl ObjectStore for BasicObjectStore<'_> {
    fn put(&mut self, data: &[u8]) -> RResult<ObjectID> {
        let mut key = self.keygen.make_key(data)?;
        key.manifest = self.freelist.allocate(key.size)?;
        self.blockstore.write(data, &key)?;
        let uuid = key.uuid.clone();
        self.keystore.set(key.uuid, key)?;

        Ok(uuid)
    }

    fn get(&mut self, uuid: ObjectID) -> RResult<Option<Vec<u8>>> {
        trace!("get {:?}", &uuid);
        if let Some(key) = self.keystore.get(&uuid)? {
            trace!("found key: {:?}", &key);
            let mut data = Vec::with_capacity(key.size as usize);
            self.blockstore.read(&mut data, &key)?;
            return Ok(Some(data));
        } else {
            return Ok(None);
        }
    }

    #[allow(unused_variables)]
    fn delete(&mut self, uuid: ObjectID) -> RResult<Option<ObjectID>> {
        if let Some(key) = self.keystore.get(&uuid)? {
            self.freelist.release(&key.manifest);
            return Ok(Some(uuid));
        } else {
            return Ok(None);
        }
    }
}

