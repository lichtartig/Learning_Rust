/// You have two numbers represented by a linked list, where each node contains a single digit. The
/// digits are stored in reverse order, such that the 1 's digit is at the head of the list. Write a
/// function that adds the two numbers and returns the sum as a linked list
use std::collections::LinkedList;
use std::io;

fn main() {
    // get the two numbers to be added
    println!("Please provide the two numbers to be added");
    let mut number_a = get_input_number();
    let mut number_b = get_input_number();

    // add the two numbers and print out the resulting linked list
    let result = add_linked_lists(&mut number_a, &mut number_b);
    for i in result.iter() {
        println!("{}", i)
    }
}

/// This function performs the addition. Since we already have the numbers in the form of linked
/// lists, we perform the addition by advancing digit by digit (carrying the 1 if necessary)
/// We also pay attention to numbers of different lengths
fn add_linked_lists(
    number_a: &mut LinkedList<u32>,
    number_b: &mut LinkedList<u32>,
) -> LinkedList<u32> {
    let mut ret: LinkedList<u32> = LinkedList::new();
    let mut digit_a = number_a.iter();
    let mut digit_b = number_b.iter();
    let mut carry: u32 = 0;

    loop {
        match (digit_a.next(), digit_b.next()) {
            (None, None) => break,
            (Some(&x), None) => {
                ret.push_back((x + carry) % 10);
                carry = (x + carry) / 10;
            }
            (None, Some(&y)) => {
                ret.push_back((y + carry) % 10);
                carry = (y + carry) / 10;
            }
            (Some(&x), Some(&y)) => {
                ret.push_back((x + y + carry) % 10);
                carry = (x + y + carry) / 10;
            }
        }
    }
    ret
}

/// This function just gets a number from the user and returns it as a linked list described above
fn get_input_number() -> LinkedList<u32> {
    // get a number from the user
    let mut buffer: String = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");

    // iterate over characters and append them to the return list as described above.
    // pressing enter is interpreted as 0
    let mut ret: LinkedList<u32> = LinkedList::new();
    ret.push_front(0);
    for c in buffer.trim().chars() {
        match c.to_digit(10) {
            None => return get_input_number(),
            Some(x) => ret.push_front(x),
        }
    }
    ret
}
