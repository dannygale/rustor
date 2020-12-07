#![allow(unused_imports)]
use crate::freelist::{FreeList, FreeListNode};
use crate::object::{Manifest, ManifestLocation};
use crate::RResult;

use log::{trace, debug, info, warn, error};
use std::rc::Rc;
use std::cell::RefCell;

use crate::blockstore::BS4K;

#[derive(Debug)]
pub struct RCVecFreeList {
    pub by_size: Vec<Rc<RefCell<FreeListNode>>>,
    pub by_addr: Vec<Rc<RefCell<FreeListNode>>>
}

impl RCVecFreeList {
    pub fn new(span:u64) -> Self {
        let new_node = Rc::new(RefCell::new(FreeListNode { blkdevid: None, span, address: 0 }));
        Self {
            by_size: vec![Rc::clone(&new_node)],
            by_addr: vec![Rc::clone(&new_node)],
        }
    }

    fn insert_node(&mut self, node: Rc<RefCell<FreeListNode>>) {
        trace!("insert {:?}", &node);
        // insert the new node into the by_addr list
        let pos = self.by_addr.binary_search_by(
            |n| n.borrow().address.cmp(&node.borrow().address))
            .unwrap_or_else(|e| e);
        self.by_addr.insert(pos, Rc::clone(&node));

        // insert the new node into the by_size list, too
        let pos = self.by_size.binary_search_by(
            |n| n.borrow().span.cmp(&node.borrow().span))
            .unwrap_or_else(|e| e);
        self.by_size.insert(pos, Rc::clone(&node));
    }

    fn remove_node(&mut self, node: &Rc<RefCell<FreeListNode>>) -> RResult<()> {
        debug!("remove node {:?}", node.borrow());
        if let Ok(pos) = self.by_addr.binary_search_by(
            |n| n.borrow().address.cmp(&node.borrow().address)) {
            self.by_addr.remove(pos);
        } else { return Err(format!("could not find node {:?}", node))?; }

        for i in 0..self.by_size.len() {
            if &self.by_size[i] == node {
                self.by_size.remove(i);
                break;
            }
        }

        /*
        if let Ok(pos) = self.by_size.binary_search_by(
            |n| n.borrow().span.cmp(&node.borrow().span)) {
            self.by_size.remove(pos);
        } else { return Err(format!("could not find node {:?}", node))?; }
        */

        Ok(())
    }

    fn find_node_containing(&self, lba:u64, span:u64) -> Option<&Rc<RefCell<FreeListNode>>> {
        trace!("looking for node containing {} at {}", &span, &lba);
        let mut pos = self.by_addr.binary_search_by(
            |node| node.borrow().address.cmp(&node.borrow().address))
            .unwrap_or_else(|e| e);

        trace!("pos {:?}, len {}", &pos, &self.by_addr.len());

        if pos > 0 { pos -= 1; }

        let node = &self.by_addr[pos];
        let n = node.borrow();

        trace!("found node {:?}", &n);
        if n.address <= lba && (n.address + n.span) >= (lba+span) {
            return Some(node);
        } else { return None; }
    }

    fn sort_size(&mut self) {
        self.by_size.sort_by(|a,b| a.borrow().span.cmp(&b.borrow().span));       
    }

    fn sort_addr(&mut self) {
        self.by_addr.sort_by(|a,b| a.borrow().address.cmp(&b.borrow().address));       
    }

    fn sort(&mut self) {
        self.sort_size();
        self.sort_addr();
    }
}


