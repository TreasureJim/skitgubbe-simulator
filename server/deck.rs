use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub type Rank = u8;

#[derive(Deserialize, Serialize)]
#[derive(Hash, Eq, PartialEq, PartialOrd, EnumIter, Clone)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade
}

#[derive(Deserialize, Serialize)]
pub struct Card (Rank, Suit);

pub struct Deck {
    pub cards: Vec<Card>
}

impl Deck {
    pub fn new_deck() -> Self {
        let mut cards = Vec::new();

        for rank in 2..=13 {
            for suit in Suit::iter() {
                cards.push(Card (rank, suit));
            }
        }

        let mut rng = rand::thread_rng();
        cards.as_mut_slice().shuffle(&mut rng);

        Self { cards }
    }

    pub fn pull_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    /// Pulls n cards from the end of the deck and returns them in a vector.
    /// If n is greater than the deck size then a maximum of the card deck will be returned. 
    pub fn pull_cards(&mut self, n: usize) -> Vec<Card> {
        let start = 0.max(self.cards.len() - n - 1);
        self.cards.drain(start..).collect()
    }
}


