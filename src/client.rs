use std::{net::{TcpStream, SocketAddr}};

use log::{debug, error, info};
use tungstenite::{WebSocket, Message};

pub struct Client {
    websock: WebSocket<TcpStream>,
    addr: SocketAddr,
    dead: bool,
}

impl Client {
    pub fn new(websock: WebSocket<TcpStream>, addr: SocketAddr) -> Client {
        Client {
            websock,
            addr,
            dead: false,
        }
    }

    pub fn send_message(&mut self, message: Message) -> Option<()> {
        if self.is_dead() { return None; };

        match self.websock.write_message(message) {
            Ok(()) => { Some(()) }
            Err(e) => { error!(target: "wsk", "{}> {:?}", self.addr, e); None }
        }
    }

    pub fn receive_message(&mut self) -> Option<Message> {
        if self.is_dead() { return None };

        match self.websock.read_message() {
            Ok(message) => {
                if message.is_close() {
                    self.dead = true;
                    info!(target: "wsk", "{} has disconnected", self.addr);

                    Some(message)
                } else if message.is_text() {
                    debug!(target: "wsk", "{}> {}", self.addr, message.to_string());

                    Some(message)
                } else { None }
            }
            Err(tungstenite::Error::AlreadyClosed) => { self.dead = true; None }
            Err(tungstenite::Error::ConnectionClosed) => { self.dead = true; None }
            Err(tungstenite::Error::Io(ref e)) if e.kind() == std::io::ErrorKind::WouldBlock => { None }
            Err(e) => { error!(target: "wsk", "{}> {:?}", self.addr, e); None }
        }
    }

    pub fn is_dead(&self) -> bool { self.dead }
    pub fn is_alive(&self) -> bool { !self.dead }
}
