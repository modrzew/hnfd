use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

// use rustc_serialize::json;
use ws;
use uuid::Uuid;

use models;

struct Message {
    client_id: String,
    content: String,
    ws: Option<ws::Sender>,
}

struct Server {
    receiver: mpsc::Receiver<Message>,
    clients: HashMap<String, ws::Sender>,
    cards: Vec<models::Card>,
}

impl Server {
    fn new(receiver: mpsc::Receiver<Message>, cards: Vec<models::Card>) -> Server {
        Server {
            receiver: receiver,
            clients: HashMap::new(),
            cards: cards,
        }
    }

    fn run(&mut self) {
        println!("Initialized server");
        while let Ok(msg) = self.receiver.recv() {
            // Add client on new connection
            if let Some(ws) = msg.ws {
                self.clients.insert(msg.client_id, ws);
                continue;
            }
            let content = msg.content.trim();
            // Skip empty messages
            if content.len() == 0 {
                continue;
            }
            // Notify all other clients
            for (id, client) in &self.clients {
                if id.to_string() != msg.client_id {
                    client.send(content).unwrap();
                } else {
                    // Tell something about card if that's an int
                    if let Ok(card_id) = content.parse::<usize>() {
                        let card = &self.cards[card_id];
                        client.send(card.get_info()).unwrap();
                    }
                }
            }
            println!("{}: {}", msg.client_id, content);
        }
    }
}

struct SingleClientHandler {
    ws: ws::Sender,
    id: String,
    sender: mpsc::Sender<Message>,
}

impl ws::Handler for SingleClientHandler {
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        self.id = Uuid::new_v4().to_simple_string();
        self.sender.send(Message {
            client_id: self.id.to_string(),
            content: "".to_string(),
            ws: Some(self.ws.clone()),
        }).unwrap();
        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        self.sender.send(Message {
            client_id: self.id.to_string(),
            content: msg.as_text().unwrap().to_string(),
            ws: None
        }).unwrap();
        Ok(())
    }
}

pub fn start() {
    let cards = models::get_cards();
    let (message_sender, message_receiver): (mpsc::Sender<Message>, mpsc::Receiver<Message>) = mpsc::channel();
    let mut server = Server::new(message_receiver, cards);

    thread::spawn(move || {
        server.run()
    });

    ws::listen("127.0.0.1:3012", |out| {
        SingleClientHandler{
            ws: out,
            sender: message_sender.clone(),
            id: "".to_string()
        }
    }).unwrap();
}
