mod deck;
mod game;
mod game_server;

use std::{net::TcpListener, io::{BufWriter, Write}};

fn main() {
    let address = std::env::args().skip(1).next().unwrap_or("127.0.0.1:0".to_string());

    let server = TcpListener::bind(address).expect("Couldn't bind to address");
    println!("Listening on: {}", server.local_addr().unwrap());

    let (user_stream, _addr) = server.accept().unwrap();

    let mut writer = BufWriter::new(user_stream.try_clone().unwrap());

    writeln!(&mut writer, "hello :D").unwrap();
    writer.flush().unwrap();

    user_stream.shutdown(std::net::Shutdown::Both).unwrap();
}
