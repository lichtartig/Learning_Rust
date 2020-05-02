/// Design the data structures for a generic deck of cards. Explain how you would
/// subclass the data structures to implement blackjack.

use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
}

/// This is my generic card deck class. It takes two types. 'S' is a series type. In our case, it
/// will be the series enum below (Clubs, Diamonds, Hearts, Spades). 'N' is a type that indicates
/// which cards exist in the series (i.e Ace, Two, ...).
struct CardDeck<S, N> {
    cards: Vec<(S, N)>,
}

impl<S: IntoEnumIterator + Clone, N: IntoEnumIterator + Clone> CardDeck<S, N> {
    /// The constructor initializes a vector which holds one instances of every distinct card
    /// and shuffles it.
    pub fn new() -> CardDeck<S, N> {
        let mut cards: Vec<(S, N)> = Vec::new();
        for s in S::iter() {
            for n in N::iter() {
                cards.push((s.clone(), n.clone()));
            }
        }
        cards.shuffle(&mut thread_rng());

        CardDeck {
            cards: cards,
        }
    }

    /// This draws a card from the stack and returns it.
    pub fn draw_card(&mut self) -> Option<(S, N)> {
        self.cards.pop()
    }
}


#[derive(Clone, Debug, EnumIter)]
enum Series {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Clone, Debug, EnumIter)]
enum Number {
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
}