/// Count of 2s: Write a method to count the number of 2s that appear in all the numbers between O
/// and n (inclusive).

fn main() {
    for n in 0..30 {
        println!("n = {}, number of 2's = {}", n, number_of_twos(n));
    }

    for n in vec![100, 200, 1000, 2000] {
        println!("n = {}, number of 2's = {}", n, number_of_twos(n));
    }

}

fn number_of_twos(n: u32) -> u32 {
    let mut num = 0;

    for i in 0..6 {
        if (10 as u32).pow(i) > n {
            return num;
        }

        let digit = n % (10 as u32).pow(i+1) / (10 as u32).pow(i);
        num += digit * two_in_powers_of_ten(i);
        if digit >= 2 {
            num += 1;
        }
    }
    num
}

fn two_in_powers_of_ten(p: u32) -> u32 {
    match p {
        0 => 0,
        _ => 10 * two_in_powers_of_ten(p-1) + 1
    }
}