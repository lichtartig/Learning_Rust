/// Imagine a (literal) stack of plates. If the stack gets too high, it might topple.
/// Therefore, in real life, we would likely start a new stack when the previous stack exceeds some
/// threshold. Implement a data structure SetOfStacks that mimics this. SetOfStacks should be
/// composed of several stacks and should create a new stack once the previous one exceeds capacity.
/// SetOfStacks. push() and SetOfStacks. pop() should behave identically to a single stack
/// (that is, pop () should return the same values as it would if there were just a single stack).
/// FOLLOW UP
/// Implement a function popAt ( int index) which performs a pop operation on a specific sub-stack.

fn main() {
    let mut my_set_of_stacks: SetOfStacks<i32> = SetOfStacks::new(2);
    my_set_of_stacks.push(5);
    println!("#stacks: {}", my_set_of_stacks.stacks.len());
    my_set_of_stacks.push(3);
    println!("#stacks: {}", my_set_of_stacks.stacks.len());
    my_set_of_stacks.push(2);
    println!("#stacks: {}", my_set_of_stacks.stacks.len());
    my_set_of_stacks.push(4);
    println!("#stacks: {}", my_set_of_stacks.stacks.len());
    my_set_of_stacks.push(2);
    println!("#stacks: {}", my_set_of_stacks.stacks.len());

    println!("Pop at stack 1: {}", my_set_of_stacks.pop_at(1).unwrap());

    while let Some(x) = my_set_of_stacks.pop() {
        println!("Popped value: {}", x);
        println!("#stacks: {}", my_set_of_stacks.stacks.len());
    }
}

/// This is the SetOfStacks struct. Because of the FOLLOW-UP question we store the stacks in a
/// vector of stacks. Without this question we could actually use a stack of stacks.
struct SetOfStacks<T> {
    capacity: usize,
    stacks: Vec<Stack<T>>,
}

impl<T> SetOfStacks<T> {
    /// A constructor. Nothing special here.
    pub fn new(capacity: usize) -> SetOfStacks<T> {
        SetOfStacks {
            capacity: capacity,
            stacks: Vec::new(),
        }
    }

    /// Checks if there is a stack to begin with. Otherwise create one.
    /// Checks if the newest stack still has capacity and if yes pushes to it.
    /// Otherwise it pushes to a new stack.
    pub fn push(&mut self, value: T) {
        if self.stacks.last().is_none() {
            self.stacks.push(Stack::new());
        }

        if self.stacks.last().unwrap().len() >= self.capacity {
            self.stacks.push(Stack::new());
        }
        self.stacks.last_mut().unwrap().push(value);
    }

    /// Checks if there is a stack to begin with. If not, return None.
    /// If yes: Pop from it. If it it's empty, destroy the stack and recursively return the value
    /// of self.pop()
    pub fn pop(&mut self) -> Option<T> {
        match self.stacks.last_mut() {
            None => return None,
            Some(x) => match x.pop() {
                Some(y) => return Some(y),
                None => {
                    self.stacks.pop();
                    return self.pop();
                }
            },
        }
    }

    /// Check if index is in range, otherwise return None. If yes pop at the requested stack.
    pub fn pop_at(&mut self, index: usize) -> Option<T> {
        if index < self.stacks.len() {
            return self.stacks[index].pop();
        } else {
            return None;
        }
    }
}

/// This is my Stack class. The only thing non-standard here, is that it has a length counter that
/// is updated with every push and pop operation that can be read out in O(1) by self.len().
type Link<T> = Option<Box<Node<T>>>;
struct Stack<T> {
    head: Link<T>,
    length: usize,
}

// a single element of the stack
struct Node<T> {
    next_node: Link<T>,
    value: T,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            head: None,
            length: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        let new_link = Box::new(Node {
            next_node: self.head.take(),
            value: value,
        });
        self.head = Some(new_link);
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next_node;
            self.length -= 1;
            node.value
        })
    }

    pub fn len(&self) -> usize {
        self.length
    }
}
