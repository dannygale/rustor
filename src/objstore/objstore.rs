use uuid::Uuid;
use std::io::Error;

pub type ObjectID = u64;

pub trait ObjectStore {
    fn put(&mut self, data: &[u8]) -> Result<ObjectID, Error>;
    fn get(&mut self, uuid: ObjectID) -> Result<Option<Vec<u8>>, Error>;
    fn delete(&mut self, uuid: ObjectID) -> Result<(), Error>;
}

