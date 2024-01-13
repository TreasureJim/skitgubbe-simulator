mod deck;
mod game;
mod game_manager;

use axum::{
    extract::{
        ws::WebSocket,
        State, WebSocketUpgrade,
    },
    http::Method,
    response::IntoResponse,
    routing::get,
    Router,
};
use game_manager::User;
use std::sync::{Arc, Mutex};
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
        .route("/ws", get(handler))
        .with_state(queue_state)
        .layer(cors);

    axum::serve(server, router).await.unwrap();
}

async fn handler(ws: WebSocketUpgrade, State(state): State<Arc<Mutex<ServerQueue>>>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<Mutex<ServerQueue>>) {
    let user = User::new(socket);
    state.lock().unwrap().push_user(user);
}
