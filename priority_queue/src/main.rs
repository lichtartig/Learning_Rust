/// This is an implementation of a priority queue using a modified binary search tree

fn main() {
    let mut q = PriorityQueue::new();
    q.push("clean", 0);
    q.push("eat", 2);
    q.push("study", 4);
    q.push("exercise", 3);

    while let Some(s) = q.pop() {
        println!("{}", s);
    }
}

type Link<T> = Option<Box<Node<T>>>;
/// The following struct implements a single node in the BST. Apart from a priority, it holds data
/// of a template type T
struct Node<T> {
    priority: u32,
    data: T,
    left: Link<T>,
    right: Link<T>
}

impl<T> Node<T> {
    fn new(data: T, priority: u32) -> Node<T> {
        Node {
            priority: priority,
            data: data,
            left: None,
            right: None
        }
    }

    fn push_box(&mut self, n: Box<Node<T>>) {
        match (&self.left, &self.right) {
            (None, _) => self.left = Some(n),
            (Some(x), _) if x.priority >= n.priority => self.left.as_mut().unwrap().push_box(n),
            (Some(x), None) if x.priority < n.priority => self.right = Some(n),
            (Some(_), Some(y)) if y.priority >= n.priority => self.right.as_mut().unwrap().push_box(n),
            (_, _) => {
                let tmp = self.right.take().unwrap();
                self.right = Some(n);
                self.right.as_mut().unwrap().push_box(tmp);
            }
        };
    }
}

/// This is the priority queue it has the two methods push and pop as would be expected.
struct PriorityQueue<T> {
    head: Link<T>
}

impl<T> PriorityQueue<T> {
    fn new() -> PriorityQueue<T> {
        PriorityQueue {
            head: None
        }
    }

    fn push(&mut self, data: T, priority: u32) {
        if let Some(h) = &mut self.head {
            if h.priority >= priority {
                h.push_box(Box::new(Node::new(data, priority)));
            } else {
                let tmp = Node {
                    priority: priority,
                    data: data,
                    left: self.head.take(),
                    right: None
                };
                self.head = Some(Box::new(tmp));
            }
        } else {
            self.head = Some(Box::new(Node::new(data, priority)));
        }
    }

    fn pop(&mut self) -> Option<T> {
        if let Some(mut n) = self.head.take() {
            match (&n.left, &n.right) {
                (None, None) => {},
                (None, Some(_)) => self.head = n.right.take(),
                (Some(_), None) => self.head = n.left.take(),
                (Some(x), Some(y)) if x.priority >= y.priority => {
                    n.left.as_mut().unwrap().push_box(n.right.take().unwrap());
                    self.head = n.left.take();
                },
                (Some(_), Some(_)) => {
                    n.right.as_mut().unwrap().push_box(n.left.take().unwrap());
                    self.head = n.right.take();
                }
            }
            return Some(n.data);
        } else {
            return None;
        }
    }
}