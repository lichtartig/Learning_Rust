///Minimal Tree: Given a sorted (increasing order) array with unique integer elements, write an
/// algorithm to create a binary search tree with minimal height.
// This is my binary tree class
mod binary_tree;
use binary_tree::Node;

fn main() {
    // take as an example the numbers from 0 to 9
    let example_list: Vec<i32> = (0..10).collect();

    // construct the binary search tree
    let bin_search_tree = make_search_tree(&example_list);

    // root node
    assert_eq!(bin_search_tree.data, 5);

    // first level
    assert_eq!(bin_search_tree.get_left().unwrap().data, 2);
    assert_eq!(bin_search_tree.get_right().unwrap().data, 8);

    // second level
    assert_eq!(bin_search_tree.get_left().unwrap().get_left().unwrap().data, 1);
    assert_eq!(bin_search_tree.get_left().unwrap().get_right().unwrap().data, 4);
    assert_eq!(bin_search_tree.get_right().unwrap().get_left().unwrap().data, 7);
    assert_eq!(bin_search_tree.get_right().unwrap().get_right().unwrap().data, 9);

    // third level
    assert_eq!(bin_search_tree.get_left().unwrap().get_left().unwrap().get_left().unwrap().data, 0);
    assert_eq!(bin_search_tree.get_left().unwrap().get_right().unwrap().get_left().unwrap().data, 3);
    assert_eq!(bin_search_tree.get_right().unwrap().get_left().unwrap().get_left().unwrap().data, 6);
}


/// This is the function constructing the binary search tree. We do this in a recursive mode:
/// 1) Take the middle element of the input and create a node with it.
/// 2) set its left element with make_search_tree(all elements before middle)
/// 3) set its right element with make_search_tree(all elements after middle)
/// 4) return
pub fn make_search_tree<T: Ord + Copy>(elements: &[T]) -> Node<T> {
    let pivot = elements.len() / 2;
    let mut ret = Node::new(elements[pivot]);
    if pivot > 0 {
        ret.set_left_node(make_search_tree(&elements[..pivot]));
    }
    if pivot + 1 < elements.len() {
        ret.set_right_node(make_search_tree(&elements[(pivot+1)..]));
    }
    return ret;
}