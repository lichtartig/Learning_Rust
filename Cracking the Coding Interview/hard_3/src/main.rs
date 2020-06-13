/// Random Set: Write a method to randomly generate a set of m integers from an array of size n.
/// Each element must have equal probability of being chosen.
use rand::Rng;

fn main() {
    let arr = vec![1, -15, 2, 7, -3, 5];
    let deck = RandSeq::new(&arr);
    for number in deck.get_rand_perm(3) {
        println!("Next number: {}", number);
    }
}

struct RandSeq {
    array: Vec<i32>
}

impl RandSeq {
    fn new(v: &[i32]) -> RandSeq {
        RandSeq {
            array: v.to_vec()
        }
    }

    fn get_rand_perm(&self, k: usize) -> Vec<i32> {
        let mut buffer = self.array.clone();
        let mut ret = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..k {
            let index: usize = rng.gen_range(0, buffer.len());
            ret.push(*buffer.get(index).unwrap());
            buffer.remove(index);
        }

        ret
    }
}