use uuid::Uuid;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use serde::{Serialize, Deserialize};

pub type ObjectID = Uuid;
pub type BlkDevID = Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct ManifestLocation {
    pub blkdevid: Option<BlkDevID>,
    /// starting LBA
    pub lba: u64,   
    /// number of LBAs 
    pub span: u64,  
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Manifest {
    pub shards: Vec<ManifestLocation>,
}


#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ObjKey {
    pub uuid: Uuid,
    pub hash: u64,
    pub size: u64,
    pub manifest: Manifest,
}

#[derive(Debug, Default)]
pub struct Object {
    pub key: ObjKey,
    pub data: Option<Vec<u8>>
}

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state)
    }
}

impl Object {
    pub fn calculate_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
}

impl Clone for ObjKey {
    fn clone(&self) -> ObjKey {
        ObjKey {..*self}
    }
}

pub trait IsObject {
    fn get_uuid(&self) -> ObjectID;
    fn get_hash(&self) -> u64;
    fn get_size(&self) -> u64;
    fn get_data(&self) -> &Option<Vec<u8>>;
}

impl IsObject for Object {
    fn get_uuid(&self) -> ObjectID { self.key.uuid }
    fn get_hash(&self) -> u64 { self.key.hash }
    fn get_size(&self) -> u64 { self.key.size }
    fn get_data(&self) -> &Option<Vec<u8>> { &self.data }
}

