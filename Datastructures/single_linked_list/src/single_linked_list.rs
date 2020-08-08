/// This is a single linked list. It also implements pop_front and push_front and thereby serves as
/// a stack.

type Link<T> = Option<Box<Node<T>>>;
pub struct Node<T> {
    next: Link<T>,
    value: T,
}

pub struct SingleLinkedList<T> {
    head: Link<T>,
}

impl<T: Copy> SingleLinkedList<T> {
    pub fn new() -> SingleLinkedList<T> {
        SingleLinkedList {
            head: None,
        }
    }

    /// Get the i-th element of the linked list.
    pub fn get(&self, i: usize) -> Option<T> {
        match self.get_node(i) {
            None => None,
            Some(n) => Some(n.value),
        }
    }

    /// Delete the i-th element of the list.
    pub fn delete(&mut self, i: usize) -> Result<T, &str> {
        // handle the case that there is no previous element to change
        if i == 0 {
            match self.pop_front() {
                None => return Err("Index exceeds list bounds."),
                Some(x) => return Ok(x),
            }
        }

        let prev_link = self.get_node_mut(i-1);
        if let Some(prev) = prev_link.as_mut() {
            if let Some(to_delete) = prev.next.take().as_mut() {
                prev.next = to_delete.next.take();
                Ok(to_delete.value)
            } else {
                Err("Index exceeds list bounds.")
            }
        } else {
            Err("Index exceeds list bounds.")
        }
    }

    /// Insert and element at the i-th position of the list and move the remaining elements back.
    pub fn insert(&mut self, data: T, i: usize) -> Result<bool, &str> {
        // handle the case that there is no previous element to change
        if i == 0 {
            self.push_front(data);
            return Ok(true);
        }

        let prev_link = self.get_node_mut(i-1);
        if let Some(prev) = prev_link.as_mut() {
            let new_node = Node {
                value: data,
                next: prev.next.take(),
            };
            prev.next = Some(Box::new(new_node));
            Ok(true)
        } else {
            Err("Index exceeds list bounds.")
        }
    }

    /// Pop and element at the front and return it.
    pub fn pop_front(&mut self) -> Option<T> {
        let mut old_head = self.head.take();
        match &mut old_head {
            None => None,
            Some(n) => {
                self.head = n.next.take();
                Some(n.value)
            }
        }
    }

    /// Create a new node in the linked list containing the 'data' and insert it at the front.
    pub fn push_front(&mut self, data: T) {
        let old_head = self.head.take();
        self.head = Some(Box::new(Node{
            next: old_head,
            value: data,
        }));
    }

    /// private function that returns a reference to the i-th element.
    fn get_node(&self, i: usize) -> &Link<T> {
        let mut curr = &self.head;
        let mut index = i;
        while index > 0 {
            match curr {
                None => break,
                Some(n) => curr = &n.next,
            }
            index -= 1;
        }
        return curr;
    }

    /// private function that returns a mutable reference to the i-th element.
    fn get_node_mut(&mut self, i: usize) -> &mut Link<T> {
        let mut curr = &mut self.head;
        let mut index = i;
        while index > 0 {
            match curr {
                None => break,
                Some(n) => curr = &mut n.next,
            }
            index -= 1;
        }
        return curr;
    }
}