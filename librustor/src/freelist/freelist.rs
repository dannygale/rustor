use log::{error, warn, info, debug, trace};

#[derive(Debug, PartialEq)]
pub struct FreeListNode {
    pub size: usize,
    pub address: usize
}

pub trait FreeList {
    fn new(size:usize) -> Self;

    fn allocate(&mut self, size:usize) -> Result<usize, String>;
    fn release(&mut self, size:usize, address:usize) -> Result<(), String>;
}




