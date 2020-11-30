use crate::object::Manifest;

#[derive(Debug, PartialEq)]
pub struct FreeListNode {
    pub size: u64,
    pub address: u64
}

pub trait FreeList {
    fn allocate(&mut self, size:u64) -> Result<Manifest, String>;
    fn release(&mut self, size:u64, address:u64) -> Result<(), String>;
}




