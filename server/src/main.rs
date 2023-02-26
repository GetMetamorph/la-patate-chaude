use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufWriter};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);
    }
}
