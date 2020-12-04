use uuid::Uuid;

use serde::{Serialize};
use serde::de::DeserializeOwned;

//use crate::object::ObjKey;

use crate::RResult;
pub trait KeyStore<T: Serialize + DeserializeOwned> {
    fn set(&mut self, uuid: Uuid, object: T) -> RResult<Option<T>>;
    fn get(&self, uuid: &Uuid) -> RResult<Option<&T>>;
    fn delete(&mut self, uuid: &Uuid) -> RResult<Option<T>>;

    //fn mset(&self, objects: &HashMap<Uuid, T>) -> io::Result<HashMap<Uuid, io::Result<()>>>;
    //fn mget(&self, uuids: Vec<Uuid>) -> io::Result<HashMap<Uuid, io::Result<T>>>;
    //fn mdelete(&self, uuid: Uuid) -> io::Result<HashMap<Uuid, io::Result<()>>>;
}

/*
pub trait GenericKeyStore {
    fn set<T>(&self, uuid: Uuid, object: T) -> io::Result<()>;
    fn get<T>(&self, uuid: &Uuid) -> io::Result<Option<&T>>;
    fn delete(&self, uuid: &Uuid) -> io::Result<()>;

    fn mset<T>(&self, objects: &HashMap<Uuid, T>) -> io::Result<HashMap<Uuid, io::Result<()>>>;
    fn mget<T>(&self, uuids: Vec<Uuid>) -> io::Result<HashMap<Uuid, io::Result<T>>>;
    fn mdelete(&self, uuid: Uuid) -> io::Result<HashMap<Uuid, io::Result<()>>>;
}
*/