impl FreeList for RCVecFreeList {
    fn allocate(&mut self, size_bytes:u64) -> RResult<Manifest> {
        // find smallest free block that can accommodate size_bytes
        let mut span = size_bytes / BS4K as u64;
        if size_bytes as usize & (BS4K - 1) != 0 { span += 1; }

        let pos = match self.by_size.binary_search_by(|node| node.borrow().span.cmp(&span)) {
            Ok(idx) => idx,
            Err(idx) => {
                // didn't find something exactly the right size
                // if we're at the end, return an error
                if idx == self.by_size.len() {
                    return Err("Could not allocate")?;
                }
                idx
            }
        };

        let rcnode = Rc::clone(&mut self.by_size[pos]);
        let mut node = rcnode.borrow_mut();
        let address = node.address;
        debug!("allocated {} blocks at {}", span, address);
        node.span -= span;
        node.address += span;

        if node.span == 0 {
            self.remove_node(&rcnode)?;
        }

        return Ok( Manifest { shards: vec![ManifestLocation { 
            lba: address as u64, span: span as u64, blkdevid: None }]});
    }

    fn release(&mut self, span:u64, address:u64) -> RResult<()> {
        debug!("Releasing {} blocks at {}", span, address);
        // TODO: check if the area being freed is already free
        // TODO: check if the area being released overlaps a free area
        // TODO: check if the area being released is outside of max size
        // TODO: check if we're adjacent to another free area and combine

        let index = match self.by_addr.binary_search_by(|node| node.borrow().span.cmp(&span)) {
            Ok(idx) => idx,
            Err(idx) => idx, // this is fine, it just means this will be the largest free block
        };
        trace!("index: {}", index);

        self.free(span, address)?;

        Ok(())
    }

    /// forcibly remove `span` at the specified `lba` from the freelist
    /// returns an error if the area specified by `lba` and `span` is not fully
    /// contianed in an existing node of the freelist
    fn take(&mut self, span:u64, lba: u64) -> RResult<()> {
        debug!("take {} at {}", &span, &lba);
        if let Some(rcnode) = &mut self.find_node_containing(lba, span) {
            // expect here that the target region is fully contained in rcnode
            
            let node = Rc::clone(rcnode);
            trace!("{} at {} contained in {:?}", &span, &lba, &node.borrow());

            if node.borrow().address == lba {

                let mut n = node.borrow_mut();
                trace!("taking {} at base addr {}", &span, &n.address);
                n.address += span;
                n.span -= span;
                trace!("now: {:?}", &n);
                drop(n);

                self.sort();
                trace!("freelist: {:#?}", &self.by_addr);
                if node.borrow().span == 0 { self.remove_node(&node)?; }
                trace!("freelist: {:#?}", &self.by_addr);

                return Ok(());
            }

            // otherwise we need to split this node
            // first create a new node at the top if needed, then reduce the size of the original
            trace!("splitting {:?}", node.borrow());
            let new_node = FreeListNode {
                blkdevid: None, 
                address: lba+span, 
                span: node.borrow().address + node.borrow().span - (lba+span)
            };
            trace!("new upper node {:?}", &new_node);
            self.insert_node(Rc::new(RefCell::new(new_node)));

            // split this node
            let mut n = node.borrow_mut();
            n.span = lba - n.address;
            trace!("reduced original node {:?}", &n);
            drop(n);
            self.sort_size();

        } else {
            return Err(format!("Could not find node spanning {} blocks at address {}", &span, &lba))?;
        }
        Ok(())
    }
    /// forcibly releases `span` blocks at `lba` by adding a node to the freelist
    /// returns an error if the area defined by `lba` and `span` is already partially freed
    fn free(&mut self, span:u64, lba: u64) -> RResult<()> {
        let node = FreeListNode { blkdevid: None, address: lba, span: span };

        let pos = self.by_addr.binary_search_by(
            |n| n.borrow().address.cmp(&node.address))
            .unwrap_or_else(|e| e);

        if pos > 0 && node.overlaps(&self.by_addr[pos-1].borrow()) {
            return Err(format!("{} blocks at lba {} already free", span, lba))?;
        }
        if pos < self.by_addr.len() && node.overlaps(&self.by_addr[pos].borrow()) {
            return Err(format!("{} blocks at lba {} already free", span, lba))?;
        }

        self.insert_node(Rc::new(RefCell::new(node)));
        
        Ok(())
    }
}


/*
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
*/
