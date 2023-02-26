use std::env;
use std::net::{TcpStream, TcpListener};
use std::io::{BufReader, BufWriter, Write, Read};

fn main() {

    let server_address = "0.0.0.0:7878";
    let mut stream = TcpStream::connect(server_address).unwrap();
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    // Send Hello World to Server
    let data = b"Hello, world!";
    writer.write(data).unwrap();
    writer.flush().unwrap();

    // Listen res from server
    let mut response = Vec::new();
    reader.read_to_end(&mut response).unwrap();
    println!("Response from server: {:?}", response);
}
