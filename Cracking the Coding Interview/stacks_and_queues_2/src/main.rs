/// How would you design a stack which, in addition to push and pop, has a function min which
/// returns the minimum element? Push, pop and min should all operate in 0(1) time.
use std::rc::Rc;

fn main() {
    let mut my_stack: Stack<i32> = Stack::new();
    my_stack.push(5);
    my_stack.push(3);
    my_stack.push(2);
    my_stack.push(4);

    println!("Minimal value: {}", my_stack.min().unwrap());

    println!("Popping stack:");
    while let Some(x) = my_stack.pop() {
        println!("{}", x);
        match my_stack.min() {
            None => println!("No values left."),
            Some(x) => println!("Minimal value: {}", x),
        }
    }
}

/// Here we implement our own stack. We solve the problem by keeping two pointers in the stack
/// struct: (i) 'head' is the usual pointer to the head of the stack.
/// (ii) 'min' is a pointer to the minimal element that is updated if elements are pushed or popped.
type Link<T> = Option<Rc<Node<T>>>;
struct Stack<T> {
    head: Link<T>,
    min: Link<T>,
}

/// A single node in the stack
struct Node<T> {
    next_node: Link<T>,
    value: T,
}

/// Demand that T has the Copy and the PartialOrd trait
impl<T: Copy + PartialOrd> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            head: None,
            min: None,
        }
    }

    pub fn push(&mut self, value: T) {
        // standard pushing operations
        let new_link = Rc::new(Node {
            next_node: self.head.clone(),
            value: value,
        });
        self.head = Some(new_link);

        // updating the min pointer if necessary
        match self.min.clone() {
            None => self.min = self.head.clone(),
            Some(x) => {
                if value < x.value {
                    self.min = self.head.clone();
                }
            }
        };
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next_node.clone();

            // we check if the popped element is the current self.min and if yes, look for a new one
            if  node.value == self.min.as_ref().unwrap().value {
                self.find_new_min();
            }
            node.value
        })
    }

    // an O(1) operation returning the minimal value
    pub fn min(&self) -> Option<T> {
        match self.min.as_ref() {
            None => return None,
            Some(x) => return Some(x.value),
        }
    }

    // a method that updates self.min after the current one has been popped.
    fn find_new_min(&mut self) {
        let mut runner = self.head.clone();
        self.min = self.head.clone();
        loop {
            match runner {
                None => break,
                Some(x) => {
                    if x.value < self.min.as_ref().unwrap().value {
                        self.min = Some(x.clone());
                    }
                    runner = x.next_node.clone();
                }
            }
        }
    }
}