/// Implement a MyQueue class which implements a queue using two stacks.

fn main() {
    let mut my_queue: Queue<i32> = Queue::new();

    my_queue.push(5);
    my_queue.push(4);
    my_queue.push(3);
    println!("{}", my_queue.pop().unwrap()); // 5
    println!("{}", my_queue.pop().unwrap()); // 4
    my_queue.push(7);
    println!("{}", my_queue.pop().unwrap()); // 3
    my_queue.push(9);
    println!("{}", my_queue.pop().unwrap()); // 7
    println!("{}", my_queue.pop().unwrap()); // 9
}

/// The Queue struct consisting of two stacks. We do things as follows:
/// (i) If a new item is pushed on the queue, we push it on the 'push_stack' (push_back)
/// (ii) If an item is popped from the queue (at the front), we check if the pop_stack has elements.
/// If so, we just pop the top element from it. If not, we call 'restack'
/// (iii) 'restack' iteratively pops all elements from the 'push_stack' and pushes them on the
/// 'pop stack'. This reverses the order from LIFO to FIFO.
struct Queue<T> {
    push_stack: Stack<T>,
    pop_stack: Stack<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            push_stack: Stack::new(),
            pop_stack: Stack::new(),
        }
    }

    pub fn push(&mut self, value: T) {
        self.push_stack.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.pop_stack.pop() {
            None => {
                self.restack();
                return self.pop_stack.pop();
            }
            Some(x) => return Some(x),
        }
    }

    fn restack(&mut self) {
        while let Some(x) = self.push_stack.pop() {
            self.pop_stack.push(x);
        }
    }
}

/// This is my Stack class. Nothing fancy here.
type Link<T> = Option<Box<Node<T>>>;
struct Stack<T> {
    head: Link<T>,
}

// a single element of the stack
struct Node<T> {
    next_node: Link<T>,
    value: T,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { head: None }
    }

    pub fn push(&mut self, value: T) {
        let new_link = Box::new(Node {
            next_node: self.head.take(),
            value: value,
        });
        self.head = Some(new_link);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next_node;
            node.value
        })
    }
}
