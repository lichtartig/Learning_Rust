/// Add Without Plus: Write a function that adds two numbers. You should not use+ or any arithmetic
/// operators.

fn main() {
    let a = 3;
    let b = 31;
    println!("{}+{}={}", a, b, add(a, b));
}

/// We implement the exercise in a recursive way:
fn add(a: i32, b: i32) -> i32 {
    match b {
        0 => a,
        _ => add(a ^ b, (a & b) << 1)
    }
}