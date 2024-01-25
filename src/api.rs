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
            ///
            /// Valid combinations:
            /// - Multiple cards of the same rank
            /// - A rank 2 card then any rank card
            /// - A rank 10 card then any rank card
            ///
            /// # Errors
            /// Server returns an error if sequence is invalid:
            PlaceCard { cards: Vec<Card> },

            /// Player pulls a card from the draw and picks up the stack
            PullCard,

            /// Player picks up the stack
            PickupStack
        }
    }

}

pub mod server_messages {
    use serde::{Deserialize, Serialize};

    use crate::deck::Card;

    #[derive(Serialize, Deserialize)]
    pub enum ServerNotification {
        GameStart(Vec<String>),
        Id(String)
    }

    #[derive(Serialize, Deserialize)]
    #[serde(tag = "Stage")]
    pub enum Stage {
        Swap,
        Play,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(tag = "state")]
    pub struct Cards {
        pub owner_id: String,
        pub hand: Vec<Card>,
        pub bottom_cards: Vec<Vec<Card>>,
    }

    impl Cards {
        pub fn new(owner_id: String, hand: Vec<Card>, bottom_cards: Vec<Vec<Card>>) -> Self {
            Self { hand, bottom_cards, owner_id }
        }
    }
}
