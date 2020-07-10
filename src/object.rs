use uuid::Uuid;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub type ObjectID = Uuid;

#[derive(Debug)]
#[derive(Default)]
pub struct Object {
    pub uuid: ObjectID,
    pub hash: u64,
    pub size: u64,
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

