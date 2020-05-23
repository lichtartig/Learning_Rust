/// Check Balanced: Implement a function to check if a binary tree is balanced. For the purposes of
/// this question, a balanced tree is defined to be a tree such that the heights of the two subtrees
/// of any node never differ by more than one.

// This is my own implementation of a binary tree in rust
mod binary_tree;
use binary_tree::Node;
use std::borrow::Borrow;
use std::cmp::max;

fn main() {
    // Example 1: construct a family tree for Ben Solo
    let ben = get_family_tree();
    let (height, balanced) = tree_height_and_balanced(&ben);
    println!("Ben Solo's binary family tree has height: {}", height);
    match balanced {
        true => println!("It is balanced."),
        false => println!("It is not balanced."),
    }

    // Example 2: a binary search tree with some bordering countries
    let abc = alphabetic_countries();
    let (height, balanced) = tree_height_and_balanced(&abc);
    println!("The country tree has height: {}", height);
    match balanced {
        true => println!("It is balanced."),
        false => println!("It is not balanced."),
    }
}

/// This is the function that performs the task described above. We do this in a recursive approach.
pub fn tree_height_and_balanced(root: &Node<&str>) -> (i32, bool)  {
    let left_res: (i32, bool);
    let right_res: (i32, bool);

    match root.get_left() {
        None => left_res = (0, true),
        Some(left) => left_res = tree_height_and_balanced(left.borrow()),
    }

    match root.get_right() {
        None => right_res = (0, true),
        Some(right) => right_res = tree_height_and_balanced(right.borrow()),
    }
    let diff = left_res.0 - right_res.0;
    (max(left_res.0, right_res.0)+1, left_res.1 & right_res.1 & (diff < 2))
}

/// This creates an example binary tree to try the algorithm on (it has height 4 and is not balanced)
pub fn get_family_tree() -> Node<&'static str> {
    let mut ben= Node::new("Ben Solo");
    ben.set_left("Leia Organa");
    ben.set_right("Han Solo");
    {
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

/// This creates a second example binary tree to try the algorithm on (it has height 4 and is not balanced)
pub fn alphabetic_countries() -> Node<&'static str> {
    let mut germany= Node::new("Germany");
    germany.set_left("France");
    germany.set_right("Poland");
    {
        let mut france = germany.get_left().unwrap();
        france.set_left("Belgium");
        france.set_right("Italy");
    }
    {
        let mut poland = germany.get_left().unwrap();
        poland.set_left("Lithuania");
        poland.set_right("Ukraine");
    }

    germany
}