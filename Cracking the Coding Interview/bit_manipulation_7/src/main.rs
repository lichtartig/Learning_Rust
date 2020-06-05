/// Pairwise Swap: Write a program to swap odd and even bits in an integer with as few instructions
/// as possible (e.g., bit 0 and bit 1 are swapped, bit 2 and bit 3 are swapped, and so on).

fn main() {
    let example = 0b11010;
    println!("original {:0b}", example);
    println!("swapped  {:0b}", swap_even_odd(example));

    let example = 0b11011110101;
    println!("original {:0b}", example);
    println!("swapped  {:0b}", swap_even_odd(example));
}

/// This performs the task: We split the even and odd parts of the number by applying two filters.
/// Than we shift and add them.
fn swap_even_odd(num: i32) -> i32{
    let even_filter = 0b0101010101010101010101010101010;
    let odd_filter = 0b1010101010101010101010101010101;
    ((num & even_filter) >> 1) +  ((num & odd_filter) << 1)
}