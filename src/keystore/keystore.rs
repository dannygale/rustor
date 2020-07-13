use std::io;
use std::collections::HashMap;
//use std::error::Error;
//use std::default::Default;
use uuid::Uuid;

use crate::object::{Object, ObjKey};

pub trait KeyStore {
    fn set(&self, uuid: Uuid, object: ObjKey) -> io::Result<()>;
    fn get(&self, uuid: &Uuid) -> io::Result<Option<&ObjKey>>;
    fn delete(&mut self, uuid: &Uuid) -> io::Result<()>;

    fn mset(&self, objects: &HashMap<Uuid, ObjKey>) -> io::Result<HashMap<Uuid, io::Result<()>>>;
    fn mget(&self, uuids: Vec<Uuid>) -> io::Result<HashMap<Uuid, io::Result<ObjKey>>>;
    fn mdelete(&mut self, uuid: Uuid) -> io::Result<HashMap<Uuid, io::Result<()>>>;
}

