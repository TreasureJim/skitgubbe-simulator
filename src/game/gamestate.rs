use crate::api::server_messages::{self, BottomCards, GameState};

use super::SkitGubbe;

impl SkitGubbe {
    async fn get_all_players_vis_cards(&self) -> Vec<(String, BottomCards)> {
        let mut other_player_cards = vec![];

        for player in self.players.iter() {
            let player = player.lock().await;
            other_player_cards.push((
                player.user.lock().await.id.to_string(),
                player.visible_cards(),
            ));
        }

        other_player_cards
    }

    pub async fn send_playing_game_state(&self, cur_player_index: usize) {
        let turn = self.players[cur_player_index as usize]
            .lock()
            .await
            .user
            .lock()
            .await
            .id
            .to_string();

        let other_player_cards = self.get_all_players_vis_cards().await;

        for player in self.players.iter() {
            let player = player.lock().await;

            let message = GameState {
                turn: turn.clone(),
                stage: server_messages::Stage::Play,
                cards: player.to_server_player_cards(),
                stack: self.playing_stack.clone(),
                other_players: other_player_cards.clone(),
            };
            let _ = player
                .user
                .lock()
                .await
                .send(&serde_json::to_string(&message).unwrap())
                .await;
        }
    }
}
