use std::io::{Read, Write};
use std::net::TcpStream;

use shared::messageStruct::{Challenge, ChallengeResult, ChallengeTimeout, EndOfGame, Hello, PublicLeaderBoard, RoundSummary, Subscribe, Welcome, Message};



fn main() {


    let stream = TcpStream::connect("127.0.0.1:7878");

    match stream {
        Ok(mut stream) => {
            let mut buf_n = [0u8; 4];


            let serialized = serde_json::to_string(&mut Message::Hello).expect("failed to serialized object");
            let serialized_size = serialized.len() as u32;
            stream.write(&serialized_size.to_be_bytes()).unwrap();
            stream.write(&serialized.as_bytes()).unwrap();

            stream.read_exact(&mut buf_n).unwrap();
            let n = u32::from_be_bytes(buf_n);
            let mut buf = Vec::<u8>::new();
            buf.resize(n as usize, 0);
            let s = stream.read(&mut buf).expect("Cannot read");
            let msg = String::from_utf8_lossy(&buf);
            println!("Receive message {msg} with size {s}");

//            let serialized = serde_json::to_string().expect("failed to serialized object");
//            let serialized_size = serialized.len() as u32;
//            stream.write(&serialized_size.to_be_bytes()).unwrap();
//            stream.write(&serialized.as_bytes()).unwrap();

//            stream.read_exact(&mut buf_n).unwrap();
//            let n = u32::from_be_bytes(buf_n);
//            buf.resize(n as usize, 0);
//            let s = stream.read(&mut buf).expect("Cannot read");
//           let msg = String::from_utf8_lossy(&buf);
//            println!("Receive message {msg} with size {s}");


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
