use crate::freelist::{FreeList, FreeListNode};
use crate::object::{Manifest, ManifestLocation};

use log::{trace, debug, info, warn, error};

#[derive(Debug)]
pub struct VecFreeList {
    free: Vec<FreeListNode>,
}

impl VecFreeList {
    fn new(size:usize) -> Self {
        let mut s = Self {
            free: Vec::new(),
        };

        let new_node = FreeListNode { size, address: 0 };
        s.free.push(new_node);
        s
    }
}

impl FreeList for VecFreeList {
    fn allocate(&mut self, size:usize) -> Result<Manifest, String> {
        let index = match self.free.binary_search_by(|node| node.size.cmp(&size)) {
            Ok(idx) => idx,
            Err(idx) => {
                // didn't find something exactly the right size
                // if we're at the end, return an error
                if idx == self.free.len() {
                    let s = String::from(format!("Could not allocate size {}", size));
                    error!("Could not allocate size {}", size);
                    return Err(s);
                }
                idx
            }
        };
        let node = &mut self.free[index];
        let address = node.address;
        debug!("allocated {} at {}", size, address);
        node.size -= size;
        node.address += size;
        if node.size == 0 {
            self.free.remove(index);
        }

        let m = Manifest { shards: Vec::new() };
        m.shards.push(ManifestLocation { lba: address as u64, span: size as u64, blkdevid: None });
        return Ok(m);
    }

    fn release(&mut self, size:usize, address:usize) -> Result<(), String> {
        debug!("Releasing {} at {}", size, address);
        // TODO: check if the area being released overlaps a free area
        // TODO: check if the area being released is outside of max size
        // TODO: check if we're adjacent to another free area and combine

        let index = match self.free.binary_search_by(|node| node.size.cmp(&size)) {
            Ok(idx) => idx,
            Err(idx) => idx
        };
        trace!("index: {}", index);

        self.free.insert(index, FreeListNode { size, address });

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

