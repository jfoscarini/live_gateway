use std::{thread, str::from_utf8, sync::mpsc::Sender, net::UdpSocket};

use log::{debug, info};

pub fn udp_main(port: &str, tx: Sender<String>) -> thread::JoinHandle<()> {
    let addr = format!("127.0.0.1:{}", port);
    let udp_server = UdpSocket::bind(&addr).expect("Could not bind address");

    info!(target: "sck", "is listening on {}", addr);

    let mut buf = [0; 1 << 10];
    thread::spawn(move || loop {
        match udp_server.recv_from(&mut buf) {
            Ok((packet_size, source_address)) => {
                if let Ok(msg) = from_utf8(&buf[..packet_size]) {
                    debug!(target: "sck", "{}> {}", source_address, msg);

                    // submit for broadcast
                    tx.send(msg.to_string()).unwrap();
                }
            }
            Err(_) => {}
        }
    })
}
