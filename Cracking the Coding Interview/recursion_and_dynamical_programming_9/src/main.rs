/// Parens: Implement an algorithm to print all valid (e.g., properly opened and closed)
/// combinations of n pairs of parentheses.
/// EXAMPLE
/// Input: 3
/// Output: ((())) , (()()) , (()) () , () (()) , ()()()
use std::collections::HashMap;

fn main() {
    let mut ps = ParenthesisString::new();
    let v = ps.generate_paren_str(3);
    for s in &v {
        println!("{}", s);
    }
}

struct ParenthesisString {
    hm: HashMap<u32, Vec<String>>
}

impl ParenthesisString {
    fn new() -> ParenthesisString {
        let mut hm = HashMap::new();
        hm.insert(0, vec![String::from("")]);
        hm.insert(1, vec![String::from("()")]);

        ParenthesisString {
            hm: hm
        }
    }

    fn generate_paren_str(&mut self, n: u32) -> Vec<String> {
        if let Some(v) = self.hm.get(&n) {
            return v.clone();
        } else {
            let mut v = Vec::new();
            for i in 0..n {
                let v1 = self.generate_paren_str(i);
                let v2 = self.generate_paren_str(n-1-i);
                v.append(self.all_comb(&v1, &v2).as_mut());
            }
            self.hm.insert(n, v.clone());
            return v;
        }
    }

    fn all_comb(&self, v1: &Vec<String>, v2: &Vec<String>) -> Vec<String> {
        let mut v = Vec::new();
        for i in v1 {
            for j in v2 {
                v.push(String::from("(") + i + ")" + j);
            }
        }
        v
    }
}