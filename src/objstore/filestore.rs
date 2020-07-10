use std::path::{PathBuf};
use std::fs::{OpenOptions};
use std::fs;
//use std::default::Default;
use std::collections::HashMap;
use std::io::{Seek, SeekFrom, Read, Write, Error};

use serde_json;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::objstore::ObjectStore;
use crate::object::Object;

type ObjectID = Uuid;

type Index = HashMap<ObjectID, ObjKey>;

#[derive(Debug)]
pub struct FileStore {
    data_path: PathBuf,
    index_path: PathBuf,
    index: Index
}

#[derive(Serialize, Deserialize, Debug)]
struct ObjKey {
    uuid: ObjectID,
    hash: u64,
    size: u64,
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
            index: HashMap::new()
        };
        
        fs
    }

    fn read_index(&self) -> Result<Index, Error> {
        println!("Opening index at {}", self.index_path.to_str().unwrap());
        let mut f = OpenOptions::new().create(true);
        let mut indexfile = OpenOptions::new()
            .read(true)
            .open(self.index_path.as_path())
            .unwrap();

        println!("Reading index from file");
        let v = serde_json::from_reader(indexfile)?;
        Ok(v)
    }


    fn write_index(&self, idx: &Index) -> Result<(), Error> {
        println!("Writing index at {}", self.index_path.to_str().unwrap());
        let mut idxfile = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(self.index_path.as_path())
            .unwrap();
        serde_json::to_writer_pretty(idxfile, idx);
        Ok(())
    }

    pub fn put_str(&mut self, text: &str) -> Result<ObjectID, Error> {
        let objid = self.put(text.as_bytes())?;
        println!("put_str: '{}' -> objid: {}", text, &objid);
        Ok(objid)
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
            uuid: Uuid::new_v5(&Uuid::NAMESPACE_OID, data),
            //uuid: Uuid::new_v4(),
            //uuid: 0,
            data: Some(data.to_vec()),
            hash: 0,
            size: data.len() as u64
        };
        new_obj.hash = new_obj.calculate_hash();
        //new_obj.uuid = new_obj.hash;
        // now have a fully poppulated object struct

        // seek to the end of the file
        let offset = objfile.seek(SeekFrom::End(0)).unwrap();
        
        // build the key
        let key = ObjKey {
            uuid: new_obj.uuid,
            hash: new_obj.hash,
            size: new_obj.size,
            offset: offset
        };
        println!("{:?}", &key);

        // insert the key into the index
        let mut index = self.read_index()?;
        println!("inserting key into index");
        index.insert(key.uuid, key);
        self.write_index(&index);

        //write the object 
        let bytes_written = objfile.write(data);
        objfile.flush()?;

        Ok(new_obj.uuid)
    }

    fn get(&mut self, uuid: ObjectID) -> Result<Option<Vec<u8>>, Error> {
        println!("get uuid: {:?}", uuid);

        let idx = self.read_index()?;

        // look up uuid
        let objkey = idx.get(&uuid).unwrap();
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
