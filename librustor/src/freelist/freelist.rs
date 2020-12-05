use crate::object::Manifest;

use crate::RResult;

#[derive(Debug, PartialEq)]
pub struct FreeListNode {
    pub span: u64,
    pub address: u64
}

pub trait FreeList {
    fn allocate(&mut self, span:u64) -> RResult<Manifest>;
    fn release(&mut self, span:u64, address:u64) -> RResult<()>;
}




