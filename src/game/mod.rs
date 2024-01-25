mod player;
mod playercards;

use std::sync::Arc;

use axum::extract::ws::Message;
use futures_util::{lock::Mutex, StreamExt};

use crate::api::server_messages::Cards;
use crate::deck;
use crate::deck::Card;
use crate::deck::Deck;
use player::Player;

use crate::user::User;

use crate::api::player_messages;
use crate::api::server_messages;

pub struct SkitGubbe {
    players: Vec<Arc<Mutex<Player>>>,
    deck: Arc<Mutex<Deck>>,
    playing_stack: Vec<Card>,
}

const MAX_TURNS: usize = 300;

impl SkitGubbe {
    pub fn new(users: Vec<Arc<Mutex<User>>>) -> Self {
        assert!(
            users.len() <= 4,
            "Skit Gubbe game must be 4 players or less"
        );

        let mut deck = Deck::new_deck();
        let mut players = vec![];

        for user in users.into_iter() {
            let hidden_cards: [Option<Card>; 3] =
                core::array::from_fn(|_| Some(deck.pull_card().expect("Should have enough cards")));
            let visible_cards: [Vec<Card>; 3] = core::array::from_fn(|_| {
                let x = deck.pull_cards(1);
                assert_eq!(x.len(), 1, "Should have enough cards");
                x
            });

            players.push(Arc::new(Mutex::new(Player::new(
                user,
                hidden_cards,
                visible_cards,
                deck.pull_cards(3),
            ))));
        }

        Self {
            deck: Arc::new(Mutex::new(deck)),
            players,
            playing_stack: vec![],
        }
    }

    pub async fn notify_all_players(&self, msg: &str) {
        for player in &self.players {
            let _ = player.lock().await.user.lock().await.send(msg).await;
        }
    }

    pub async fn run(mut self) -> Result<Option<usize>, String> {
        let mut player_ids = vec![];
        for player in &self.players {
            player_ids.push(player.lock().await.user.lock().await.id.to_string());
        }
        let game_start_msg = server_messages::ServerNotification::GameStart(player_ids.clone());

        for (player, id) in self.players.iter_mut().zip(&player_ids) {
            player
                .lock()
                .await
                .user
                .lock()
                .await
                .send(&serde_json::to_string(&game_start_msg).unwrap())
                .await
                .map_err(|_| id)?;
        }

        // start setup round
        self.notify_all_players(&serde_json::to_string(&server_messages::Stage::Swap).unwrap())
            .await;
        self.execute_setup_round().await?;

        // start normal rounds
        self.notify_all_players(&serde_json::to_string(&server_messages::Stage::Play).unwrap())
            .await;
        let mut winner_index = None;
        for _ in 0..MAX_TURNS {
            if let Some(player_index) = self.execute_round().await? {
                winner_index = Some(player_index);
                break;
            }
        }

        self.notify_end(winner_index).await;
        Ok(winner_index)
    }

    /// Executes the setup round for all players asynchronously
    ///
    /// # Errors
    ///
    /// Returns an error if player connects with player ID
    async fn execute_setup_round(&mut self) -> Result<(), String> {
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

        // TODO: Add timeout period if one of the players takes too long
        let answers = futures::future::join_all(player_futures).await;
        let all_player_cards: Vec<Cards> = answers
            .into_iter()
            .map(|x| x.unwrap())
            .collect::<Result<Vec<_>, _>>()?;

        // send all players all cards
        for player in &self.players {
            let player = player.lock().await;
            let _ = player
                .user
                .lock()
                .await
                .send(&serde_json::to_string(&all_player_cards).unwrap())
                .await;
        }

        Ok(())
    }

