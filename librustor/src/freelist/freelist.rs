use crate::object::Manifest;

use crate::RResult;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FreeListNode {
    pub blkdevid: Option<Uuid>,
    pub span: u64,
    pub address: u64
}

impl FreeListNode {
    pub fn overlaps(&self, other:&Self) -> bool {
        self.address >= other.address && self.address <= other.address + other.span ||
            other.address >= self.address && other.address <= self.address + self.span
    }

    pub fn adjacent(&self, other:&Self) -> bool {
        self.address + self.span == other.address ||
            other.address + other.span == self.address
    }
}

use std::rc::Rc;
pub type RCFreeListNode = Rc<FreeListNode>;

use crate::object::ObjKey;
pub trait FreeList {
    fn allocate(&mut self, span:u64) -> RResult<Manifest>;
    fn release(&mut self, span:u64, address:u64) -> RResult<()>;

    fn take(&mut self, span:u64, lba: u64) -> RResult<()>;
    fn free(&mut self, span:u64, lba: u64) -> RResult<()>;
}

pub trait FreeListFromKeys {
    fn from_keys<'a, I>(&mut self, keys: I) -> RResult<()> where I: Iterator<Item=&'a ObjKey>;
}

