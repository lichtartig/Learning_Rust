/// Implement an algorithm to delete a node in the middle (i.e., any node but the first and last
/// node, not necessarily the exact middle) of a singly linked list, given only access to that node.

fn main() {
    // initialize and example of a LinkedList
    let mut my_list: LinkedList<i32> = LinkedList::new();

    my_list.push(5);
    my_list.push(7);
    my_list.push(3);
    my_list.push(9);
    my_list.push(1);
    my_list.push(4);

    delete_middle_note(my_list.get_ith_node(3));

    // 4 1 9 7 5
    while let Some(x) = my_list.pop() {
        println!("Popping {}", x);
    }
}

/// The function that does the job described above. It takes the node to be deleted and overwrites
/// it with the data from the node it points to. This way effectively deleting it.
fn delete_middle_note(node: &mut Node<i32>) {
    let nxt = node.next_node.take().unwrap();
    node.value = nxt.value;
    node.next_node = nxt.next_node;
}

/// This is my LinkedList class. Nothing fancy here.
type Link<T> = Option<Box<Node<T>>>;
struct LinkedList<T> {
    head: Link<T>,
}

// a single element of the stack
struct Node<T> {
    next_node: Link<T>,
    value: T,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList { head: None }
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

    pub fn get_ith_node(&mut self, index: usize) -> &mut Node<T> {
        let mut runner = self.head.as_mut();
        for _i in 0..(index) {
            runner = runner.unwrap().next_node.as_mut();
        }
        runner.unwrap()
    }
}
