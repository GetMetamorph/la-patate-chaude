use std::env;
use std::net::{TcpStream, TcpListener};
use std::io::{BufReader, BufWriter, Write, Read};
use serde_json;

fn main() {

    let stream = TcpStream::connect("127.0.0.1:7878");
    match stream {
        Ok(mut stream) => {
            let message = "Hello";
            let buf = message.as_bytes();
            let n = buf.len() as u32;
            let buf_n = n.to_be_bytes();
            stream.write(&buf_n).unwrap();
            stream.write(&buf).unwrap();
        }
        Err(err) => { panic!("ERROR: {err}") }
    }


    //let mut reader = BufReader::new(&stream);
    //let mut writer = BufWriter::new(&stream);

    // Send Hello World to Server
    //let data = b"Hello";
    //writer.write(data).unwrap();
    //writer.flush().unwrap();

    // Listen res from server
    //let mut response = Vec::new();
    //reader.read_to_end(&mut response).unwrap();
    //println!("Response from server: {:?}", response);
}
