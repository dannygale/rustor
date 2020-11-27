
use std::default::Default;

#[derive(Debug, Default)]
struct FreeTreeNode<'a> {
    left: Option<&'a mut FreeTreeNode<'a>>,
    right: Option<&'a mut FreeTreeNode<'a>>,

    size: u64,
    locations: Vec<u64>
}

impl<'a> FreeTreeNode<'a> {
    pub fn new(size: u64, offset: u64) -> Self {
        Self {
            left: None,
            right: None,
            size: size,
            locations: vec![offset]
        }
    }

    /// return the smallest node >= size
    pub fn find(&self, size: u64) -> Option<&Self> {

        if self.size >= size { // if this node is big enough
            if let Some(node) = &self.left { // if we have a left child 
                // if the left child is too small, we're the one we're looking for
                if node.size < size { return Some(&self); }
                // otherwise, recurse to it 
                else { return node.find(size); }
            } else { return Some(&self); } // no left child, this is it
        } else { 
            // this node isn't big enough. if we have a right child, go right. if not, return None
            if let Some(node) = &self.right {
                return node.find(size);
            } else { return None; }
        }
    }
    
    pub fn push(&mut self, other: &'a mut Self) {
        if other.size <= self.size {
            // it goes left if it's less than or equal to current size
            if let Some(node) = &mut self.left {
                node.push(other);
                // TODO: rebalance
            } else {
                self.left = Some(other);
            }
        } else {
            // other.size > self.size
            if let Some(node) = &mut self.right {
                node.push(other);
                // TODO: rebalance
            } else {
                self.right = Some(other);
            }
        }
    }

}

#[derive(Debug, Default)]
pub struct FreeTree<'a> {
    root: FreeTreeNode<'a>
}

impl<'a> FreeTree<'a> {
    pub fn new(size: u64) -> Self {
        Self { 
            root: FreeTreeNode {
                left: None,
                right: None,
                size: size,
                locations: vec![0]
            } 
        }
    }

    pub fn find(&self, size: u64) -> Option<&'a FreeTreeNode> {
        self.root.find(size)
    }

    pub fn find_flat(&self, size: u64) -> Option<&'a FreeTreeNode> { 
        let mut cursor = &self.root;

        // find the smallest block that's >= what we need
        loop {
            if cursor.size == size {
                return Some(&cursor);
            } else if cursor.size < size {
                // this free spot is too small. 
                // see if there's a bigger one to the right
                if let Some(node) = &cursor.right {
                    cursor = node;
                    continue;
                } else {
                    // this is too small but there are none bigger
                    return None;
                }
            } else { // node.size > size
                if let Some(node) = &cursor.left {
                    if node.size < size {
                        return Some(cursor);
                    } else {
                        cursor = node;
                    }
                } else {
                    return Some(cursor);
                }
            }
        }
    }

    pub fn push(&mut self, size: u64, offset: u64) -> Result<(), String> {
        let mut cursor = &mut self.root;
        let mut stack: Vec<&FreeTreeNode> = Vec::new();

        // if the new one is greater than the cursor, go right
        loop {
            if size == cursor.size {
                cursor.locations.push(offset);
                return Ok(());
            } else if size < cursor.size {
                // if new node is smaller than cursor, find smaller block (go left)
                if let Some(&mut node) = &mut cursor.left {
                    // if there's a node to the left, move to that one
                    stack.push(&cursor);
                    *cursor = node;
                } else {
                    // otherwise, put this one there
                    cursor.left = Some(&mut FreeTreeNode::new(size, offset));
                }
            } else { // new node is > cursor 
                if let Some(&mut node) = &mut cursor.right {
                    *cursor = node;
                } else {
                    cursor.right = Some(&mut FreeTreeNode::new(size, offset));
                }
            }
        }
    }

    pub fn rebalance(&mut self) {
        // TODO: ensure the tree is always balanced
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mut tree = FreeTree::new(1000);

        assert_eq!(tree.root.size, 1000);
        assert_eq!(tree.root.left, None);
        assert_eq!(tree.root.right, None);
    }

    #[test]
    fn test_insert() {
    }

    #[test]
    fn test_search() {

    }

    #[test]
    fn test_sort() {

    }
}


