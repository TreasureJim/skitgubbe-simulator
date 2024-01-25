#![feature(async_closure)]

mod game_manager;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    http::Method,
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::lock::Mutex;
use futures_util::SinkExt;
use skitgubbe_game::user::User;
use std::sync::Arc;
use tokio::{self, net::TcpListener};
use tower_http::cors::{Any, CorsLayer};

use crate::game_manager::ServerQueue;

#[tokio::main]
async fn main() {
    let port;
    {
        let port_env = std::env::var("PORT");
        if port_env.is_ok() {
            port = port_env
                .unwrap()
                .parse::<u16>()
                .expect("PORT env var is not a number");
        } else {
            // default
            eprintln!("No PORT env var set defaulting to port 0");
            port = 0;
        }
    }

    let address = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    let server = TcpListener::bind(address)
        .await
        .expect("Couldn't bind to address");
    println!("Listening on: {}", server.local_addr().unwrap());

    let queue_state = Arc::new(Mutex::new(ServerQueue::new()));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET]);

    let router = Router::new()
        .route("/queue", get(handler))
        .with_state(queue_state)
        .layer(cors);

    axum::serve(server, router).await.unwrap();
}

async fn handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<Mutex<ServerQueue>>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

use skitgubbe_game::api::server_messages::ServerNotification;

async fn handle_socket(socket: WebSocket, state: Arc<Mutex<ServerQueue>>) {
    let mut user = User::new(socket);

    let server_id_msg = ServerNotification::Id(user.id.to_string());
    let _ = user
        .sender
        .send(Message::Text(serde_json::to_string(&server_id_msg).unwrap()))
        .await;

    state.lock().await.push_user(user).await;
}
