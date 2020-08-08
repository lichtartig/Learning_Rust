fn main() {
    let a = 5;
    let b = 7;
    println!("{}, {}", get_bit(a, 1), get_bit(b, 1));
    println!("{}", set_bit(a, 1));
    println!("{}", clear_bit(b, 1));
}

pub fn get_bit(num: i32, i: i32) -> bool {
    (num & (1 << i)) != 0
}

pub fn set_bit(num: i32, i: i32) -> i32 {
    num | (1 << i)
}

pub fn clear_bit(num: i32, i: i32) -> i32 {
    num & !(1 << i)
}