use std::{
    fmt::{Display, Write},
    sync::Arc,
};

use axum::extract::ws::Message;
use futures_util::{lock::Mutex, StreamExt};

use crate::{
    deck::{self, Deck},
    User,
};

struct Player {
    user: Arc<Mutex<User>>,
    hidden_cards: Vec<deck::Card>,
    visible_cards: Vec<deck::Card>,
    hand: Vec<deck::Card>,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Player {
    pub async fn notify_invalid_action(&mut self) {
        self.user.lock().await.send("Invalid action").await;
    }

    pub async fn send_cards(&mut self) {
        let mut s = "".to_string();

        writeln!(s, "Hand Cards:").unwrap();
        for card in self.hand.iter() {
            writeln!(s, "{}", serde_json::ser::to_string(card).unwrap()).unwrap();
        }

        writeln!(s, "Top Cards:").unwrap();
        for card in self.visible_cards.iter() {
            writeln!(s, "{}", serde_json::ser::to_string(card).unwrap()).unwrap();
        }
    }
}

pub struct SkitGubbe {
    players: Vec<Arc<Mutex<Player>>>,
    deck: Arc<Mutex<deck::Deck>>,
}

const MAX_TURNS: usize = 300;

impl SkitGubbe {
    pub fn new(users: Vec<Arc<Mutex<User>>>) -> Self {
        assert!(
            users.len() <= 4,
            "Skit Gubbe game must be 4 players or less"
        );

        let mut deck = deck::Deck::new_deck();
        let mut players = vec![];
        for user in users.into_iter() {
            players.push(Arc::new(Mutex::new(Player {
                user,
                hand: deck.pull_cards(3),
                hidden_cards: deck.pull_cards(3),
                visible_cards: deck.pull_cards(3),
            })));
        }

        Self {
            deck: Arc::new(Mutex::new(deck)),
            players,
        }
    }

    pub async fn run(mut self) -> Result<Option<usize>, ()> {
        for player in self.players.iter_mut() {
            player
                .lock()
                .await
                .user
                .lock()
                .await
                .send("Game has started.")
                .await
                .map_err(|_| ())?;
        }

        self.execute_setup_round().await;

        let mut winner = None;
        for _ in 0..MAX_TURNS {
            if let Some(player) = self.execute_round().await {
                winner = Some(player);
                break;
            }
        }

        self.notify_end(winner).await;
        Ok(winner)
    }

    /// Executes the setup round for all players asynchronously
    async fn execute_setup_round(&mut self) {
        let player_futures: Vec<_> = self
            .players
            .iter()
            .map(|player| {
                tokio::spawn(Self::player_setup_round(
                    Arc::clone(&player),
                    Arc::clone(&self.deck),
                ))
            })
            .collect();
        futures::future::join_all(player_futures).await;
    }

    async fn player_setup_round(
        player: Arc<Mutex<Player>>,
        deck: Arc<Mutex<Deck>>,
    ) -> Result<(), ()> {
        let mut player = player.lock().await;

        // show players their cards
        player.send_cards().await;

        // allow players to exchange their cards
        loop {
            let message = player.user.lock().await.receiver.next().await;
            // if no message
            let Some(message) = message else {
                continue;
            };

            // if error reading
            let Message::Text(message) = message.map_err(|_| ())? else {
                continue;
            };
            // parse msg
            let Ok(action) = serde_json::from_str::<action::PlayerSetup>(&message) else {
                player.notify_invalid_action().await;
                continue;
            };
            match action {
                action::PlayerSetup::ExchangeCard { hand, bottom } => todo!(),
                action::PlayerSetup::CompoundCard { hand, target } => todo!(),
                action::PlayerSetup::FinishExchange => todo!(),
            }
        }
    }

    /// Executes a round where all players play, if a player wins the game stops and the index of
    /// the winning player is returned.
    ///
    /// Returns: index of winning player
    async fn execute_round(&mut self) -> Option<usize> {
        todo!()
    }

    /// Notifies all players of end of game and the optional winner
    async fn notify_end(&self, winner: Option<usize>) {
        todo!()
    }
}

mod action {
    use serde::{Deserialize, Serialize};

    use crate::deck::Card;

    #[derive(Deserialize, Serialize)]
    pub enum PlayerSetup {
        ExchangeCard { hand: Vec<Card>, bottom: usize },
        CompoundCard { hand: Vec<Card>, target: usize },
        FinishExchange,
    }
}
