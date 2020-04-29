#![feature(linked_list_cursors)]
/// Implement a function to check if a linked list is a palindrome, that is it looks the same from
/// the front and from the back.
/// Note: This has to be compiled with 'nightly'
use std::collections::LinkedList;
use std::io;

fn main() {
    // get the items of a linked list from the user
    let mut example_list: LinkedList<i32> = LinkedList::new();
    println!("Provide as many list elements (i32) as you want!");
    loop {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line.");
        match buffer.trim().parse() {
            Ok(x) => example_list.push_back(x),
            Err(_) => break,
        }
    }

    // check if linked list is palindrome
    if is_palindrom(example_list) {
        println!("Palindrome!!!");
    } else {
        println!("Example list is not a palindrome.");
    }
}

/// This is the function checking if the linked list is a palindrome. It does so by using two
/// iterators. One starting from the front and one starting from the back. At each iteration the
/// two values are compared until they meet in the middle.
fn is_palindrom(list: LinkedList<i32>) -> bool {
    // This is implemented as an O(1) method.
    let len = list.len().clone();

    let mut front_cursor = list.cursor_front();
    let mut back_cursor = list.cursor_back();

    for _i in 0..(len / 2) {
        if front_cursor.current() != back_cursor.current() {
            return false;
        }
        front_cursor.move_next();
        back_cursor.move_prev();
    }
    true
}
