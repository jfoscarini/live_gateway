use std::{sync::{Arc, Mutex}, thread::{self, sleep}, net::TcpListener, time::Duration};

use log::{info, error};

use crate::{clients::Clients, client::Client};

pub fn websock_main(port: &str, clients: Arc<Mutex<Clients>>) -> thread::JoinHandle<()> {
    let addr = format!("127.0.0.1:{}", port);
    let websock_server = TcpListener::bind(&addr).unwrap();
    websock_server.set_nonblocking(true).unwrap();

    info!(target: "wsk", "is listening on {}", addr);

    thread::spawn(move || {
        for maybe_stream in websock_server.incoming() {
            // avoid MidHandshake interrupt
            sleep(Duration::from_millis(10));

            match maybe_stream {
                Ok(client) => {
                    let addr = client.peer_addr().unwrap();

                    match tungstenite::accept(client) {
                        Ok(tungstenite_client) => {
                            clients.lock().unwrap().push_client(Client::new(tungstenite_client, addr));
                            info!(target: "wsk", "{} has connected", addr);
                        }
                        Err(tungstenite::HandshakeError::Interrupted(e)) => {
                            error!(target: "wsk interrupt", "{}: {:?}", addr, e);
                        }
                        Err(tungstenite::HandshakeError::Failure(e)) => {
                            error!(target: "wsk failure", "{}: {:?}", addr, e);
                        }
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => { // there is no new client
                    clients.lock().unwrap().read_messages().drop_dead_connections();
                }
                Err(e) => { error!(target: "wsk", "{:?}", e); }
            }
        }
    })
}
