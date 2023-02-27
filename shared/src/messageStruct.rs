use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Hello {}

#[derive(Serialize, Deserialize)]
pub struct Welcome {
    pub version: u8
}

#[derive(Serialize, Deserialize)]
pub struct Subscribe {
    pub name: String
}

#[derive(Serialize, Deserialize)]
pub enum SubscribeError {
    InvalidName,
    AlreadyRegistered
}

#[derive(Serialize, Deserialize)]
pub enum SubscribeResult {
    Ok,
    Err(SubscribeError)
}

#[derive(Serialize, Deserialize)]
pub struct RecoverSecretIn {
    pub word_count: usize,
    pub letters: String,
    pub tuple_size: Vec<usize>
}

#[derive(Serialize, Deserialize)]
pub struct RecoverSecretOut {
    pub secret: String
}

#[derive(Serialize, Deserialize)]
pub enum Challenge {
    RecoverSecret(RecoverSecretIn)
}

#[derive(Serialize, Deserialize)]
pub enum ChallengeAnswer {
    RecoverSecret(RecoverSecretOut)
}

#[derive(Serialize, Deserialize)]
pub struct ChallengeResult {
    pub answer: ChallengeAnswer,
    pub next_target: String
}

#[derive(Serialize, Deserialize)]
pub struct ChallengeTimeout {
    pub message: String
}


#[derive(Serialize, Deserialize)]
pub enum ChallengeValue {
    Unreachable,
    Timeout,
    BadResult { used_time: f64, next_target: String },
    Ok { used_time: f64, next_target: String }
}

#[derive(Serialize, Deserialize)]
pub struct ReportedChallengeResult {
    pub name: String,
    pub value: ChallengeValue
}

#[derive(Serialize, Deserialize)]
pub struct RoundSummary {
    pub challenge: String,
    pub chain: Vec<ReportedChallengeResult>
}

#[derive(Serialize, Deserialize)]
pub struct PublicPlayer {
    name: String,
    stream_id: String,
    score: i32,
    steps: u32,
    is_active: bool,
    total_used_time: f64
}

#[derive(Serialize, Deserialize)]
pub struct PublicLeaderBoard {
    pub playerList: Vec<PublicPlayer>
}

#[derive(Serialize, Deserialize)]
pub struct EndOfGame {
    pub leader_board: PublicLeaderBoard
}


#[derive(Serialize, Deserialize)]
pub enum Message {
    Hello,
    Welcome(Welcome),
    Subscribe(Subscribe),
    PublicLeaderBoard(Vec<PublicPlayer>),
    Challenge(Challenge),
    ChallengeResult(ChallengeResult),
    ChallengeTimeout(ChallengeTimeout),
    RoundSummary(RoundSummary),
    EndOfGame(EndOfGame)
}
