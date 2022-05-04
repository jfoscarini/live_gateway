use tungstenite::Message;

use crate::client::Client;

#[derive(Default)]
pub struct Clients {
    clients: Vec<Client>,
    has_dead_clients: bool,
}

impl Clients {
    pub fn push_client(&mut self, new_client: Client) -> &mut Self { self.clients.push(new_client); self }

    pub fn broadcast_message(&mut self, message: &str) -> &mut Self {
        for client in self.clients.iter_mut() {
            client.send_message(Message::text(message));
        }

        self
    }

    pub fn read_messages(&mut self) -> &mut Self {
        for client in self.clients.iter_mut() {
            if let Some(message) = client.receive_message() {
                if message.is_close() { self.has_dead_clients = true; }
            }
        }

        self
    }

    pub fn drop_dead_connections(&mut self) -> &mut Self {
        if self.has_dead_clients {
            self.clients.retain(|c| { c.is_alive() });
        }

        self
    }
}
