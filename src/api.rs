pub mod player_messages {
    pub mod action {
        use serde::{Deserialize, Serialize};

        use crate::deck::Card;

        #[derive(Deserialize, Serialize)]
        pub enum SetupAction {
            ExchangeCard { hand: Vec<Card>, bottom: usize },
            CompoundCard { hand: Vec<Card>, bottom: usize },
            FinishExchange,
        }

        #[derive(Deserialize, Serialize)]
        pub enum PlayAction {
            /// Player places a card or cards
            /// If player has < 3 cards then will automatically pick up a card if possible
            ///
            /// Valid combinations:
            /// - Multiple cards of the same rank
            /// - A rank 2 card then any rank card
            /// - A rank 10 card then any rank card
            ///
            /// # Errors
            /// Server returns an error if sequence is invalid:
            PlaceCard { card: Card },

            /// Player picks up the stack
            PickupStack,
        }
    }
}

pub mod server_messages {
    use serde::{Deserialize, Serialize};

    use crate::deck::Card;

    #[derive(Serialize, Deserialize)]
    pub enum ServerNotification {
        GameStart(Vec<String>),
        Id(String),
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(tag = "Stage")]
    pub enum Stage {
        Swap,
        Play,
    }

    pub type BottomCards = Vec<Vec<Card>>;

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(tag = "state")]
    pub struct Cards {
        pub hand: Vec<Card>,
        pub bottom_cards: BottomCards,
    }

    /// A struct representing the game state
    #[derive(Serialize, Deserialize, Debug)]
    pub struct GameState {
        /// The ID of the player who turn it is
        pub turn: String,
        pub stage: Stage,
        /// The cards of the player
        pub cards: Cards,
        /// The stack of played cards
        pub stack: Vec<Card>,
        /// Other players visible cards
        pub other_players: Vec<(String, BottomCards)>,
    }
}
