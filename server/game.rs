use std::ops::Deref;

use crate::{deck, User};

pub struct GameServer {}

impl GameServer {
    pub fn wait_response() {
        todo!()
    }
}

pub struct GameManager {}

trait Game {
    fn execute_turn(&mut self);
}

struct Player {
    user: User,
    hidden_cards: Vec<deck::Card>,
    visible_cards: Vec<deck::Card>,
    hand: Vec<deck::Card>,
}

enum GameState {
    Setup,
    Playing,
}

pub struct SkitGubbe {
    turn: usize,
    deck: deck::Deck,

    players: Vec<Player>,

    state: GameState,
}

const MAX_TURNS: usize = 300;

impl SkitGubbe {
    pub fn new(users: Vec<User>) -> Self {
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

        Self {
            turn: 0,
            deck,
            players,
            state: GameState::Setup,
        }
    }

    pub async fn run(mut self) -> (Vec<User>, Option<usize>) {
        self.notify_players().await;
        // self.execute_setup_round().await;

        let mut winner = None;
        for _ in 0..MAX_TURNS {
            if let Some(player) = self.execute_round().await {
                winner = Some(player);
                break;
            }
        }

        todo!("Rework design of queue system");
        self.notify_end(&winner.map(|x| &x.user.id)).await;

        let SkitGubbe { players, .. } = self;
        return (players.into_iter().map(|player| player.user).collect(), None);


        // return winner.to_owned().map(|x| x.to_owned().user);
    }

    fn get_winner(self, player: &Player) -> Player {
        *player
    }

    async fn execute_setup_round(&mut self) {
        todo!()
    }

    async fn execute_round(&mut self) -> Option<&Player> {
        todo!()
    }

    async fn notify_players(&self) {
        todo!()
    }

    /// Notifies all players of end of game and the optional winner
    async fn notify_end(&self, winner: &Option<&uuid::Uuid>) {
        todo!()
    }
}
