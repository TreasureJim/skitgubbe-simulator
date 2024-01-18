use tokio_tungstenite;
use url;
use futures_util::{SinkExt, StreamExt};
use log::*;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Error, Result},
};
use futures_util::StreamExt;

#[tokio::main]
async fn main() {
    let addr = std::env::args().nth(1).expect("parsing cli address arg");
    let url = url::Url::parse(&(format!("ws://{addr}/queue"))).expect("Invalid ws url");

    let (stream, _) = tokio_tungstenite::connect_async(&url)
        .await
        .expect("connecting to address");
    println!("Connected to {addr}");

    let (mut read , mut write) = stream.split();

    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => println!("Received a message: {}", msg),
            Err(e) => eprintln!("Error receiving message: {}", e),
        }
    }
}
