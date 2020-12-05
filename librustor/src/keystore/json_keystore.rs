#![allow(unused_imports)]
use std::fs::{OpenOptions};
use std::io;
use std::path::PathBuf;
use std::collections::HashMap;

use uuid::Uuid;
use serde::{Serialize};
use serde::de::DeserializeOwned;

use log::{trace, debug, info, warn, error};

//use crate::object::ObjKey;
use crate::keystore::KeyStore;
use crate::RResult;

// to be re-constructable from JSON, this HashMap must contain objects, not references (T, not &T)
type Index<T> = HashMap<Uuid, T>;

#[derive(Debug)]
pub struct JsonKeystore<T: Serialize + DeserializeOwned> {
    keystore: Index<T>,
    path: PathBuf,
}

impl<'a, T> JsonKeystore<T> where T: Serialize + DeserializeOwned {
    pub fn new(path: PathBuf) -> Self {
        let mut s = Self {
            keystore: Index::new(),
            path: path
        };

        s.keystore = match s.read_index() {
            Ok(index) => index,
            _ => Index::new()
        };
        s
    }

    fn read_index(&self) -> io::Result<Index<T>> {
        trace!("Opening index at {}", self.path.to_str().unwrap());
        let indexfile = OpenOptions::new()
            .read(true)
            .open(self.path.as_path())?;

        trace!("Reading index from file");
        let v: Index<T> = serde_json::from_reader(indexfile)?;
        Ok(v)
    }

    fn write_index(&self) -> RResult<()> {
        trace!("Writing index at {}", self.path.to_str().unwrap());
        let idxfile = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(self.path.as_path())
            .unwrap();
        serde_json::to_writer_pretty(idxfile, &self.keystore)?;
        Ok(())
    }

    pub fn get_objects(&self) -> &Index<T> {
        &self.keystore
    }
}

impl<T> Default for JsonKeystore<T> where T: Serialize + DeserializeOwned {
    fn default() -> Self {
        Self::new(PathBuf::from("data.json"))
    }
}

use std::fmt;
impl<T> KeyStore<T> for JsonKeystore<T> where T: Serialize + DeserializeOwned + fmt::Debug {
    fn set<'a>(&mut self, uuid: Uuid, key: T) -> RResult<Option<T>>
    {
        trace!("uuid: {:?}, key: {:?}", &uuid, &key);
        let resp = self.keystore.insert(uuid, key);
        self.write_index()?;
        Ok(resp)
    }
    fn get(&self, uuid: &Uuid) -> RResult<Option<&T>> {
        let resp = self.keystore.get(uuid);
        //self.write_index().unwrap();
        Ok(resp)
    }
    fn delete(&mut self, uuid: &Uuid) -> RResult<Option<T>> {
        let key = self.keystore.remove(uuid);
        self.write_index()?;
        Ok(key)
    }
    /*
    fn mset(&self, objects: &HashMap<Uuid, T>) -> io::Result<HashMap<Uuid, io::Result<()>>> {
        let mut results: HashMap<Uuid, io::Result<()>> = HashMap::new();
        // TODO: write JsonKeystore.mset
        Ok(results)
    }
    fn mget(&self, uuids: Vec<Uuid>) -> io::Result<HashMap<Uuid, io::Result<T>>> {
        let mut results: HashMap<Uuid, io::Result<T>> = HashMap::new();
        // TODO: write JsonKeystore.mget
        Ok(results)
    }
    fn mdelete(&self, uuid: Uuid) -> io::Result<HashMap<Uuid, io::Result<()>>> {
        let mut results: HashMap<Uuid, io::Result<()>> = HashMap::new();
        // TODO: write JsonKeystore.mdelete
        Ok(results)
    }
    */
}

