/// Shuffle: Write a method to shuffle a deck of cards. It must be a perfect shuffle-in other words,
/// each of the 52! permutations of the deck has to be equally likely. Assume that you are given a
/// random number generator which is perfect.
use rand::Rng;

fn main() {
    let deck = CardDeck::new(52);
    for card in deck.get_rand_perm() {
        println!("Next card: {}", card);
    }
}


/// We solve the problem by implementing the card deck in a struct. The key method is
/// get_rand_perm().
/// We make sure that all permutations are equally likely by picking one card at a time with equal
/// probability.

struct CardDeck {
    cards: Vec<usize>,
    n: usize
}

impl CardDeck {
    fn new(n: usize) -> CardDeck {
        let mut v = Vec::new();
        for i in 0..n {
            v.push(i);
        }

        CardDeck {
            cards: v,
            n
        }
    }

    fn get_rand_perm(&self) -> Vec<usize> {
        let mut buffer = self.cards.clone();
        let mut ret = Vec::new();
        let mut rng = rand::thread_rng();

        for i in 0..self.n {
            let index: usize = rng.gen_range(0, self.n-i);
            ret.push(*buffer.get(index).unwrap());
            buffer.remove(index);
        }

        ret
    }
}