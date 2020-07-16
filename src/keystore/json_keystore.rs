use std::fs::{OpenOptions};
use std::io;
use std::path::PathBuf;
use std::collections::HashMap;

use uuid::Uuid;
use serde::{Serialize};
use serde::de::DeserializeOwned;

//use crate::object::ObjKey;
use crate::keystore::KeyStore;

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
            keystore: HashMap::new(),
            path: path
        };

        s.keystore = s.read_index().unwrap();
        s
    }

    fn read_index(&self) -> io::Result<Index<T>> {
        println!("Opening index at {}", self.path.to_str().unwrap());
        let indexfile = OpenOptions::new()
            .read(true)
            .open(self.path.as_path())
            .unwrap();

        println!("Reading index from file");
        let v: Index<T> = serde_json::from_reader(indexfile)?;
        Ok(v)
    }

    fn write_index(&self) -> io::Result<()> {
        println!("Writing index at {}", self.path.to_str().unwrap());
        let idxfile = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(self.path.as_path())
            .unwrap();
        serde_json::to_writer_pretty(idxfile, &self.keystore)?;
        Ok(())
    }
}

impl<T> Default for JsonKeystore<T> where T: Serialize + DeserializeOwned {
    fn default() -> Self {
        Self {
            keystore: Index::new(),
            path: PathBuf::from("data.json")
        }
    }
}

impl<T> KeyStore<T> for JsonKeystore<T> where T: Serialize + DeserializeOwned {
    fn set<'a>(&mut self, uuid: Uuid, key: T) -> Option<T>
    {
        self.keystore.insert(uuid, key)
    }
    fn get(&self, uuid: &Uuid) -> Option<&T> {
        self.keystore.get(uuid)
    }
    fn delete(&mut self, uuid: &Uuid) -> io::Result<Option<T>> {
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
