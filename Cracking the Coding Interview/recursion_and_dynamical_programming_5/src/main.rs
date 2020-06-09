/// Recursive Multiply: Write a recursive function to multiply two positive integers without using
/// the *operator.You can use addition, subtraction, and bit shifting, but you should minimize the
/// number of those operations.

fn main() {
    println!("{}", recursive_multiply(5, 9));
}

fn recursive_multiply(a: i32, b: i32) -> i32 {
    if b == 1 {
        return a
    } else if b & (b-1) == 0 {
        return recursive_multiply(a << 1, b >> 1);
    } else {
        return a + recursive_multiply(a, b-1)
    }
}