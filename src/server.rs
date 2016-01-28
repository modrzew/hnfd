use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

use rustc_serialize::json;
use ws;
use uuid::Uuid;

struct Message {
    client_id: String,
    content: String,
    ws: Option<ws::Sender>,
}

struct Server {
    receiver: mpsc::Receiver<Message>,
    clients: HashMap<String, ws::Sender>
}

impl Server {
    fn new(receiver: mpsc::Receiver<Message>) -> Server {
        Server {
            receiver: receiver,
            clients: HashMap::new()
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
                }
            }
            println!("{}: {}", msg.client_id, msg.content);
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
    let (message_sender, message_receiver): (mpsc::Sender<Message>, mpsc::Receiver<Message>) = mpsc::channel();
    let mut server = Server::new(message_receiver);

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
