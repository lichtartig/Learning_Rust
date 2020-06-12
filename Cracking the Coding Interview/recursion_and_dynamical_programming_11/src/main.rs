/// Coins: Given an infinite number of quarters (25 cents), dimes (10 cents), nickels (5 cents), and
/// pennies (1 cent), write code to calculate the number of ways of representing n cents.
use std::collections::HashMap;

fn main() {
    let mut c = CoinReps::new();
    let v = c.get_all_coin_dist(25,25);
    for e in &v {
        println!("{}, {}, {}, {}", e.0, e.1, e.2, e.3);
    }
}

type CoinDist = (u32, u32, u32, u32);
struct CoinReps {
    hm: HashMap<u32, Vec<CoinDist>>
}

impl CoinReps {
    fn new() -> CoinReps {
        let mut hm = HashMap::new();
        hm.insert(0, vec![(0, 0, 0, 0)]);
        hm.insert(1, vec![(1, 0, 0, 0)]);
        hm.insert(2, vec![(2, 0, 0, 0)]);
        hm.insert(3, vec![(3, 0, 0, 0)]);
        hm.insert(4, vec![(4, 0, 0, 0)]);
        CoinReps {
            hm: hm
        }
    }

    fn get_all_coin_dist(&mut self, n: u32, m: u32) -> Vec<CoinDist> {
        if let Some(v) = self.hm.get(&n) {
            return v.clone();
        } else if m == 1 {
            return vec![(n, 0, 0, 0)];
        }

        let mut ret = Vec::new();
        for i in 0..(n/m + 1) {
            let (next_m, add) = match m {
                25 => (10, (0, 0, 0, i)),
                10 => (5, (0, 0, i, 0)),
                _ => (1, (0, i, 0, 0)),
            };
            for e in self.get_all_coin_dist(n - i * m, next_m) {
                ret.push((e.0 + add.0, e.1 + add.1, e.2 + add.2, e.3 + add.3))
            }
        }
        self.hm.insert(n, ret.clone());
        ret
    }

}