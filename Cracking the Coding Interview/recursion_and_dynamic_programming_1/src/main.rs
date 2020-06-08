/// Triple Step: A child is running up a staircase with n steps and can hop either 1 step, 2 steps,
/// or 3 steps at a time. Implement a method to count how many possible ways the child can run up
/// the stairs.
use std::collections::HashMap;

fn main() {
    let mut poss_counter = PossibilityCounter::new();
    for i in 0..10 {
        println!("{} steps: {}", i, poss_counter.no_of_possibilities(i));
    }
}

struct PossibilityCounter {
    hm: HashMap<i32, i32>
}
impl PossibilityCounter {
    fn new() -> PossibilityCounter {
        PossibilityCounter {
            hm: HashMap::new()
        }
    }

    fn no_of_possibilities(&mut self, n: i32) -> i32 {
        if let Some(x) = self.hm.get(&n) {
            return *x;
        } else {
            let mut total = 0;
            if n >= 3 {
                total += 1 + self.no_of_possibilities(n - 3);
            }
            if n >= 2 {
                total += 1 + self.no_of_possibilities(n - 2);
            }
            if n >= 1 {
                total += 1 + self.no_of_possibilities(n - 1);
            }
            self.hm.insert(n, total);
            return total
        }
    }
}