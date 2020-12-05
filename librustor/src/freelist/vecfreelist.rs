#![allow(unused_imports)]
use crate::freelist::{FreeList, FreeListNode};
use crate::object::{Manifest, ManifestLocation};

use log::{trace, debug, info, warn, error};
use std::error::Error;

use crate::blockstore::BS4K;

#[derive(Debug)]
pub struct VecFreeList {
    free: Vec<FreeListNode>,
}

impl VecFreeList {
    pub fn new(span:u64) -> Self {
        let mut s = Self {
            free: Vec::new(),
        };

        let new_node = FreeListNode { span, address: 0 };
        s.free.push(new_node);
        s
    }
}

use crate::RResult;
use crate::GeneralError;

impl FreeList for VecFreeList {
    fn allocate(&mut self, size_bytes:u64) -> RResult<Manifest> {
        let mut span = size_bytes / BS4K as u64;
        if size_bytes as usize & (BS4K - 1) != 0 { span += 1; }

        let index = match self.free.binary_search_by(|node| node.span.cmp(&span)) {
            Ok(idx) => idx,
            Err(idx) => {
                // didn't find something exactly the right size
                // if we're at the end, return an error
                if idx == self.free.len() {
                    return Err("Could not allocate")?;
                }
                idx
            }
        };
        let node = &mut self.free[index];
        let address = node.address;
        debug!("allocated {} blocks at {}", span, address);
        node.span -= span;
        node.address += span;
        if node.span == 0 {
            self.free.remove(index);
        }

        let mut m = Manifest { shards: Vec::new() };
        m.shards.push(ManifestLocation { lba: address as u64, span: span as u64, blkdevid: None });
        return Ok(m);
    }

    fn release(&mut self, span:u64, address:u64) -> RResult<()> {
        debug!("Releasing {} blocks at {}", span, address);
        // TODO: check if the area being freed is already free
        // TODO: check if the area being released overlaps a free area
        // TODO: check if the area being released is outside of max size
        // TODO: check if we're adjacent to another free area and combine

        let index = match self.free.binary_search_by(|node| node.span.cmp(&span)) {
            Ok(idx) => idx,
            Err(idx) => idx, // this is fine, it just means this will be the largest free block
        };
        trace!("index: {}", index);

        self.free.insert(index, FreeListNode { span, address });

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocate() {
        let mut list = VecFreeList::new(1000);
        
        // test empty list
        assert_eq!(list.free[0], FreeListNode {size: 1000, address: 0});

        // test allocate into empty list
        let address = list.allocate(10).unwrap();
        assert_eq!(address, 0);
        // test remaining space was pared down appropriately
        assert_eq!(list.free[0], FreeListNode {size: 990, address: 10});

        // allocate a larger section
        let address = list.allocate(20).unwrap();
        // test that it's placed at the lowest available address
        assert_eq!(address, 10);
        // test that the remaining space is reduced properly
        assert_eq!(list.free[0], FreeListNode {size: 970, address: 30});
    }

    #[test]
    fn test_release () {
        let mut list = VecFreeList {
            free: vec![
                FreeListNode { size:70, address:30 },
                FreeListNode { size:900, address:100 }
            ]
        };
        let _resp = list.release(10, 0);
        assert_eq!(list.free[0], FreeListNode {size:10, address: 0});
        assert_eq!(list.free[1], FreeListNode {size:70, address: 30});

        // make sure this goes in the lowest slot
        let _resp = list.allocate(10);
        assert_eq!(list.free[0], FreeListNode {size:70, address:30});

        let address = list.allocate(30).unwrap();
        assert_eq!(address, 30);

    }
}

