use std::cmp;
use std::mem;

// TODO: use configuration options to handle duplicates
//      (a) put with duplicate key replaces old data
//      (b) put with duplicate key appends data to list in node
//      (c) put with duplicate key keeps data versions (?)
//      (d) ???

type OptBoxNode<K,D> = Option<Box<Node<K,D>>>;

#[derive(Default)]
struct Node<K, D> {
    key: K,
    data: D,

    height: isize,

    left: OptBoxNode<K,D>,
    right: OptBoxNode<K,D>,
}

use std::fmt;
impl<K,D> fmt::Debug for Node<K,D> 
where K: fmt::Debug, D: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let left = match &self.left {
            Some(node) => format!("Node {{ {:?}:{:?} }}", node.key, node.data),
            None => String::from("None"),
        };
        let right = match &self.right {
            Some(node) => format!("Node {{ {:?}:{:?} }}", node.key, node.data),
            None => String::from("None"),
        };
        write!(f, "{{ {:?}:{:?}, left: {:?}, right: {:?} }}", &self.key, &self.data, left, right)
    }
}

impl<K,D> Node<K,D> 
where K: PartialEq + PartialOrd,
      D: PartialEq + PartialOrd
{
    pub fn new(key: K, data: D) -> Self {
        Self { key, data, height: 1, left: None, right: None }
    }

    fn height(&mut self) -> isize {
        // cache result from potentially expensive drill-down
        // TODO: when does this need to be invalidated?
        if self.height != 0 {
            return self.height;
        }

        self.update_height()
    }

    fn update_height(&mut self) -> isize {
        let mut left_height = 0;
        let mut right_height = 0;
        if let Some(node) = &mut self.left {
            left_height = node.height() + 1;
        } 
        if let Some(node) = &mut self.right {
            right_height = node.height() + 1;
        } 

        self.height = cmp::max(left_height, right_height);
        self.height
    }

    /// return the difference in height between the right tree and the left tree
    /// a positive value indicates that the right tree is deeper
    /// a negative value indicates that the left tree is deeper
    fn balance_factor(&mut self) -> isize {
        let mut left_height = 0;
        let mut right_height = 0;
        if let Some(node) = &mut self.left {
            left_height = node.height();
        }
        if let Some(node) = &mut self.right {
            right_height = node.height();
        }

        right_height - left_height
    }

    /// recursively search for the given key
    pub fn find(&self, key: K) -> Option<&D> {
        if key == self.key {
            return Some(&self.data);
        } else if key < self.key {
            if let Some(node) = &self.left {
                return node.find(key);
            } else { 
                return None; 
            }
        } else { // key > self.key
            if let Some(node) = &self.right {
                return node.find(key);
            } else { 
                return None; 
            }
        }
    }

    /// insert a new key/data pair
    pub fn put(&mut self, key: K, data: D) -> Result<(), String> {
        self.height = 0;

        if key == self.key {
            self.data = data;
        } else if key < self.key {
            // key is less than self.key
            if let Some(node) = &mut self.left {
                // if we have a left node, recurse to it
                node.put(key, data)?;
                self.height = node.height() + 1;
            } else {
                // otherwise, create it
                self.left = Some(Box::new(Node::new(key, data,)));
            }
        } else {
            // key is greater than self.key
            if let Some(node) = &mut self.right {
                // if we have a right node, recurse to it
                node.put(key, data)?;
                self.height = node.height() + 1;
            } else {
                // otherwise, create it
                self.right = Some(Box::new(Node::new(key, data,)));
            }
        }

        self.rebalance();

        Ok(())
    }

    fn left_heavy(&mut self) -> bool {
        if self.balance_factor() < -1 { return true; }
        else { return false; }
    }
    fn right_heavy(&mut self) -> bool {
        if self.balance_factor() > 1 { return true; }
        else { return false; }
    }

    /*             self                  left
     *            /                     /    \
     *           left    =>     left_left    self
     *          /
     * left_left
     *
     *
     */
    fn rotate_right(&mut self) -> bool {
        let mut left: Box<Node<K,D>> = self.left.take().expect("no left child");
        let left_left: Box<Node<K,D>> = left.left.take().expect("no left-left child");

        // now self.left is None and left.left is None, but we have the data
        // need to:
        //   swap the keys and data between self and left
        //   make self.left = &left_left
        //   make self.right = &left
        mem::swap(&mut self.key, &mut left.key);
        mem::swap(&mut self.data, &mut left.data);
        self.left = Some(left_left);
        self.right = Some(left);

        true

        /*
        let mut new_root: Box<Node<K,D>> = self.left.unwrap();
        new_root.right = Some(rotate_root);
        self.left = None;
        self.right = None;
        */
    }

    /* self                            right
     *     \                          /     \
     *      right    =>           self      right_right
     *          \ 
     *           right_right
     *  move self to self.right.left and return self.right
     */
    fn rotate_left(&mut self) {
        if self.right.is_none() { return; };

        let right_right: Box<Node<K,D>> = self.right.as_mut().unwrap().right.take().unwrap();
        let mut right: Box<Node<K,D>> = self.right.take().unwrap();

        // now self.right is None and right.right is None, but we have the data
        // need to:
        //   swap the keys and data between self and right
        //   make self.right = &right_right
        //   make self.right = &right
        mem::swap(&mut self.key, &mut right.key);
        mem::swap(&mut self.data, &mut right.data);

        self.right = Some(right_right);
        self.right = Some(right);
        /*
        let new_root = self.right.unwrap();
        new_root.left = Some(Box::new(*self));
        self.left = None;
        self.right = None;

        &new_root
        */
    }

    /*     c             b
     *    /             / \
     *   a    =>       a   c 
     *    \ 
     *     b
     *
     */
    /*
    fn rotate_left_right(&mut self) -> &Self {
        let mut new_root = self.left.unwrap().right.unwrap();
        new_root.left = self.left; 
        new_root.right = Some(Box::new(*self));

        self.left = None;
        self.right = None;
        new_root.left.unwrap().left = None;
        new_root.left.unwrap().right = None;
         
        &new_root
    }
    */

    /*  a             b \           / \
     *    c    =>   a   c 
     *   /  
     *  b   
     *
     */
    /*
    fn rotate_right_left(&mut self) -> &Self {
        let mut new_root = self.right.unwrap().left.unwrap();
        new_root.left = Some(Box::new(*self));
        new_root.right = self.right;

        self.left = None;
        self.right = None;
        new_root.right.unwrap().left = None;
        new_root.right.unwrap().right = None;

        &new_root
    }
    */

    /* right rotation after a node is inserted in the left subtree of a left subtree
     * left rotation after a node is inserted in the right subtree of a right subtree
     * left-right rotation after a node is inserted as the right subtree of a left subtree
     * right-left rotation after a node is inserted as the left subtree of a right subtree
     */

    fn rebalance(&mut self) {
        match self.balance_factor() {
            -2 => {
                // the sub-tree rooted at this node is left-heavy
                let left = self.left.as_mut().unwrap();
                if left.balance_factor() > 0 {
                    left.rotate_right();
                }
            }
            2 => {
                // the sub-tree rooted at this node is right-heavy
                let right = self.right.as_mut().unwrap();
                if right.balance_factor() < 0 {
                    right.rotate_left();
                }
            }
            _ => {}
        };
    }

}

