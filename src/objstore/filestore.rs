use std::path::{PathBuf};
use std::fs::{File, OpenOptions};
use std::default::Default;
use std::collections::HashMap;
use std::io::{Seek, SeekFrom, Read, Write};


use uuid::Uuid;

use crate::objstore::ObjectStore;
use crate::object::Object;

#[derive(Debug)]
pub struct FileStore {
    data_path: PathBuf,
    objfile: File,
    index_path: PathBuf,
    index: HashMap<Uuid, ObjKey>,
}

#[derive(Debug)]
struct ObjKey {
    uuid: Uuid,
    hash: u64,
    size: u64,
    offset: u64,
}

impl FileStore {
    pub fn new(root_path: PathBuf, n_files: usize) -> FileStore {
        let data_path = root_path.clone();
        let index_path = root_path.clone();

        index_path.push("index.json");
        data_path.push("data.bin");

        let mut fs = FileStore { 
            objfile: OpenOptions::new().read(true).write(true).create(true).
                open(data_path).unwrap(),
            data_path: data_path,
            index_path: index_path,
            index: HashMap::new()
        };
        
        fs
    }
}

impl ObjectStore for FileStore {
    // store a binary object. return its uuid
    fn put(self, data: &[u8]) -> Uuid {
        let mut new_obj = Object {
            //uuid: Uuid::new_v5(&Uuid::NAMESPACE_OID, data),
            uuid: Uuid::new_v4(),
            data: Some(data.to_vec()),
            hash: 0,
            size: data.len() as u64
        };
        new_obj.hash = new_obj.calculate_hash();
        // now have a fully poppulated object struct

        // seek to the end of the file
        let offset = self.objfile.seek(SeekFrom::End(0)).unwrap();
        
        //write the object there
        self.objfile.write(data);

        // build the key
        let key = ObjKey {
            uuid: new_obj.uuid,
            hash: new_obj.hash,
            size: new_obj.size,
            offset: offset
        };

        // insert the key into the index
        self.index.insert(key.uuid, key);

        new_obj.uuid
    }

    fn get(self, uuid: Uuid) -> &'static[u8] {

    }
}
