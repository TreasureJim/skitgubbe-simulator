use std::mem;
use crate::api;
use crate::deck::{self, Card};

#[derive(Clone)]
pub struct PlayerCards {
    /// 3 flipped cards that are hidden from the player at the beginning
    hidden_cards: [Option<deck::Card>; 3],
    /// Cards on top of the hidden cards. Each vector must contain only duplicate card ranks.
    /// The vector can only have a maximum length of 4
    visible_cards: Vec<Vec<deck::Card>>,
    /// Cards in the players hand. Must be ordered during the setup stage.
    pub hand: Vec<deck::Card>,
}

impl PlayerCards {
    pub fn new(
        hand: Vec<deck::Card>,
        visible_cards: Vec<Vec<deck::Card>>,
        hidden_cards: [Option<deck::Card>; 3],
    ) -> Self {
        Self {
            hidden_cards,
            visible_cards,
            hand,
        }
    }

    pub fn has_won(&self) -> bool {
        return self.hand.is_empty() && self.get_bottom_cards().is_empty();
    }
    pub fn to_server_player_cards(&self, id: String) -> api::server_messages::Cards {
        let bottom_cards;
        if !self.visible_cards.is_empty() {
            bottom_cards = self.visible_cards.to_vec();
        } else {
            bottom_cards = self
                .hidden_cards
                .to_vec()
                .into_iter()
                .map(|x| {
                    let mut arr = Vec::with_capacity(1);
                    if let Some(card) = x {
                        arr.push(card);
                    }
                    arr
                })
                .collect();
        }

        let cards = api::server_messages::Cards::new(
            id, 
            self.hand.to_vec(),
            bottom_cards,
        );

        cards
    }

    /// Checks if the player is able to play the card and removes it from the correct card stack (hand, visible or hidden)
    ///
    /// # Returns
    /// Returns the removed card or None.
    pub fn play_card(&mut self, card: &Card) -> Option<Card> {
        // hand
        if !self.hand.is_empty() {
            if let Some(index) = self.hand.iter().position(|x| x == card) {
                return Some(self.hand.remove(index));
            }
            return None;
        }

        // visible cards
        if !self.visible_cards.is_empty() {
            // find if any stacks match
            let vis_vec_index = self
                .visible_cards
                .iter()
                .map(|vec| &vec[0])
                .position(|vis_card| vis_card.rank == card.rank)?;

            let matching_vis_card_index = self.visible_cards[vis_vec_index]
                .iter()
                .position(|vis_card| vis_card == card)?;
            let removed_card = self.visible_cards[vis_vec_index].remove(matching_vis_card_index);

            if self.visible_cards[vis_vec_index].is_empty() {
                self.visible_cards.swap_remove(vis_vec_index);
            }

            return Some(removed_card);
        }

        // hidden cards
        let hidden_cards: Vec<(usize, &Card)> = self
            .hidden_cards
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.as_ref().unwrap())
            .enumerate()
            .collect();
        if card.rank == 2
            || card.rank == 10
            || card.rank == deck::ACE_RANK && hidden_cards.len() == 1
        {
            // Cant play rank 2, 10 or ACE for last card
            return None;
        }
        let matching_hidden = hidden_cards
            .into_iter()
            .find(|(_, hidden_card)| **hidden_card == *card)?;
        return self.hidden_cards[matching_hidden.0].take();
    }

    pub fn get_bottom_cards(&self) -> Vec<Vec<deck::Card>> {
        if !self.visible_cards.is_empty() {
            return self.visible_cards.to_vec();
        } else {
            return self
                .hidden_cards
                .to_vec()
                .into_iter()
                .map(|option| {
                    if let Some(card) = option {
                        return vec![card];
                    } else {
                        return vec![];
                    }
                })
                .collect();
        }
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
            return Err(
                "card to compound doesn't have the same rank as the bottom card index rank",
            );
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
