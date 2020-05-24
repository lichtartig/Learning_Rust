/// Validate BST: Implement a function to check if a binary tree is a binary search tree.
// This is my binary tree class
mod binary_tree;
use binary_tree::Node;

fn main() {
    // Note that ordering is defined on &str objects by alphabetic ordering.
    let ben = get_family_tree();
    let germany = get_alphabetic_countries();

    println!("Is Ben's family tree a binary search tree? {}", is_search_tree(&ben));
    println!("Is the alphabetic country tree a binary search tree? {}", is_search_tree(&germany));
}

/// This function checks if a binary tree is a binary search tree.
/// We do this in a recursive way: Check if the current node satisfies the ordering and if yes,
/// return the logical AND of the results of is_search_tree() on the two children.
/// If a node has only one child, it is regarded as ordered.
pub fn is_search_tree<T: Ord>(root: &Node<T>) -> bool {
    // check if both nodes are non-empty. Otherwise return true.
    if let (Some(n), Some(m)) = (root.get_left(), root.get_right()) {
        if n.data < m.data {
            return is_search_tree(&*n) & is_search_tree(&*m);
        } else {
            return false;
        }
    } else {
        return true;
    }
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
pub fn get_alphabetic_countries() -> Node<&'static str> {
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