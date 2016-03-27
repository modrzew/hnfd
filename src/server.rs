use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

use mio;
use rustc_serialize::json;
use uuid::Uuid;
use ws;

use game::{Game, Player};
use models;

enum MessageKind {
    Standard,
    NewPlayer,
    Quit,
}

struct Message {
    client_id: mio::Token,
    content: Option<String>,
    ws: Option<ws::Sender>,
    kind: MessageKind,
}

struct Server {
    receiver: mpsc::Receiver<Message>,
    clients: HashMap<mio::Token, ws::Sender>,
    games: Vec<Game>,
    cards: Vec<models::Card>,
}

impl Server {
    fn new(receiver: mpsc::Receiver<Message>, cards: Vec<models::Card>) -> Server {
        Server {
            receiver: receiver,
            clients: HashMap::new(),
            games: Vec::new(),
            cards: cards,
        }
    }

    fn handle_new_player(&mut self, msg: Message) {
        let mut should_start = false;
        // Add client on new connection
        if let Some(ws) = msg.ws {
            if self.clients.len() < 2 {
                self.clients.insert(msg.client_id, ws);
            } else {
                ws.send("Only 2 players supported at this time");
                ws.close_with_reason(ws::CloseCode::Normal, "Only 2 players supported at this time").unwrap();
            }
            self.games[0].add_player(msg.client_id);
            if self.clients.len() == 2 {
                should_start = true;
            }
        }
        if should_start {
            self.games[0].start();
            let current = self.clients.get(&msg.client_id).unwrap();
            let opponent = self.clients.get(&self.games[0].get_other(msg.client_id)).unwrap();
            let to_current = self.games[0].get_start_message(1);
            let to_opponent = self.games[0].get_start_message(0);
            self.send(&current, &opponent, to_current, to_opponent);
        }
    }

    fn handle_quit(&mut self) {
    }

    fn handle_standard(&mut self, msg: Message) {
        // Remove client if
        let current = self.clients.get(&msg.client_id).unwrap();
        // Less than 2 players?
        if self.clients.len() < 2 {
            current.send("Please wait for another player");
            return;
        }
        // Now we're 100% at the point where there are 2 players and we can
        // send messages to both of them
        let opponent = self.clients.get(&self.games[0].get_other(msg.client_id)).unwrap();
        let (mut to_current, mut to_opponent) = ("".to_string(), "".to_string());
        let dupa = msg.content.unwrap();
        let content = dupa.trim();
        // Skip empty messages
        if content.len() == 0 {
            return;
        }
        // Handle message
        if let Some(temp) =self.games[0].handle(msg.client_id, content, &self.cards) {
            to_current = temp.0;
            to_opponent = temp.1;
        }
        // Debug
        println!("{}: {}", msg.client_id.as_usize(), content);
        if to_current != "" && to_opponent != "" {
            self.send(&current, &opponent, to_current, to_opponent);
        }
    }

    fn run(&mut self) {
        println!("Initialized server");
        self.games.push(Game::new());
        while let Ok(msg) = self.receiver.recv() {
            match msg.kind {
                MessageKind::NewPlayer => self.handle_new_player(msg),
                MessageKind::Quit => self.handle_quit(),
                MessageKind::Standard => self.handle_new_player(msg),
            }
        }
    }

    fn send(&self, current: &ws::Sender, opponent: &ws::Sender, to_current: String, to_opponent: String) {
        current.send(json::encode(&to_current).unwrap());
        opponent.send(json::encode(&to_opponent).unwrap());
    }
}

struct SingleClientHandler {
    ws: ws::Sender,
    sender: mpsc::Sender<Message>,
}

impl ws::Handler for SingleClientHandler {
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        self.sender.send(Message {
            client_id: self.ws.token(),
            kind: MessageKind::NewPlayer,
            content: None,
            ws: Some(self.ws.clone()),
        }).unwrap();
        Ok(())
    }

    fn on_close(&mut self, code:ws::CloseCode, reason: &str) {
        self.sender.send(Message {
            client_id: self.ws.token(),
            kind: MessageKind::NewPlayer,
            content: None,
            ws: None,
        }).unwrap();
        println!("Connection closing due to ({:?}) {}", code, reason);
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        self.sender.send(Message {
            client_id: self.ws.token(),
            kind: MessageKind::Standard,
            content: Some(msg.as_text().unwrap().to_string()),
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
        }
    }).unwrap();
}