impl<K,D> PartialEq for Node<K,D> 
where K: PartialEq + PartialOrd,
      D: PartialEq + PartialOrd
{
    fn eq(&self, other: &Self) -> bool {
        (self.key == other.key) && (self.data == other.data)
    }
}



struct NodeIter<'a, K, D> {
    stack: Vec<&'a Node<K,D>>,
    curr: &'a OptBoxNode<K,D>,
}

impl<'a, K,D> Iterator for NodeIter<'a, K,D> 
where K: PartialEq + PartialOrd,
      D: PartialEq + PartialOrd
{
    type Item = &'a Node<K,D>;

    /// iterate over the elements in sorted order
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match *self.curr {
                // if we're at a node 
                Some (ref node) => {
                    // if this node has a left child, save this node on the stack and drill down
                    if node.left.is_some() {
                        self.stack.push(&node);
                        self.curr = &node.left;
                        continue;
                    } 
                    // if this node has a right child, put it on the stack and return this node
                    if node.right.is_some() {
                        self.curr = &node.right;
                        return Some(node);
                    }
                    // return this node and on the next iteration return the one from the top of
                    // the stack
                    // this is kind of like .take() ourself
                    self.curr = &None;
                    return Some(node);
                }

                // we're at a leaf. pop the top node off the stack.
                // if it has a right child, put that on the stack
                // return the popped top node
                None =>  {
                    match self.stack.pop() {
                        Some(node) => {
                            self.curr = &node.right;
                            return Some(node);
                        }
                        // end of iteration
                        None => return None
                    }
                }
            }
        }
    }
}



