mod deck;
mod game;

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
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};
use tokio::{self, net::TcpListener};
use tower_http::cors::{Any, CorsLayer};

struct ServerQueue {
    queue: Mutex<VecDeque<User>>,
}

use uuid;

pub struct User {
    id: uuid::Uuid,
    socket: WebSocket
}

impl User {
    pub fn new(socket: WebSocket) -> Self {
        Self { id: uuid::Uuid::new_v4(), socket }
    }
}

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

    let queue_state = Arc::new(ServerQueue {
        queue: Mutex::new(VecDeque::new()),
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET]);

    let router = Router::new()
        .route("/ws", get(handler))
        .with_state(queue_state)
        .layer(cors);

    axum::serve(server, router).await.unwrap();
}

async fn handler(ws: WebSocketUpgrade, State(state): State<Arc<ServerQueue>>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<ServerQueue>) {
    let user = User::new(socket);
    state.queue.lock().unwrap().push_back(user);
}
