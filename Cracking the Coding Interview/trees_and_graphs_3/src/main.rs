/// List of Depths: Given a binary tree, design an algorithm which creates a linked list of all the nodes
/// at each depth (e.g., if you have a tree with depth D, you'll have D linked lists).
// This is my own implementation of a binary tree in rust
mod binary_tree;
use binary_tree::Node;
use std::collections::LinkedList;
use std::ops::Deref;

// This is my own implementation of a queue in rust
use std::borrow::BorrowMut;

fn main() {
    // construct a family tree for Ben solo
    let ben = get_family_tree();
    // use the function to get the linked lists for each depth level
    let mut vec = lists_of_depth(&ben);

    for q in &mut vec {
        println!("\nNext depth-level:");
        while let Some(name) = q.pop_front() {
            println!("{}", name);
        }
    }
}

/// This function solves the exercise. We do this in a recursive approach:
/// Make a linked list for the current nodes' children and then consolidate the lists that are
/// return by calling this function on the children.
pub fn lists_of_depth<'a>(root: &Node<&'a str>) ->LinkedList<LinkedList<&'a str>> {
    let mut result_vec: LinkedList<LinkedList<&str>> = LinkedList::new();
    result_vec.push_back(LinkedList::new());

    let mut left_vec: LinkedList<LinkedList<&str>> = LinkedList::new();
    let mut right_vec: LinkedList<LinkedList<&str>> = LinkedList::new();

    match root.get_left() {
        Some(x) => {
            result_vec.front_mut().unwrap().push_back(x.data);
            left_vec.append(lists_of_depth(x.deref()).borrow_mut());
        },
        None => {},
    }
    match root.get_right() {
        Some(x) => {
            result_vec.front_mut().unwrap().push_back(x.data);
            right_vec.append(lists_of_depth(x.deref()).borrow_mut());
        },
        None => {},
    }

    while let Some(mut q_left) = left_vec.pop_front() {
        if let Some(q_right) = right_vec.pop_front().as_mut() {
            q_left.append(q_right);
        }
        result_vec.push_back(q_left);
    }
    while let Some(q_right) = right_vec.pop_front() {
        result_vec.push_back(q_right);
    }

    result_vec
}

/// This creates an example binary tree to try the algorithm on
pub fn get_family_tree() -> Node<&'static str> {
    let mut ben= Node::new("Ben Solo");
    {
        ben.set_left("Leia Organa");
        ben.set_right("Han Solo");
        let mut leia = ben.get_left().unwrap();
        leia.set_left("Padmé Amidala");
        leia.set_right("Anakin Skywalker");

        let mut padme = leia.get_left().unwrap();
        padme.set_left("Ruwee Naberrie");
        padme.set_right("Jobal Naberrie");

        let mut anakin = leia.get_right().unwrap();
        anakin.set_left("Shmi Skywalker");
    }

    ben
}
