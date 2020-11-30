#![allow(unused_imports)]
use std::path::{PathBuf};
use std::fs::{OpenOptions};
use std::io::{Seek, SeekFrom, Read, Write, Error};

// for data hashing
use std::hash::Hasher;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;

//use serde_json;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

//use crate::keystore::json_keystore::JsonKeystore;
use crate::objstore::StoresObjects;
use crate::object::{ObjKey};
use crate::keystore::keystore::KeyStore;
use crate::keystore::JsonKeystore;

use log::{trace, debug, info, warn, error};

type ObjectID = Uuid;

#[derive(Debug)]
pub struct FileStore {
    data_path: PathBuf,
    index_path: PathBuf,
    index: JsonKeystore<ObjKey>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FilestoreObjKey {
    key: ObjKey,
    offset: u64,
}

impl FileStore {
    pub fn new(root_path: PathBuf) -> FileStore {
        let mut data_path = root_path.clone();
        let mut index_path = root_path.clone();

        index_path.push("index.json");
        data_path.push("data.bin");

        debug!("Opening file at {}", data_path.to_str().unwrap());
        let fs = FileStore { 
            data_path: data_path,
            index_path: index_path,
            index: JsonKeystore::default()
        };
        
        fs
    }

    pub fn get_objects(&self) -> &HashMap<ObjectID, ObjKey> {
        self.index.get_objects()
    }
}

impl Clone for FilestoreObjKey {
    fn clone(&self) -> Self {
        Self {..*self}
    }
}

use crate::object::{Manifest, ManifestLocation};
impl StoresObjects for FileStore {
    // store a binary object. return its uuid
    fn put(&mut self, data: &[u8]) -> Result<ObjectID, Error> {
        debug!("put data: {:?}", &data);

        let mut objfile = OpenOptions::new()
            .write(true)
            .create(true)
            .open(self.data_path.as_path())
            .unwrap();

        // key generation
        let mut key = ObjKey {
                uuid: Uuid::new_v5(&Uuid::NAMESPACE_OID, data),
                //uuid: Uuid::new_v4(),
                //uuid: 0,
                hash: 0,
                size: data.len() as u64, 
                manifest: Manifest { 
                    shards: Vec::from([ManifestLocation::SingleBlockDevice { 
                        lba: objfile.seek(SeekFrom::End(0)).unwrap(),
                        size: data.len() as u64
                    } ]) 
                }
            //offset: objfile.seek(SeekFrom::End(0)).unwrap()
        };
        let mut hasher = DefaultHasher::new();
        hasher.write(data);
        key.hash = hasher.finish();

        debug!("{:?}", &key);

        // store key
        // insert the key into the index
        self.index.set(key.uuid, key);

        // store object
        //write the object 
        let _bytes_written = objfile.write(data);
        objfile.flush()?;

        Ok(key.uuid)
    }

    fn get(&mut self, uuid: ObjectID) -> Result<Option<Vec<u8>>, Error> {
        debug!("get uuid: {:?}", uuid);

        // retrieve key
        // look up uuid
        let key = self.index.get(&uuid);
        if key.is_none() { return Err(format!("No object with uuid {}", &uuid)); }

        let key = key.unwrap();

        debug!("{:?}", &key);

        // create a vector for the data
        let mut data = vec![0u8; key.size as usize];
        debug!("vector capacity: {:?}", data.len());

        // retrieve data
        // open the file, seek to the right spot, and read the data
        let mut f = OpenOptions::new().read(true).open(&self.data_path)?;

        let loc = key.manifest.shards[0];
        f.seek(SeekFrom::Start(loc.lba))?;
        let read_bytes = f.read(&mut data)?;
        debug!("read {:?} bytes", read_bytes);

        Ok(Some(data))
    }

    fn delete(&mut self, uuid: ObjectID) -> Result<Option<ObjectID>, Error> {
        // retrieve key
        // TODO: separate key retrieval and deletion
        let key = self.index.delete(&uuid)?;
        if key.is_none() { return Err(format!("No object with uuid {}", &uuid)); }

        let key = key.unwrap();

        let shard = key.manifest.shards[0];

        // zero-out data in data file
        //  is this necessary?
        let mut f = OpenOptions::new().write(true).open(&self.data_path)?;
        f.seek(SeekFrom::Start(shard.lba))?;
        f.write(&vec![0; key.size as usize])?;

        // TODO: delete key
        // TODO: release space on freelist

        Ok(Some(uuid))
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn test_save_index() {

    }
}
