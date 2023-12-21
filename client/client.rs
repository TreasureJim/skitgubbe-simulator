use std::{net::TcpStream, io::{BufReader, BufRead}};

fn main() {
    let addr = std::env::args().skip(1).next().expect("parsing cli address arg");
    let stream = TcpStream::connect(addr).expect("connecting to address");

    let mut reader = BufReader::new(stream.try_clone().unwrap());

    loop {
        let mut s = String::new();
        let bytes_read = reader.read_line(&mut s).unwrap();
        if bytes_read == 0 {
            return;
        }
        print!("{}", s);
    }
}
