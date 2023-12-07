use std::collections::HashMap;

fn main() {
    unimplemented!();
}

use rand::seq::SliceRandom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub type Rank = u8;

#[derive(Hash, Eq, PartialEq, PartialOrd, EnumIter)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade
}

pub type Card = (Rank, Suit);

pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn new_deck() -> Self {
        let mut cards = Vec::new();

        for rank in 2..=13 {
            for suit in Suit::iter() {
                cards.push((rank, suit));
            }
        }

        let mut rng = rand::thread_rng();
        cards.as_mut_slice().shuffle(&mut rng);

        Self { cards }
    }

    pub fn pull_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}


