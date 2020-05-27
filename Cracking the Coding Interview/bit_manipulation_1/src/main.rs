/// Insertion: You are given two 32-bit numbers, N and M, and two bit positions, i and
/// j. Write a method to insert M into N such that M starts at bit j and ends at bit i. You
/// can assume that the bits j through i have enough space to fit all of M. That is, if
/// M = 10011, you can assume that there are at least 5 bits between j and i. You would not, for
/// example, have j = 3 and i = 2, because M could not fully fit between bit 3 and bit 2

fn main() {
    // The example from the book
    let n = 0b10000000000;
    let m = 0b10011;
    println!("{:0b}", insert_m(n, m, 2));
    // does it work if there are ones before inserting m? yes!
    let n = 0b10000000100;
    println!("{:0b}", insert_m(n, m, 2));
}

pub fn insert_m(n:i32, m: i32, i: i32) -> i32 {
    n | (m << i) ^ n
}