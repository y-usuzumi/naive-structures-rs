use Color::*;
use std::cmp::Ordering::*;
use std::rc::Rc;
use std::cell::{RefCell};

#[derive(Debug)]
enum Color {Black, Red}

#[derive(Debug)]
pub struct RBTree<K, V> where K: Ord + std::fmt::Debug, V: std::fmt::Debug {
    root: Option<RBNodeWrapper<K, V>>
}

#[derive(Debug)]
pub struct RBNodeWrapper<K, V> where K: Ord + std::fmt::Debug, V: std::fmt::Debug {
    rbnode: Rc<RefCell<RBNode<K, V>>>
}

#[derive(Debug)]
pub struct RBNode<K, V> where K: Ord + std::fmt::Debug, V: std::fmt::Debug {
    color: Color,
    key: K,
    value: V,
    left: Option<RBNodeWrapper<K, V>>,
    right: Option<RBNodeWrapper<K, V>>,
    parent: Option<RBNodeWrapper<K, V>>
}

impl<K, V> RBNodeWrapper<K, V> where K: Ord + std::fmt::Debug, V: std::fmt::Debug {
    fn from(rc: Rc<RefCell<RBNode<K, V>>>) -> Self {
        RBNodeWrapper{rbnode: rc}
    }

    fn insert(&mut self, k: K, v: V) -> RBNodeWrapper<K, V> {
        let new_node;
        let mut rbnode = self.rbnode.borrow_mut();
        match rbnode.key.cmp(&k) {
            Less | Equal => {
                match rbnode.right {
                    None => {
                        new_node = RBNode::new_rc(k, v);
                        new_node.borrow_mut().parent = Some(RBNodeWrapper::from(self.rbnode.clone()));
                        rbnode.right = Some(RBNodeWrapper::from(new_node.clone()));
                    },
                    Some(ref mut node) => {
                        new_node = node.insert(k, v).rbnode;
                    }
                };
            },
            Greater => {
                match rbnode.left {
                    None => {
                        new_node = RBNode::new_rc(k, v);
                        new_node.borrow_mut().parent = Some(RBNodeWrapper::from(self.rbnode.clone()));
                        rbnode.left = Some(RBNodeWrapper::from(new_node.clone()));
                    },
                    Some(ref mut node) => {
                        new_node = node.insert(k, v).rbnode;
                    }
                };
            },
        }
        let mut result = RBNodeWrapper::from(new_node);
        result.insert_fixup();
        result
    }

    fn insert_fixup(&mut self) {
    }
}

impl<K, V> RBTree<K, V> where K: Ord + std::fmt::Debug, V: std::fmt::Debug {
    pub fn new() -> Self {
        RBTree{ root: None }
    }

    pub fn insert(&mut self, k: K, v: V) {
        match self.root {
            None => self.root = Some(
                RBNodeWrapper::from(RBNode::new_rc(k, v))
            ),
            Some(ref mut node) => {
                node.insert(k, v);
            }
        }
    }
}

impl<K, V> RBNode<K, V> where K: Ord + std::fmt::Debug, V: std::fmt::Debug {
    fn new(k: K, v: V) -> Self {
        RBNode{
            color: Red,
            key: k,
            value: v,
            left: None,
            right: None,
            parent: None,
        }
    }

    fn new_rc(k: K, v: V) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::new(k, v)))
    }
}
