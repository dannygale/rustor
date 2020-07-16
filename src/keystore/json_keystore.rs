use std::fs::{OpenOptions};
use std::io;
use std::path::PathBuf;
use std::collections::HashMap;

use uuid::Uuid;
use serde::{Serialize};
use serde::de::DeserializeOwned;

//use crate::object::ObjKey;
use crate::keystore::KeyStore;

type Index<T> where T: Serialize + DeserializeOwned = HashMap<Uuid, T>;

#[derive(Debug)]
pub struct JsonKeystore<T: Serialize + DeserializeOwned> {
    keystore: Index<T>,
    path: PathBuf,
}

impl<T> JsonKeystore<T> where T: Serialize + DeserializeOwned {
    pub fn new(path: PathBuf) -> Self {
        Self {
            keystore: HashMap::new(),
            path: path
        }
    }

    fn read_index(&self) -> io::Result<Index<T>> {
        println!("Opening index at {}", self.path.to_str().unwrap());
        let indexfile = OpenOptions::new()
            .read(true)
            .open(self.path.as_path())
            .unwrap();

        println!("Reading index from file");
        let v = serde_json::from_reader(indexfile)?;
        Ok(v)
    }

    fn write_index(&self, idx: &Index<T>) -> io::Result<()> {
        println!("Writing index at {}", self.path.to_str().unwrap());
        let idxfile = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(self.path.as_path())
            .unwrap();
        serde_json::to_writer_pretty(idxfile, idx);
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
    fn set<'a>(&self, uuid: Uuid, key: &'a T) -> io::Result<()> 
    {
        let mut i = self.read_index()?;
        i.insert(uuid, *key);
        self.write_index(&i)
    }
    fn get(&self, uuid: &Uuid) -> io::Result<T> {
        let i = self.read_index()?;
        let key = i.get(uuid).unwrap();
        Ok(*key)
    }
    fn delete(&self, uuid: &Uuid) -> io::Result<Option<T>> {
        let mut i = self.read_index()?;
        let key = i.remove(uuid);
        self.write_index(&i);
        Ok(key)
    }
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
}
