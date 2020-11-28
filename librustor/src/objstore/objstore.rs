use uuid::Uuid;
use std::io::Error;

pub type ObjectID = Uuid;

/// An ObjectStore is the top-level interface to put, get, or delete stored data
pub trait ObjectStore {
    fn put(&mut self, data: &[u8]) -> Result<ObjectID, Error>;
    fn get(&mut self, uuid: ObjectID) -> Result<Option<Vec<u8>>, Error>;
    fn delete(&mut self, uuid: ObjectID) -> Result<Option<ObjectID>, Error>;
}

