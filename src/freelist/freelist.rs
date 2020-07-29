use log::{error, warn, info, debug, trace};

#[derive(Debug, PartialEq)]
struct FreeListNode {
    size: usize,
    address: usize
}

pub trait FreeList {
    fn new(size:usize) -> Self;

    fn allocate(&mut self, size:usize) -> Result<usize, String>;
    fn release(&mut self, size:usize, address:usize) -> Result<(), String>;
}




