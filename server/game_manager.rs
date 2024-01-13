use std::collections::VecDeque;

use axum::extract::ws::WebSocket;
use futures::{
    stream::{SplitSink, SplitStream},
    StreamExt,
};

use crate::game;

pub struct ServerQueue {
    queue: VecDeque<User>,
}

const GAME_PLAYER_SIZE: usize = 3;

impl ServerQueue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    pub async fn push_user(&mut self, user: User) {
        self.queue.push_back(user);

        let len = self.queue.len();
        if len >= GAME_PLAYER_SIZE {
            let users: Vec<_> = self.queue.split_off(len - GAME_PLAYER_SIZE).into();
            // start game
            let winner = game::SkitGubbe::new(&users).run().await;
            if let Some(winner) = winner {
                db_add_winner(winner).await
            }
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
}

async fn db_add_winner(user: &User) {
    todo!("notify db of win");
}
