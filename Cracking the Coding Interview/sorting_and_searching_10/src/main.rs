/// Rank from Stream: Imagine you are reading in a stream of integers. Periodically, you wish to be
/// able to look up the rank of a number x (the number of values less than or equal to x). Implement
/// the data structures and algorithms to support these operations. That is, implement the method
/// track(int x), which is called when each number is generated, and the method
/// getRankOfNumber(int x), which returns the number of values less than or equal to x
/// (not including x itself).
use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::Borrow;

fn main() {
    let mut bst = BST::new();
    bst.append(5);
    bst.append(1);
    bst.append(4);
    bst.append(4);
    bst.append(5);
    bst.append(9);
    bst.append(13);
    bst.append(3);

    for i in 1..14 {
        match bst.get_rank_of_n(i) {
            None => println!("{} has not been in the integer stream so far.", i),
            Some(n) => println!("{} has rank {}", i, n),
        }
    }
}

type Link = Option<Rc<RefCell<Node>>>;
struct Node {
    left: Link,
    right: Link,
    number: i32,
    nodes_below: u32
}

impl Node {
    fn new(a: i32) -> Node {
        Node {
            left: None,
            right: None,
            number: a,
            nodes_below: 0
        }
    }

    fn append(&mut self, a: i32) {
        self.nodes_below += 1;
        if let Some(n) = &mut self.left {
            if RefCell::borrow(n).number >= a {
                n.borrow_mut().append(a);
                return;
            }
            if let Some(n) = &mut self.right {
                if RefCell::borrow(n).number >= a {
                    n.borrow_mut().append(a);
                    return;
                } else {
                    let m = Node {
                        left: Some(Rc::clone(n)),
                        right: None,
                        number: a,
                        nodes_below: 1 + RefCell::borrow(n).nodes_below,
                    };
                    self.right = Some(Rc::new(RefCell::new(m)));
                }
            } else {
                self.right = Some(Rc::new(RefCell::new(Node::new(a))));
            }
        } else {
            self.left = Some(Rc::new(RefCell::new(Node::new(a))));
        }
    }

    fn get_rank_of_n(&self, n: i32) -> Option<u32> {
        if self.borrow().number == n {
            return Some(self.nodes_below);
        }

        if let Some(m) = &self.left {
            if RefCell::borrow(m).number >= n {
                return RefCell::borrow(m).get_rank_of_n(n);
            }
            if let Some(p) = &self.right {
                if RefCell::borrow(p).number >= n {
                    return RefCell::borrow(p).get_rank_of_n(n);
                }
            }
        }
        return None;
    }
}

struct BST {
    root: Link
}

impl BST {
    fn new() -> BST {
        BST {
            root: None
        }
    }

    fn append(&mut self, a: i32) {
        if let Some(n) = &mut self.root {
            if RefCell::borrow(n).number >= a {
                n.borrow_mut().append(a);
            } else {
                let m = Node {
                    left: Some(Rc::clone(n)),
                    right: None,
                    number: a,
                    nodes_below: 1 + RefCell::borrow(n).nodes_below
                };
                self.root = Some(Rc::new(RefCell::new(m)));
            }
        } else {
            self.root = Some(Rc::new(RefCell::new(Node::new(a))));
        }
    }

    fn get_rank_of_n(&self, n: i32) -> Option<u32> {
        // search for number, then return its nodes_below value
        match &self.root {
            None => return None,
            Some(p) => return RefCell::borrow(p).get_rank_of_n(n),
        }
    }
}