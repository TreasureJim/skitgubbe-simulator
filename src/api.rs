pub mod player_messages {
    pub mod action {
        use serde::{Deserialize, Serialize};

        use crate::deck::Card;

        #[derive(Deserialize, Serialize)]
        pub enum PlayerSetup {
            ExchangeCard { hand: Vec<Card>, bottom: usize },
            CompoundCard { hand: Vec<Card>, bottom: usize },
            FinishExchange,
        }
    }
}

pub mod server_messages {
    use serde::{Deserialize, Serialize};

    use crate::deck::Card;

    #[derive(Serialize, Deserialize)]
    pub enum ServerMessages {
        Id(String)
    }

    #[derive(Serialize, Deserialize)]
    #[serde(tag = "Stage")]
    pub enum Stage {
        Swap,
        Play,
    }

    #[derive(Serialize)]
    #[serde(tag = "state")]
    pub struct PlayerCards {
        owner_id: String,
        hand: Vec<Card>,
        bottom_cards: Vec<Vec<Card>>,
    }

    impl PlayerCards {
        pub fn new(owner_id: String, hand: Vec<Card>, bottom_cards: Vec<Vec<Card>>) -> Self {
            Self { hand, bottom_cards, owner_id }
        }
    }
}