#![feature(async_closure)]

use core::panic;
use std::sync::Mutex;

use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{self, tungstenite::Message};
use url;

use skitgubbe_game::{
    api::{player_messages, server_messages},
    deck::Card,
};

#[tokio::main]
async fn main() {
    let addr = std::env::args().nth(1).expect("No arg for address");
    let url = url::Url::parse(&(format!("ws://{addr}/queue"))).expect("Invalid ws url");

    let (mut stream, _) = tokio_tungstenite::connect_async(&url)
        .await
        .expect("connecting to address");
    println!("Connected to {addr}");

    //              GET ID
    let our_id = serde_json::from_str::<server_messages::ServerNotification>(
        &stream.next().await.unwrap().unwrap().to_string(),
    )
    .expect("expect server notification game start");
    let server_messages::ServerNotification::Id(our_id) = our_id else {
        panic!("message isn't id");
    };
    println!("ID is: {our_id}");

    // Wait for game to start
    let s_notif = serde_json::from_str::<server_messages::ServerNotification>(
        &stream.next().await.unwrap().unwrap().to_string(),
    )
    .expect("expect server notification game start");

    let server_messages::ServerNotification::GameStart(player_ids) = s_notif else {
        panic!("server notif not game start");
    };
    let other_player_ids: Vec<String> = player_ids.into_iter().filter(|id| *id != our_id).collect();

    //              GAME STARTED
    println!("Game starting");

    //              SWAP STAGE
    let stage = serde_json::from_str::<server_messages::Stage>(
        &stream.next().await.unwrap().unwrap().to_string(),
    )
    .expect("expect game stage swap");
    if let server_messages::Stage::Swap = stage {
        println!("Swap stage");
    } else {
        panic!("stage not swap");
    }

    let mut cards;

    loop {
        cards = serde_json::from_str::<server_messages::Cards>(
            &stream.next().await.unwrap().unwrap().to_string(),
        )
        .expect("expect cards notif");
        if cards.owner_id != our_id {
            panic!("cards not our own");
        }

        //          CARD SWAPPING STRATEGY

        let hand_high = cards
            .hand
            .iter()
            .max()
            .expect("we should never have an empty hand");
        let bottom_min = cards
            .bottom_cards
            .iter()
            .enumerate()
            .map(|(i, vec)| {
                (
                    i,
                    vec.get(0)
                        .expect("Each bottom card stack should never be empty"),
                )
            })
            .min_by_key(|&(_, card)| card)
            .unwrap();

        // if we have a higher card than one of the cards below then swap as many as we can
        if hand_high > bottom_min.1 {
            // count how many we have
            let num_cards = cards
                .hand
                .iter()
                .filter(|card| card.rank == hand_high.rank)
                .count();
            let swap_cards = vec![hand_high.clone(); num_cards];
            let msg = player_messages::action::SetupAction::ExchangeCard {
                hand: swap_cards,
                bottom: bottom_min.0,
            };

            let _ = stream
                .send(Message::Text(serde_json::to_string(&msg).unwrap()))
                .await;
            println!("Swapping high card");

            continue;
        }

        // if able to compound card then do it
        let compoundable_card_hand_indexes: Option<(usize, Vec<&Card>)> = cards
            .bottom_cards
            .iter()
            .enumerate()
            .find_map(|(bottom_card_index, vis_card_vec)| {
                let vis_card = &vis_card_vec[0];

                let matching_cards = cards
                    .hand
                    .iter()
                    .filter(|hand_card| hand_card.rank == vis_card.rank)
                    .collect::<Vec<&Card>>();

                if !matching_cards.is_empty() {
                    return Some((bottom_card_index, matching_cards));
                }
                None
            });
        if let Some((bottom_index, compound_cards)) = compoundable_card_hand_indexes {
            let msg = player_messages::action::SetupAction::CompoundCard {
                hand: compound_cards.iter().map(|&card| card.clone()).collect(),
                bottom: bottom_index,
            };
            let _ = stream
                .send(Message::Text(serde_json::to_string(&msg).unwrap()))
                .await;
            println!("compounding cards");

            continue;
        }

        break;
    }

    let _ = stream
        .send(Message::Text(
            serde_json::to_string(&player_messages::action::SetupAction::FinishExchange).unwrap(),
        ))
        .await;
    println!("Finished exchange");

    let mut all_cards = serde_json::from_str::<Vec<server_messages::Cards>>(
        &stream.next().await.unwrap().unwrap().to_string(),
    )
    .expect("expect all cards status");
    cards = all_cards.remove(
        all_cards
            .iter()
            .position(|other_cards| other_cards.owner_id == our_id)
            .expect("our cards should be in list"),
    );
    println!("Received everyones cards");

    let stage = serde_json::from_str::<server_messages::Stage>(
        &stream.next().await.unwrap().unwrap().to_string(),
    )
    .expect("expect game stage play");
    if let server_messages::Stage::Play = stage {
        println!("Play stage");
    } else {
        panic!("stage not play");
    }

    //                  CARD PLAYING STRATEGY

    // loop {
    //     todo!();
    // }

    loop {
        println!(
            "Received message: {}",
            stream.next().await.unwrap().unwrap()
        );
    }
}
