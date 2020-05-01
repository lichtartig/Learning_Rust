/// Write a program to sort a stack such that the smallest items are on the top. You can use an
/// additional temporary stack, but you may not copy the elements into any other data structure
/// (such as an array). The stack supports the following operations: push, pop, peek, and is Empty.

fn main() {
    let mut my_stack: Stack<i32> = Stack::new();

    my_stack.push(5);
    my_stack.push(6);
    my_stack.push(2);
    my_stack.push(7);
    my_stack.push(1);

    sort_stack(&mut my_stack);

    while let Some(x) = my_stack.pop() {
        println!("{}", x);
    }
}

/// This function sorts the stack using a secondary stack. It works as follows:
/// (i)     Pop the top item in the stack. If the top item in the secondary stack is less or equal
///         (or doesn't exist). Push the item to the secondary step.
/// (ii)    Otherwise pop items from the secondary stack and push them to the primary stack until
///         (i) is satisfied.
/// (iii)   Pop the entire secondary stack and push to the main stack.
fn sort_stack(stack: &mut Stack<i32>) {
    let mut tmp_stack: Stack<i32> = Stack::new();

    while let Some(x) = stack.pop() {
        while let Some(y) = tmp_stack.peek() {
            if y > x {
                stack.push(tmp_stack.pop().unwrap());
            } else {
                break;
            }
        }
        tmp_stack.push(x);
    }

    while let Some(y) = tmp_stack.pop() {
        stack.push(y);
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

impl<T: Copy> Stack<T> {
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

    pub fn peek(&self) -> Option<T> {
        self.head.as_ref().map(|x| x.value)
    }
}
