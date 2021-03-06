
pub mod freelist;
pub use freelist::*;

pub mod vecfreelist;
pub use vecfreelist::VecFreeList;

pub mod rcvecfreelist;
pub use rcvecfreelist::*;

mod bitmap;

pub mod bitmapfreelist;
pub use bitmapfreelist::*;


//pub mod avl;
//pub use avl::*;

//pub mod freetree;
//pub use freetree::*;

//pub mod llfreelist;
//pub use llfreelist::*;

//use crate::object::Manifest;
//use crate::RResult;

/*
#[derive(Debug, PartialEq)]
pub struct FreeListNode {
    pub size: u64,
    pub address: u64
}

pub trait FreeList {
    fn allocate(&mut self, size:u64) -> RResult<Manifest>;
    fn release(&mut self, size:u64, address:u64) -> RResult<()>;
}
*/

/*
#[derive(Debug)]
pub enum FreeListError {
    AllocationError,
    ReleaseOfFreeArea,
}

impl Error for FreeListError {}

use std::fmt;
impl fmt::Display for FreeListError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FreeListError::AllocationError => write!(f, "Could not allocate"),
            FreeListError::ReleaseOfFreeArea => write!(f, "Tried to release a free area"),
        }
    }
}
*/
