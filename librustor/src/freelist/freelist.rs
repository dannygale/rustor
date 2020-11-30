use log::{error, warn, info, debug, trace};

use crate::object::Manifest;

#[derive(Debug, PartialEq)]
pub struct FreeListNode {
    pub size: usize,
    pub address: usize
}

pub trait FreeList {
    fn allocate(&mut self, size:usize) -> Result<Manifest, String>;
    fn release(&mut self, size:usize, address:usize) -> Result<(), String>;
}




