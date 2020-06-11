/// Towers of Hanoi: In the classic problem of the Towers of Hanoi, you have 3 towers and N disks of
/// different sizes which can slide onto any tower. The puzzle starts with disks sorted in ascending
/// order of size from top to bottom (i.e., each disk sits on top of an even larger one). You have
/// the following constraints:
/// (1) Only one disk can be moved at a time.
/// (2) A disk is slid off the top of one tower onto another tower.
/// (3) A disk cannot be placed on top of a smaller disk.
/// Write a program to move the disks from the first tower to the last using stacks.
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let n = 3;
    let mut ht = HanoiTowers::new(n);
    ht.solve_tower(n, true);
}

struct HanoiTowers {
    a: Stack,
    b: Stack,
    c: Stack
}

impl HanoiTowers {
    fn new(n: u32) -> HanoiTowers {
        let mut ht = HanoiTowers {
            a: Stack::new(),
            b: Stack::new(),
            c: Stack::new()
        };

        for i in 0..n {
            ht.a.push(n-i);
        }
        ht
    }

    fn print_stacks(&self) {
        println!("Current stacks:");
        self.a.print();
        self.b.print();
        self.c.print();
    }

    fn solve_tower(&mut self, n: u32, direction: bool) {
        if direction {
            if n == 1 {
                self.c.push(self.a.pop().unwrap());
                self.print_stacks();
            } else {
                self.solve_tower(n-1, direction);
                self.b.push(self.a.pop().unwrap());
                self.print_stacks();
                self.solve_tower(n-1, !direction);
                self.c.push(self.b.pop().unwrap());
                self.print_stacks();
                self.solve_tower(n-1, direction);
            }
        } else {
            if n == 1 {
                self.a.push(self.c.pop().unwrap());
                self.print_stacks();
            } else {
                self.solve_tower(n-1, direction);
                self.b.push(self.c.pop().unwrap());
                self.print_stacks();
                self.solve_tower(n-1, !direction);
                self.a.push(self.b.pop().unwrap());
                self.print_stacks();
                self.solve_tower(n-1, direction);
            }
        }
    }
}

type Link = Option<Rc<RefCell<Node>>>;
struct Node {
    next: Link,
    value: u32,
}

impl Node {
    fn new(a: u32) -> Node {
        Node {
            next: None,
            value: a
        }
    }

    fn get_str(&self) -> String {
        if let Some(n) = &self.next {
            let mut s = RefCell::borrow(n).get_str();
            s += &self.value.to_string();
            return s;
        } else {
            return self.value.to_string();
        }
    }
}

struct Stack {
    head: Link
}

impl Stack {
    fn new() -> Stack {
        Stack {
            head: None,
        }
    }

    fn push(&mut self, a: u32) {
        if let Some(n) = &self.head {
            let m = Node {
                next: Some(Rc::clone(n)),
                value: a
            };
            self.head = Some(Rc::new(RefCell::new(m)));
        } else {
            self.head = Some(Rc::new(RefCell::new(Node::new(a))));
        }
    }

    fn pop(&mut self) -> Option<u32> {
        match self.head.take() {
            None => None,
            Some(n) => {
                let ret: u32 = RefCell::borrow(&n).value.clone();
                self.head = n.borrow_mut().next.take();
                Some(ret)
            }
        }
    }

    fn print(&self) {
        if let Some(n) = &self.head {
            println!("{}", RefCell::borrow(n).get_str())
        } else {
            println!("");
        }
    }
}