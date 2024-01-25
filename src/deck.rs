use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub type Rank = u8;

#[derive(Deserialize, Serialize, Hash, PartialEq, PartialOrd, Eq, Ord, EnumIter, Clone, Debug)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct Card {
    /// Card order:
    /// 2 3 4 5 6 7 8 9 10 J Q K A
    /// 2 = 2
    /// Ace = 14
    pub rank: Rank,
    pub suit: Suit,
}

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new_deck() -> Self {
        let mut cards = Vec::new();

        for rank in 2..=14 {
            for suit in Suit::iter() {
                cards.push(Card { rank, suit });
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
        let start = 0.max(self.cards.len().saturating_sub(n));
        self.cards.drain(start..).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_deck_has_correct_size() {
        let deck = Deck::new_deck();
        assert_eq!(deck.cards.len(), 52);
    }

    #[test]
    fn test_pull_card_reduces_deck_size() {
        let mut deck = Deck::new_deck();
        let original_size = deck.cards.len();
        let _ = deck.pull_card();
        assert_eq!(deck.cards.len(), original_size - 1);
    }

    #[test]
    fn test_pull_cards_returns_correct_number() {
        let mut deck = Deck::new_deck();
        let original_size = deck.cards.len();
        let n = 5;
        let pulled_cards = deck.pull_cards(n);
        assert_eq!(pulled_cards.len(), n);
        assert_eq!(deck.cards.len(), original_size - n);
    }

    #[test]
    fn test_pull_cards_handles_large_n() {
        let mut deck = Deck::new_deck();
        let original_size = deck.cards.len();
        let n = 100;
        let pulled_cards = deck.pull_cards(n);
        assert_eq!(pulled_cards.len(), original_size);
        assert_eq!(deck.cards.len(), 0);
    }

    #[test]
    fn test_pull_cards_handles_empty_deck() {
        let mut deck = Deck { cards: Vec::new() };
        let n = 5;
        let pulled_cards = deck.pull_cards(n);
        assert_eq!(pulled_cards.len(), 0);
        assert_eq!(deck.cards.len(), 0);
    }
}