struct AVLTree<K,D> {
    root: OptBoxNode<K,D>
}

impl <'a, K,D> AVLTree<K,D> 
where K: PartialEq + PartialOrd + Copy + Clone,
      D: PartialEq + PartialOrd + Copy + Clone
{
    pub fn new() -> Self {
        Self {
            root: None
        }
    }
    pub fn iter(&'a self) -> NodeIter<'a, K, D> {
        NodeIter {
            stack: Vec::new(),
            curr: &self.root
        }
    }
    pub fn put(&mut self, key: K, data: D) -> bool {
        if let Some(ref mut boxnode) = &mut self.root {
            boxnode.put(key, data);
        } else {
            self.root = Some(Box::new(Node::new(key, data)));
        }
        return true;
    }

    /// return a vector of key/value tuples
    pub fn items(&self) -> Vec<(K,D)> {
        let mut iter = self.iter();
        let mut v = Vec::new();
        loop {
            if let Some(node) = iter.next() {
                v.push((node.key, node.data))
            } else {
                return v;
            }
        }
    }
}

impl<K,D> From <&Vec<(K,D)>> for AVLTree<K,D> 
where K: PartialEq + PartialOrd + Copy + Clone,
      D: PartialEq + PartialOrd + Copy + Clone,
{
    fn from(nodes: &Vec<(K,D)>) -> AVLTree<K,D>{
        let mut tree = AVLTree::new();
        for node in nodes {
            tree.put(node.0, node.1);
        }
        tree
    }
}

use std::iter::{Iterator, FromIterator, IntoIterator};
impl <K,D> FromIterator <Node<K,D>> for AVLTree<K,D> 
where K: PartialEq + PartialOrd + Copy + Clone,
      D: PartialEq + PartialOrd + Copy + Clone, 
{
    fn from_iter<I: IntoIterator<Item = Node<K,D>>>(iter: I) -> Self {
        let mut tree = Self::new();
        for i in iter {
            tree.put(i.key, i.data);
        }
        tree
    }
}

/*
impl<K,D> IntoIterator for AVLTree<K,D> 
where K: PartialEq + PartialOrd,
      D: PartialEq + PartialOrd
{
    type Item = (K,D);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
    }
}
*/

/*
use proptest::prelude::*;

fn arb_node(max_qty: usize) -> impl Strategy<Value = Node<isize, String>> {
    (any::<isize>(), "[a-z]*")
        .prop_map(|(key, data)| Node::new(key, data))
        .boxed()
}

prop_compose! {
    fn arb_node2(_d: isize) 
        (key in 0..100isize, data in "[a-z]*")
            -> Node<isize, String> {
                Node::new(key, data) 
        }
}


proptest! {
    #[test]
    fn test_put_inorder_set(nodes in arb_node2(0)) {
        println!("{:?}", nodes);
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_put_inorder_set() {
        let data = vec![
            (0, "asdf"),
            (1, "qwerty"),
            (2, "zxcv")
        ];
        let tree = AVLTree::from(&data);

        assert!(tree.items().eq(&data));
    }

    #[test]
    fn test_rotate_right() {
        let data = vec![
            (2, "zxcv"),
            (1, "qwerty"),
            (0, "asdf"),
        ];

        let mut a = Node::new(data[0].0, data[0].1);
        let mut b = Node::new(data[1].0, data[1].1); 
        let mut c = Node::new(data[2].0, data[2].1);

        let bref = &mut b;
        let aref = &mut a;

        b.left = Some(Box::new(c));
        a.left = Some(Box::new(b));

        assert_eq!(*bref.left.unwrap(), c);
        assert_eq!(*aref.left.unwrap(), b)
    }
}