    /// Runs the player's swap round asynchronously.
    /// Returns the players new cards
    ///
    /// # Errors
    ///
    /// This function will return an error if the player disconnects.
    /// Returns the player ID if error.
    async fn player_setup_round(
        player: Arc<Mutex<Player>>,
        deck: Arc<Mutex<Deck>>,
    ) -> Result<server_messages::Cards, String> {
        let mut player = player.lock().await;
        let player_id = player.user.lock().await.id.to_string();

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
            let Message::Text(message) = message.map_err(|_| player_id.clone())? else {
                continue;
            };
            // parse msg
            let Ok(action) = serde_json::from_str::<player_messages::action::SetupAction>(&message)
            else {
                player.notify_invalid_action().await;
                continue;
            };
            match action {
                player_messages::action::SetupAction::ExchangeCard { hand, bottom } => {
                    if let Err(e) = player.exchange_cards(hand, bottom).await {
                        let _ = player.user.lock().await.send(e).await;
                        continue;
                    }
                    player.send_cards().await;
                }
                player_messages::action::SetupAction::CompoundCard { hand, bottom } => {
                    if let Err(e) = player.compound_cards(hand, bottom).await {
                        let _ = player.user.lock().await.send(&e).await;
                        continue;
                    }

                    if player.cards.hand.len() < 3 {
                        let num_pick_up = 3 - player.cards.hand.len();
                        player
                            .cards
                            .hand
                            .append(&mut deck.lock().await.pull_cards(num_pick_up));
                        player.cards.hand.sort();
                    }

                    player.send_cards().await;
                }
                player_messages::action::SetupAction::FinishExchange => {
                    break;
                }
            };
        }

        let id = player.user.lock().await.id.to_string();
        Ok(server_messages::Cards {
            owner_id: id,
            hand: player.cards.hand.to_vec(),
            bottom_cards: player.get_bottom_cards(),
        })
    }

    /// Executes a round where all players play, if a player wins the game stops and the index of
    /// the winning player is returned.
    ///
    /// Returns: index of winning player
    async fn execute_round(&mut self) -> Result<Option<usize>, String> {
        let mut player_index = 0;
        'player_loop: while player_index <= self.players.len() {
            let mut player = self.players[player_index].lock().await;
            if player.has_won() {
                return Ok(Some(player_index));
            }

            let player_id = player.user.lock().await.id.to_string();

            let message = player.user.lock().await.receiver.next().await;
            // if no message
            let Some(message) = message else {
                continue;
            };

            // if error reading
            let Message::Text(message) = message.map_err(|_| player_id.clone())? else {
                continue;
            };
            // parse msg
            let Ok(action) = serde_json::from_str::<player_messages::action::PlayAction>(&message)
            else {
                player.notify_invalid_action().await;
                continue;
            };

            let card = match action {
                player_messages::action::PlayAction::PlaceCard { card } => card,
                player_messages::action::PlayAction::PickupStack => {
                    player.hand.append(&mut self.playing_stack);
                    player_index += 1;
                    continue 'player_loop;
                }
            };

            // check if actions are valid
            let mut player_cards_copy = player.cards.clone();
            if player_cards_copy.play_card(&card).is_none() {
                player.notify_invalid_action().await;
                continue 'player_loop;
            }

            match card.rank {
                2 => {
                    self.playing_stack.push(card);
                }
                10 => {
                    self.playing_stack = vec![];
                }
                _ => {
                    // if bad card
                    if let Some(last_card) = self.playing_stack.last()
                        && last_card.rank > card.rank
                    {
                        player.notify_invalid_action().await;
                        continue 'player_loop;
                    }

                    self.playing_stack.push(card);

                    // if last 4 are the same rank then restart the stack
                    if self.playing_stack.len() >= 4
                        && self.playing_stack[self.playing_stack.len() - 4..]
                            .iter()
                            .filter(|x| self.playing_stack.last().unwrap().rank == x.rank)
                            .count()
                            == 4
                    {
                        self.playing_stack = vec![];
                    }
                }
            }

            // confirm player actions
            player.cards = player_cards_copy;

            // pickup any cards if needed
            if player.cards.hand.len() < 3 {
                let pickup_num = 3 - player.cards.hand.len();
                player.cards.hand.append(&mut self.deck.lock().await.pull_cards(pickup_num));
            }
        }

        Ok(None)
    }

    /// Notifies all players of end of game and the optional winner
    async fn notify_end(&self, winner: Option<usize>) {
        let msg;
        if let Some(winner) = winner {
            msg = format!(
                "Game finished, winner is {}!",
                self.players[winner].lock().await.user.lock().await.id
            );
        } else {
            msg = format!("Game finished, its a draw!",);
        }

        for player in self.players.iter() {
            let _ = player.lock().await.user.lock().await.send(&msg).await;
        }
    }
}
