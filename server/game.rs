use crate::{deck, User};

pub struct GameServer {}

impl GameServer {
    pub fn wait_response() {
        todo!()
    }
}

struct Player<'a> {
    user: &'a User,
    hidden_cards: Vec<deck::Card>,
    visible_cards: Vec<deck::Card>,
    hand: Vec<deck::Card>,
}

pub struct SkitGubbe<'a> {
    players: Vec<Player<'a>>,
    deck: deck::Deck,
}

const MAX_TURNS: usize = 300;

impl<'a> SkitGubbe<'a> {
    pub fn new(users: &'a [User]) -> Self {
        assert!(
            users.len() <= 4,
            "Skit Gubbe game must be 4 players or less"
        );

        let mut deck = deck::Deck::new_deck();
        let mut players = vec![];
        for user in users {
            players.push(Player {
                user,
                hand: deck.pull_cards(3),
                hidden_cards: deck.pull_cards(3),
                visible_cards: deck.pull_cards(3),
            });
        }

        Self { deck, players }
    }

    pub async fn run(mut self) -> Option<&'a User> {
        // self.notify_players().await;
        // self.execute_setup_round().await;

        let mut winner = None;
        for _ in 0..MAX_TURNS {
            if let Some(player) = self.execute_round().await {
                winner = Some(player);
                break;
            }
        }

        self.notify_end(&winner.map(|x| &self.players[x])).await;

        let Self { players, .. } = self;
        winner.map(|x| players[x].user)
    }

    async fn execute_setup_round(&mut self) {
        todo!()
    }

    /// Executes a round where all players play, if a player wins the game stops and the index of
    /// the winning player is returned.
    ///
    /// Returns: index of winning player
    async fn execute_round(&mut self) -> Option<usize> {
        todo!()
    }

    async fn notify_players(&self) {
        todo!()
    }

    /// Notifies all players of end of game and the optional winner
    async fn notify_end(&self, winner: &Option<&Player<'_>>) {
        todo!()
    }
}
