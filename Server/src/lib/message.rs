use chrono::{ DateTime, Local };
use std::net::SocketAddr;

#[derive(Debug)]
pub struct Message {
    pub from: SocketAddr,
    pub text: String,
    pub time: DateTime<Local>,
}

