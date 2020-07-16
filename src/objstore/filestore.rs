use std::path::{PathBuf};
use std::fs::{OpenOptions};
use std::collections::HashMap;
use std::io::{Seek, SeekFrom, Read, Write, Error};

// for data hashing
use std::hash::Hasher;
use std::collections::hash_map::DefaultHasher;

//use serde_json;
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use uuid::Uuid;

//use crate::keystore::json_keystore::JsonKeystore;
use crate::objstore::ObjectStore;
use crate::object::{ObjKey};
use crate::keystore::keystore::KeyStore;
use crate::keystore::JsonKeystore;

type ObjectID = Uuid;

type Index<T> where T: Serialize + DeserializeOwned = HashMap<ObjectID, T>;

#[derive(Debug)]
pub struct FileStore {
    data_path: PathBuf,
    index_path: PathBuf,
    index: JsonKeystore<FilestoreObjKey>
}

#[derive(Serialize, Deserialize, Debug, Copy)]
struct FilestoreObjKey {
    key: ObjKey,
    offset: u64,
}

impl FileStore {
    pub fn new(root_path: PathBuf) -> FileStore {
        let mut data_path = root_path.clone();
        let mut index_path = root_path.clone();

        index_path.push("index.json");
        data_path.push("data.bin");

        println!("Opening file at {}", data_path.to_str().unwrap());
        let fs = FileStore { 
            data_path: data_path,
            index_path: index_path,
            index: JsonKeystore::default()
        };
        
        fs
    }
}

impl Clone for FilestoreObjKey {
    fn clone(&self) -> Self {
        Self {..*self}
    }
}

impl ObjectStore for FileStore {
    // store a binary object. return its uuid
    fn put(&mut self, data: &[u8]) -> Result<ObjectID, Error> {
        println!("put data: {:?}", &data);

        let mut objfile = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(self.data_path.as_path())
            .unwrap();

        let mut fs_key = FilestoreObjKey {
            key: ObjKey {
                uuid: Uuid::new_v5(&Uuid::NAMESPACE_OID, data),
                //uuid: Uuid::new_v4(),
                //uuid: 0,
                hash: 0,
                size: data.len() as u64 
            },
            offset: objfile.seek(SeekFrom::End(0)).unwrap()
        };
        let mut hasher = DefaultHasher::new();
        hasher.write(data);
        fs_key.key.hash = hasher.finish();

        // seek to the end of the file
        fs_key.offset = objfile.seek(SeekFrom::End(0)).unwrap();
        
        println!("{:?}", &fs_key.key);

        // insert the key into the index
        self.index.set(fs_key.key.uuid, fs_key);

        //write the object 
        let _bytes_written = objfile.write(data);
        objfile.flush()?;

        Ok(fs_key.key.uuid)
    }

    fn get(&mut self, uuid: ObjectID) -> Result<Option<Vec<u8>>, Error> {
        println!("get uuid: {:?}", uuid);

        // look up uuid
        let objkey = self.index.get(&uuid).unwrap();
        println!("{:?}", &objkey);

        // create a vector for the data
        let mut data = vec![0u8; objkey.key.size as usize];
        println!("vector capacity: {:?}", data.len());

        // open the file, seek to the right spot, and read the data
        let mut f = OpenOptions::new().read(true).open(&self.data_path)?;
        f.seek(SeekFrom::Start(objkey.offset))?;
        let read_bytes = f.read(&mut data)?;
        println!("read {:?} bytes", read_bytes);

        Ok(Some(data))
    }

    fn delete(&mut self, uuid: ObjectID) -> Result<Option<ObjectID>, Error> {
        let fs_key = match self.index.delete(&uuid)? {
            Some(k) => k,
            None => return Ok(None)
        };

        // zero-out data in data file
        let mut f = OpenOptions::new().write(true).open(&self.data_path)?;
        f.seek(SeekFrom::Start(fs_key.offset))?;
        f.write(&vec![0; fs_key.key.size as usize])?;

        Ok(Some(uuid))
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn test_save_index() {

    }
}
