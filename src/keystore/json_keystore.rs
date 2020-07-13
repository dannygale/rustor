use std::fs::{OpenOptions};
use std::io;
use std::path::PathBuf;
use std::collections::HashMap;

use uuid::Uuid;

use crate::object::{IsObject, Object, ObjKey};
use crate::keystore::KeyStore;

type Index = HashMap<Uuid, ObjKey>;

#[derive(Debug)]
pub struct JsonKeystore {
    keystore: Index,
    path: PathBuf,
}

impl JsonKeystore {
    pub fn new(path: PathBuf) -> Self {
        Self {
            keystore: HashMap::new(),
            path: path
        }
    }

    fn read_index(&self) -> io::Result<Index> {
        println!("Opening index at {}", self.path.to_str().unwrap());
        let mut f = OpenOptions::new().create(true);
        let mut indexfile = OpenOptions::new()
            .read(true)
            .open(self.path.as_path())
            .unwrap();

        println!("Reading index from file");
        let v = serde_json::from_reader(indexfile)?;
        Ok(v)
    }

    fn write_index(&self, idx: &Index) -> io::Result<()> {
        println!("Writing index at {}", self.path.to_str().unwrap());
        let mut idxfile = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(self.path.as_path())
            .unwrap();
        serde_json::to_writer_pretty(idxfile, idx);
        Ok(())
    }
}

impl Default for JsonKeystore {
    fn default() -> Self {
        Self {
            keystore: Index::new(),
            path: PathBuf::from("data.json")
        }
    }
}

impl KeyStore for JsonKeystore {
    fn set(&self, uuid: Uuid, key: ObjKey) -> io::Result<()> {
        let mut i = self.read_index()?;
        i.insert(uuid, key);
        self.write_index(&i)
    }
    fn get(&self, uuid: &Uuid) -> io::Result<Option<&ObjKey>> {
        let mut i = self.read_index()?;
        Ok(i.get(uuid))
    }
    fn delete(&mut self, uuid: &Uuid) -> io::Result<()> {
        let mut i = self.read_index()?;
        i.remove(uuid);
        self.write_index(&i)
    }
    fn mset(&self, objects: &HashMap<Uuid, ObjKey>) -> io::Result<HashMap<Uuid, io::Result<()>>> {
        let mut results: HashMap<Uuid, io::Result<()>> = HashMap::new();
        // TODO: write JsonKeystore.mset
        Ok(results)
    }
    fn mget(&self, uuids: Vec<Uuid>) -> io::Result<HashMap<Uuid, io::Result<ObjKey>>> {
        let mut results: HashMap<Uuid, io::Result<ObjKey>> = HashMap::new();
        // TODO: write JsonKeystore.mget
        Ok(results)
    }
    fn mdelete(&mut self, uuid: Uuid) -> io::Result<HashMap<Uuid, io::Result<()>>> {
        let mut results: HashMap<Uuid, io::Result<()>> = HashMap::new();
        // TODO: write JsonKeystore.mdelete
        Ok(results)
    }
}
