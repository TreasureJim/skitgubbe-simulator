use futures_util::lock::Mutex;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use super::playercards::PlayerCards;
use crate::api::server_messages::{self, GameState};
use crate::deck;
use crate::user::User;

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
            cards: PlayerCards::new(hand, visible_cards.to_vec(), hidden_cards),
        }
    }

    pub async fn notify_invalid_action(&mut self) {
        let _ = self.user.lock().await.send("Invalid action").await;
    }

    pub async fn send_setup_game_state(&self) {
        let message = GameState {
            turn: "".to_string(),
            // turn: self.user.lock().await.id.to_string(),
            stage: server_messages::Stage::Swap,
            cards: self.to_server_player_cards(),
            stack: vec![],
            other_players: vec![],
        };

        let _ = self
            .user
            .lock()
            .await
            .send(&serde_json::to_string(&message).unwrap())
            .await;
    }

    // pub async fn send_cards(&mut self) {
    //     let cards = self.cards.to_server_player_cards();
    //     let s = serde_json::to_string(&cards).unwrap();
    //     let _ = self.user.lock().await.send(&s).await;
    // }
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
