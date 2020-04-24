const MAX: usize = 1000000;

fn main() {
    // initialize array first with zeros then write the index as each field's value
    let mut a = [0; MAX];
    for i in 0..MAX {
        a[i] = i
    }


    // the actual Erastothenes sieve
    for i in 2..MAX {
        if a[i] != 0 {
            println!("{}", i);
            for j in ((2*i)..MAX).step_by(i) {
               a[j] = 0;
            }
        }
    }
}

