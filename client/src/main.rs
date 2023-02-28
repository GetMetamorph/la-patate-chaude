use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;
use shared::challenge::Challenge as HashChallenge;
use shared::hash_code::Hash;

use shared::message_struct::{*};
use shared::recover_secret::Recover;


struct Game {
    name: String,
    players: Vec<PublicPlayer>
}

pub fn send(mut stream: &TcpStream, message: Message) {
    let serialized = serde_json::to_string(&message).expect("Fail serialize");
    let serialized_size = serialized.len() as u32;
    stream.write(&serialized_size.to_be_bytes()).unwrap();
    stream.write(&serialized.as_bytes()).unwrap();
}


fn resolve_challenge(mut stream: &TcpStream, challenge: Challenge, game: &mut Game, name: String) {
    match challenge {
        Challenge::RecoverSecret(recover) => {
            let input = Recover::new(recover);
            let output = input.solve();
            let challenge_result = ChallengeAnswer::RecoverSecret(output);
            send(stream, Message::ChallengeResult(ChallengeResult{answer: challenge_result, next_target: "test1".to_string()}))
        }
        Challenge::MD5HashCash(hashcode) => {
            let hash = Hash::new(hashcode);
            let result = hash.solve();
            let challenge_result = ChallengeAnswer::MD5HashCash(result);
            send(stream, Message::ChallengeResult(ChallengeResult{ answer: challenge_result, next_target: "test1".to_string() }))
        }
        Challenge::ChallengeTimeout(timeout) => { }
    }
}


fn loop_listening(mut stream: &TcpStream, game: &mut Game) {
    let mut buf_n = [0u8; 4];

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
            Message::Welcome(_) => {
                send(&stream, Message::Subscribe(Subscribe { name: game.name.clone() }))
            }
            Message::Subscribe(_) => { }
            Message::SubscribeResult(sub) => {
                let code = match sub {
                    SubscribeResult::Ok => { 0 }
                    SubscribeResult::Err(ref err) => { 1 }
                };
                if code == 1 { break; }
            }
            Message::PublicLeaderBoard(lead) => {println!("Leaderboard {lead:?}")}
            Message::Challenge(challenge) => {resolve_challenge(&stream, challenge, game, game.name.clone()) }
            Message::ChallengeTimeout(_) => {
                println!("test timeout");
                break;
            }
            Message::ChallengeResult(_) => {}
            Message::RoundSummary(_) => {}
            Message::EndOfGame(_) => {
                println!("test end");
                break;
            }
        }
    }
}





fn main() {

    let args: Vec<String> = env::args().collect();
    let name = String::from(&args[1]);

    let stream = TcpStream::connect("127.0.0.1:7878");

    match stream {
        Ok(mut stream) => {
            send(&stream, Message::Hello);

            let mut game = Game {
                name,
                players: vec![],
            };
            loop_listening(&stream, &mut game);
        }
        Err(err) => { panic!("ERROR: {err}") }
    }

}