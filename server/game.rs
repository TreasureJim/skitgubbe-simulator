use std::fmt::Write;

use crate::{deck, User};

pub struct GameServer {}

impl GameServer {
    pub fn wait_response() {
        todo!()
    }
}

struct Player<'a> {
    user: &'a mut User,
    hidden_cards: Vec<deck::Card>,
    visible_cards: Vec<deck::Card>,
    hand: Vec<deck::Card>,
}

impl<'a> Player<'a> {
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

pub struct SkitGubbe<'a> {
    players: Vec<Player<'a>>,
    deck: deck::Deck,
}

const MAX_TURNS: usize = 300;

impl<'a> SkitGubbe<'a> {
    pub fn new(users: &'a mut [User]) -> Self {
        assert!(
            users.len() <= 4,
            "Skit Gubbe game must be 4 players or less"
        );

        let mut deck = deck::Deck::new_deck();
        let mut players = vec![];
        for user in users.iter_mut() {
            players.push(Player {
                user,
                hand: deck.pull_cards(3),
                hidden_cards: deck.pull_cards(3),
                visible_cards: deck.pull_cards(3),
            });
        }

        Self { deck, players }
    }

    pub async fn run(mut self) -> Result<Option<&'a User>, ()> {
        for Player { user, ..} in self.players.iter_mut() {
            user.send("Game has started.").await;
        }

        self.execute_setup_round().await;

        let mut winner = None;
        for _ in 0..MAX_TURNS {
            if let Some(player) = self.execute_round().await {
                winner = Some(player);
                break;
            }
        }

        self.notify_end(&winner.map(|x| &self.players[x])).await;

        let Self { players, .. } = self;
        let users: Vec<_> = players.into_iter().map(|x| &*x.user).collect();
        Ok(winner.map(|x| users[x]))
    }

    async fn execute_setup_round(&mut self) {
        // show players their cards


        // start multiple asyncronous tasks to allow players to exchange their cards
    }

    /// Executes a round where all players play, if a player wins the game stops and the index of
    /// the winning player is returned.
    ///
    /// Returns: index of winning player
    async fn execute_round(&mut self) -> Option<usize> {
        todo!()
    }

    /// Notifies all players of end of game and the optional winner
    async fn notify_end(&self, winner: &Option<&Player<'_>>) {
        todo!()
    }
}
