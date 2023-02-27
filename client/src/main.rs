use std::io::{Read, Write};
use std::mem::transmute;
use std::net::TcpStream;

use shared::messageStruct::{*};


pub fn send(mut stream: &TcpStream, message: Message) {
    let serialized = serde_json::to_string(&message).expect("Fail serialize");
    let serialized_size = serialized.len() as u32;
    stream.write(&serialized_size.to_be_bytes()).unwrap();
    stream.write(&serialized.as_bytes()).unwrap();
}




fn main() {


    let stream = TcpStream::connect("127.0.0.1:7878");

    match stream {
        Ok(mut stream) => {
            let mut buf_n = [0u8; 4];
            send(&stream, Message::Hello);

            loop {
                match stream.read_exact(&mut buf_n) {
                    Ok(_) => {}
                    Err(_) => {}
                }
                let n = u32::from_be_bytes(buf_n);
                let mut buf = Vec::<u8>::new();
                buf.resize(n as usize, 0);
                let s = stream.read(&mut buf).expect("Error read");
                let msg = String::from_utf8_lossy(&buf);
                println!("Receive message {msg} with size {s}");

                match serde_json::from_str(&msg).expect("Fail serialize") {
                    Message::Hello => { }
                    Message::Welcome(welcome) => {
                        send(&stream, Message::Subscribe(Subscribe { name: "test".to_string() }))
                    }
                    Message::Subscribe(_) => { }
                    Message::SubscribeResult(sub) => {
                        let code = match sub {
                            SubscribeResult::Ok => { 0 }
                            SubscribeResult::Err(ref err) => { 1 }
                        };
                        if code == 1 { break; }
                    }
                    Message::PublicLeaderBoard(lead) => {}
                    Message::Challenge(lead) => {}
                    Message::ChallengeTimeout(lead) => {}
                    Message::ChallengeResult(lead) => {}
                    Message::RoundSummary(lead) => {}
                    Message::EndOfGame(lead) => { break; }
                }
            }
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
