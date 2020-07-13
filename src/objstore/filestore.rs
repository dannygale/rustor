use std::path::{PathBuf};
use std::fs::{OpenOptions};
use std::collections::HashMap;
use std::io::{Seek, SeekFrom, Read, Write, Error};

//use serde_json;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

//use crate::keystore::json_keystore::JsonKeystore;
use crate::objstore::ObjectStore;
use crate::object::{Object, ObjKey};
use crate::keystore::keystore::KeyStore;
use crate::keystore::json_keystore::JsonKeystore;

type ObjectID = Uuid;

type Index = HashMap<ObjectID, ObjKey>;

#[derive(Debug)]
pub struct FileStore {
    data_path: PathBuf,
    index_path: PathBuf,
    index: JsonKeystore
}

#[derive(Serialize, Deserialize, Debug)]
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

        let mut new_obj = Object {
            key: ObjKey {
                uuid: Uuid::new_v5(&Uuid::NAMESPACE_OID, data),
                //uuid: Uuid::new_v4(),
                //uuid: 0,
                hash: 0,
                size: data.len() as u64 
            },
            data: Some(data.to_vec()),
        };
        new_obj.key.hash = new_obj.calculate_hash();
        //new_obj.uuid = new_obj.hash;
        // now have a fully poppulated object struct

        // seek to the end of the file
        let offset = objfile.seek(SeekFrom::End(0)).unwrap();
        
        let key = new_obj.key;
        println!("{:?}", &key);

        // insert the key into the index
        self.index.set(key.uuid, key);

        //write the object 
        let bytes_written = objfile.write(data);
        objfile.flush()?;

        Ok(new_obj.key.uuid)
    }

    fn get(&mut self, uuid: ObjectID) -> Result<Option<Vec<u8>>, Error> {
        println!("get uuid: {:?}", uuid);

        // look up uuid
        let objkey = match self.index.get(&uuid)? {
            None => return Ok(None),
            Some(k) => k,
        };
        println!("{:?}", &objkey);

        // create a vector for the data
        let mut data = vec![0u8; objkey.size as usize];
        println!("vector capacity: {:?}", data.len());

        // open the file, seek to the right spot, and read the data
        let mut f = OpenOptions::new().read(true).open(&self.data_path)?;
        f.seek(SeekFrom::Start(objkey.offset))?;
        let read_bytes = f.read(&mut data)?;
        println!("read {:?} bytes", read_bytes);

        Ok(Some(data))
    }

    fn delete(&mut self, uuid: ObjectID) -> Result<(), Error> {
        // open index
        let mut idx = self.read_index()?;

        // get object key and remove it
        let key = idx.remove(&uuid).unwrap();

        // zero-out data in data file
        let mut f = OpenOptions::new().write(true).open(&self.data_path)?;
        f.seek(SeekFrom::Start(key.offset))?;
        f.write(&vec![0; key.size as usize])?;

        // persist the change in the index
        self.write_index(&idx)?;

        Ok(())
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn test_save_index() {

    }
}
