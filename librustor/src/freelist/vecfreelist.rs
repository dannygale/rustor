mod freelist;
use freelist::{FreeList, FreeListNode};

#[derive(Debug)]
struct VecFreeList {
    free: Vec<FreeListNode>,
}

impl FreeList for VecFreeList {
    fn new(size:usize) -> Self {
        let mut s = Self {
            free: Vec::new(),
        };

        let new_node = FreeListNode { size, address: 0 };
        s.free.push(new_node);
        s
    }

    fn allocate(&mut self, size:usize) -> Result<usize, String> {
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

        return Ok(address);
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
        
        assert_eq!(list.free[0], FreeListNode {size: 1000, address: 0});

        let address = list.allocate(10).unwrap();
        assert_eq!(address, 0);
        assert_eq!(list.free[0], FreeListNode {size: 990, address: 10});

        let address = list.allocate(20).unwrap();
        assert_eq!(address, 10);
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

        let address = list.allocate(30).unwrap();
        assert_eq!(address, 30);

    }
}

