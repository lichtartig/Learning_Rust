/// Permutations without Dups: Write a method to compute all permutations of a string of unique
/// characters.

fn main() {
    let s = String::from("abcd");
    let res = get_all_permutations(&s);
    for t in &res {
        println!("{}", t);
    }
    println!("Total of {} permutations.", res.len());
}

fn get_all_permutations(s: &str) -> Vec<String> {
    let mut res = Vec::new();
    if s.len() == 1 {
        res.push(String::from(s));
        return res;
    }
    let perm = get_all_permutations(&s[1..]);
    for p in &perm {
        for i in 0..p.len()+1 {
            res.push(String::from(&p[0..i]) + &s[0..1] + &p[i..]);
        }
    }
    return res;
}
