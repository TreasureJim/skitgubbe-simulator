use std::{collections::VecDeque, sync::Arc};

use axum::extract::ws::{WebSocket, Message};
use futures::{
    lock::Mutex,
    stream::{SplitSink, SplitStream},
    StreamExt, SinkExt,
};

use crate::game;

pub struct ServerQueue {
    queue: Arc<Mutex<VecDeque<User>>>,
}

const GAME_PLAYER_SIZE: usize = 3;

impl ServerQueue {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub async fn push_user(&mut self, mut user: User) {
        println!("User: {} added to queue", user.id);
        if let Err(_) = user.sender.send(Message::Text("You have been added to queue.".into())).await {
            drop(user);
            return;
        };
        self.queue.lock().await.push_back(user);


        let len = self.queue.lock().await.len();
        if len >= GAME_PLAYER_SIZE {
            let users: Vec<User> = self
                .queue
                .lock()
                .await
                .split_off(len - GAME_PLAYER_SIZE)
                .into();
            let original_arr = Arc::clone(&self.queue);

            tokio::spawn(async move {
                let original_arr = original_arr;
                let mut users = users;

                // start game
                let winner = game::SkitGubbe::new(&mut users).run().await;
                if let Ok(Some(winner)) = winner {
                    db_add_winner(winner).await
                }

                // add users back to queue
                original_arr.lock().await.append(&mut users.into());
            });
        }
    }
}

pub struct User {
    pub id: uuid::Uuid,
    pub sender: SplitSink<WebSocket, axum::extract::ws::Message>,
    pub receiver: SplitStream<WebSocket>,
}

impl User {
    pub fn new(socket: WebSocket) -> Self {
        let (sender, receiver) = socket.split();
        Self {
            id: uuid::Uuid::new_v4(),
            sender,
            receiver,
        }
    }

    pub async fn send(&mut self, s: &str) {
        self.sender.send(Message::Text(s.to_string())).await;
    }
}

async fn db_add_winner(user: &User) {
    todo!("notify db of win");
    todo!("compute elo")
}

fn compute_elo() {
    todo!("http://sradack.blogspot.com/2008/06/elo-rating-system-multiple-players.html");
}
