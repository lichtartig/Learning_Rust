/// Binary to String: Given a real number between O and 1 (e.g., 0.72) that is passed in as a double, print
/// the binary representation. If the number cannot be represented accurately in binary with at most 32
/// characters, print "ERROR!

fn main() {
    let a = 0.125;
    match conv_to_str(a) {
        None => println!("ERROR!"),
        Some(x) => println!("{}", x),
    }
}

fn conv_to_str(number: f32) -> Option<String>{
    let mut out = number;
    let mut shift = 0;
    for i in 0..32 {
        out *= 2.;
        if out - out.floor() == 0. && i == 31 {
            return None;
        } else if out - out.floor() == 0. {
            shift = i;
            break
        }
    }
    Some(format!("0.{}{:0b}", "0".repeat(shift), (out as i32) ))
}