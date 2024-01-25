use std::fmt::Write;
use std::mem;
use std::sync::Arc;

use futures_util::lock::Mutex;

use crate::api;
use crate::deck::{self, Card};
use crate::user::User;

pub struct Player {
    pub user: Arc<Mutex<User>>,
    /// 3 flipped cards that are hidden from the player at the beginning
    hidden_cards: [Option<deck::Card>; 3],
    /// Cards on top of the hidden cards. Each vector must contain only duplicate card ranks.
    /// The vector can only have a maximum length of 4
    visible_cards: [Vec<deck::Card>; 3],
    /// Cards in the players hand. Must be ordered during the setup stage.
    pub hand: Vec<deck::Card>,
}

impl Player {
    pub fn new(
        user: Arc<Mutex<User>>,
        hidden_cards: [Option<deck::Card>; 3],
        visible_cards: [Vec<deck::Card>; 3],
        mut hand: Vec<deck::Card>,
    ) -> Self {
        hand.sort();

        Self {
            user,
            hidden_cards,
            visible_cards,
            hand,
        }
    }

    pub async fn notify_invalid_action(&mut self) {
        let _ = self.user.lock().await.send("Invalid action").await;
    }

    pub async fn send_cards(&mut self) {
        let bottom_cards;
        if !self.visible_cards.is_empty() {
            bottom_cards = self.visible_cards.to_vec();
        } else {
            bottom_cards = self.hidden_cards.to_vec().into_iter().map(|x| {
                let mut arr = Vec::with_capacity(1); if let Some(card) = x { arr.push(card); } arr 
            }).collect();
        }

        let cards = api::server_messages::PlayerCards::new(self.user.lock().await.id.to_string(), self.hand.to_vec(), bottom_cards);
        let s = serde_json::to_string(&cards).unwrap();
        let _ = self.user.lock().await.send(&s).await;
    }

    /// Switches the cards from `cards` by removing from `self.hands` into
    /// `self.visible_cards[bottom_index]`
    ///
    /// Validates that:
    ///     - `cards` all are the same rank
    ///     - `cards` exist in `self.hand`. eg. if `cards` is `[4, 4]` then `self.hands` must contains two 4s
    pub async fn exchange_cards(
        &mut self,
        mut cards: Vec<Card>,
        bottom_index: usize,
    ) -> Result<(), &'static str> {
        let first_index_hand = self.check_given_cards_valid(&cards)?;

        // remove the cards form `self.hand`
        self.hand
            .drain(first_index_hand..first_index_hand + cards.len());

        // exchange the bottom cards into `self.hand` ensuring order
        self.hand.append(&mut self.visible_cards[bottom_index]);
        self.hand.sort();
        // swap the cards into the bottom cards
        mem::swap(&mut cards, &mut self.visible_cards[bottom_index]);

        Ok(())
    }

    /// Compound `cards` into the vector of `self.visible_cards[bottom_index]`.
    ///
    /// # Errors
    /// If `cards` is empty
    /// If `cards` do not all have the same rank
    /// If `self.hand` does not contain `cards`
    pub async fn compound_cards(
        &mut self,
        mut cards: Vec<Card>,
        bottom_index: usize,
    ) -> Result<(), &'static str> {
        let first_index_hand = self.check_given_cards_valid(&cards)?;

        // remove the cards form `self.hand`
        self.hand
            .drain(first_index_hand..first_index_hand + cards.len());

        // check that card is same as bottom card
        // here the bottom cards will always be the visible cards
        if cards[0].rank != self.visible_cards[bottom_index][0].rank {
            return Err("card to compound doesn't have the same rank as the bottom card index rank");
        }

        // add to the bottom cards
        self.visible_cards[bottom_index].append(&mut cards);

        Ok(())
    }

    /// Check if `self.hand` contains `cards` and that given cards are all the same rank
    ///
    /// # Errors
    /// If `cards` is empty
    /// If `cards` do not all have the same rank
    /// If `self.hand` does not contain `cards`
    ///
    /// Returns: the index of the first occurence of a card with the rank `cards`
    fn check_given_cards_valid(&mut self, cards: &[Card]) -> Result<usize, &'static str> {
        if cards.is_empty() {
            return Err("The given cards was empty");
        }
        if cards.iter().filter(|x| cards[0].rank == x.rank).count() != cards.len() {
            return Err("All the the given cards must have the same rank");
        }

        // ensure hand is sorted before exchanging
        self.hand.sort();

        // ensure all the cards in `card` is in `self.hand`
        // collect all indexes of matches
        let first_index_hand = self
            .hand
            .iter()
            .position(|x| cards[0].rank == x.rank)
            .ok_or("Card to swap is not in hand")?;

        let mut num_in_hand = 0;
        for card in self.hand.iter().skip(first_index_hand) {
            if num_in_hand == 4 || card.rank != cards[0].rank {
                break;
            }
            num_in_hand += 1;
        }
        if num_in_hand != cards.len() {
            return Err("Your hand doesn't contain the given cards");
        }

        Ok(first_index_hand)
    }
}
