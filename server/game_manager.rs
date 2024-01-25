use std::{collections::VecDeque, sync::Arc};

use axum::extract::ws::Message;
use futures::{
    lock::Mutex,
    SinkExt,
};

use skitgubbe_game::game;
use skitgubbe_game::user::User;

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

    pub async fn push_user(&mut self, user: User) {
        println!("User: {} added to queue", user.id);
        // if let Err(_) = user
        //     .sender
        //     .send(Message::Text("Waiting in queue".into()))
        //     .await
        // {
        //     drop(user);
        //     return;
        // };
        self.queue.lock().await.push_back(user);

        let len = self.queue.lock().await.len();
        if len >= GAME_PLAYER_SIZE {
            // TODO: Check all players are connected
            let users: Vec<User> = self
                .queue
                .lock()
                .await
                .split_off(len - GAME_PLAYER_SIZE)
                .into();
            let original_arr = Arc::clone(&self.queue);

            tokio::spawn(async move {
                let original_arr = original_arr;
                let users: Vec<_> = users
                    .into_iter()
                    .map(|user| Arc::new(Mutex::new(user)))
                    .collect();

                // start game
                let winner =
                    game::SkitGubbe::new(users.iter().map(|user| Arc::clone(user)).collect())
                        .run()
                        .await;
                if let Ok(Some(winner)) = winner {
                    db_add_winner(&*users[winner].lock().await).await
                }

                // TODO: If error check which player caused it and don't add back to queue

                // add users back to queue
                let mut users = users
                    .into_iter()
                    .map(|user| {
                        Arc::into_inner(user).expect("Someone else has a reference to this user").into_inner()
                    })
                    .collect();

                original_arr.lock().await.append(&mut users);
                todo!("trigger queue")
            });
        }
    }
}


async fn db_add_winner(user: &User) {
    compute_elo();
    todo!("compute elo & notify db of win");
}

fn compute_elo() {
    todo!("http://sradack.blogspot.com/2008/06/elo-rating-system-multiple-players.html");
}
