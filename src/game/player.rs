use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use futures_util::lock::Mutex;

use crate::deck;
use crate::user::User;
use super::playercards::PlayerCards;


pub struct Player {
    pub user: Arc<Mutex<User>>,
    pub cards: PlayerCards,
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
            cards: PlayerCards::new(hand, visible_cards.to_vec(), hidden_cards)
        }
    }


    pub async fn notify_invalid_action(&mut self) {
        let _ = self.user.lock().await.send("Invalid action").await;
    }

    pub async fn send_cards(&mut self) {
        let cards = self.cards.to_server_player_cards(self.user.lock().await.id.to_string());
        let s = serde_json::to_string(&cards).unwrap();
        let _ = self.user.lock().await.send(&s).await;
    }
}

impl Deref for Player {
    type Target = PlayerCards;

    fn deref(&self) -> &Self::Target {
        &self.cards
    }
}

impl DerefMut for Player {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cards
    }
}
