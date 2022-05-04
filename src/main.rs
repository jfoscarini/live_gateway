use std::sync::{Arc, Mutex, mpsc::{Sender, Receiver, channel}};

use clap::{Command, Arg};
use clients::Clients;
use udp::udp_main;
use websock::websock_main;

mod client;
mod clients;
mod udp;
mod websock;

fn main() {
    let args =
        Command::new("Live Gateway")
        .version("1.0.0")
        .author("João Pedro Foscarini <jfoscarini@gmail.com>")
        .about("Live Gateway broadcasts on WebSocket what it receives from UDP socket")
        .arg(
            Arg::new("UDP port")
            .short('p')
            .long("udp_port")
            .takes_value(true)
            .required(true)
            .help("UDP Port")
        )
        .arg(
            Arg::new("websocket port")
            .short('w')
            .long("websocket_port")
            .takes_value(true)
            .required(true)
            .help("WebSocket port")
        )
        .get_matches();

    let udp_port = args.value_of("UDP port").unwrap().trim();
    let websock_port = args.value_of("websocket port").unwrap().trim();

    env_logger::init();

    let clients: Arc<Mutex<Clients>> = Arc::new(Mutex::new(Clients::default()));

    // creates a channel for the udp_thread to foward messages for broadcast
    let (tx, rx): (Sender<String>, Receiver<String>) = channel();

    udp_main(udp_port, tx);
    websock_main(websock_port, clients.clone());

    // broadcast message from udp to wsk
    for message in &rx {
        clients.lock().unwrap().broadcast_message(&message);
    }
}
