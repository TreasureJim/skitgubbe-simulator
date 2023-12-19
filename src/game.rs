use crate::deck;

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
    hidden_cards: Vec<deck::Card>,
    visible_cards: Vec<deck::Card>,
    hand: Vec<deck::Card>,
}

impl Player {
    fn new() -> Self {
        Self {
            hidden_cards: vec![],
            visible_cards: vec![],
            hand: vec![],
        }
    }
}

enum GameState {
    Setup, 
    Playing
}

pub struct SkitGubbe<'server> {
    server: &'server GameServer,

    turn: usize,
    deck: deck::Deck,

    players: Vec<Player>,

    state: GameState
}

impl<'server> SkitGubbe<'server> {
    pub fn new(server: &'server GameServer, num_players: usize) -> Self {
        assert!(
            num_players <= 4,
            "Skit Gubbe game must be 4 players or less"
        );

        let mut deck = deck::Deck::new_deck();
        let mut players = vec![];
        for _ in 0..num_players {
            players.push(Player {
                hand: vec![],
                hidden_cards: deck.pull_cards(3),
                visible_cards: deck.pull_cards(3),
            });
        }

        Self {
            server,
            turn: 0,
            deck,
            players,
            state: GameState::Setup
        }
    }

    fn execute_setup_turn(&mut self) {
        
    }

    fn execute_turn(&mut self) {

    }
}

impl Game for SkitGubbe<'_> {
    fn execute_turn(&mut self) {
        match self.state {
            GameState::Setup => self.execute_setup_turn(),
            GameState::Playing => self.execute_turn()
        }
    }
}
