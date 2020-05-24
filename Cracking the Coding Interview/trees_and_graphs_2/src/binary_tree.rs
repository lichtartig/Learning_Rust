/// This is my binary tree written in Rust.
use std::rc::Rc;
use std::cell::{RefCell, RefMut};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
pub struct Node<T> {
    left: Link<T>,
    right: Link<T>,
    pub data: T,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Node<T> {
        Node {
            left: None,
            right: None,
            data: data,
        }
    }

    pub fn set_left(&mut self, data: T) {
        self.left.replace(Rc::new(RefCell::new(Node::new(data))));
    }

    pub fn set_right(&mut self, data: T) {
        self.right.replace(Rc::new(RefCell::new(Node::new(data))));
    }

    pub fn set_left_node(&mut self, node: Node<T>) {
        self.left.replace(Rc::new(RefCell::new(node)));
    }

    pub fn set_right_node(&mut self, node: Node<T>) {
        self.right.replace(Rc::new(RefCell::new(node)));
    }

    pub fn get_left(&self) -> Option<RefMut<Node<T>>> {
        match &self.left {
            None => None,
            Some(x) => Some(x.borrow_mut()),
        }
    }

    pub fn get_right(&self) -> Option<RefMut<Node<T>>> {
        match &self.right {
            None => None,
            Some(x) => Some(x.borrow_mut()),
        }
    }
}