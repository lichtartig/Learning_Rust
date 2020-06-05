/// Next Number: Given a positive integer, print the next smallest and the next largest number that
/// have the same number of 1 bits in their binary representation.

fn main() {
    let a = 0b11011101111;
    println!("\n{:0b}", a);
    match next_largest(a) {
        None => println!("There is no larger i32 number w/ the same number of 1-bits"),
        Some(x) => println!("{:0b}", x),
    }
    match next_smallest(a) {
        None => println!("There is no smaller number w/ the same number of 1-bits"),
        Some(x) => println!("{:0b}", x),
    }

    let a = 0b11011111110;
    println!("\n{:0b}", a);
    match next_largest(a) {
        None => println!("There is no larger i32 number w/ the same number of 1-bits"),
        Some(x) => println!("{:0b}", x),
    }
    match next_smallest(a) {
        None => println!("There is no smaller number w/ the same number of 1-bits"),
        Some(x) => println!("{:0b}", x),
    }
}

/// find next largest number with same number of 1-bits
fn next_largest(num: i32) -> Option<i32>{
    for i in 0..31 {
        if get_bit(num, i) && !get_bit(num, i+1) {
            let ret = del_bit(num, i);
            return Some(set_bit(ret, i+1));
        }
    }
    None
}

/// find next smallest number with same number of 1-bits
fn next_smallest(num: i32) -> Option<i32> {
    for i in 1..32 {
        if get_bit(num, i) && !get_bit(num, i-1) {
            let ret = del_bit(num, i);
            return Some(set_bit(ret, i-1));
        }
    }
    None
}

fn get_bit(num: i32, i: i32) -> bool {
    num & (1 << i) != 0
}

fn set_bit(num: i32, i: i32) -> i32 {
    num | (1 << i)
}

fn del_bit(num: i32, i: i32) -> i32{
    num ^ (1 << i)
}