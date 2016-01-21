use std::sync::mpsc;
use std::thread;

use rustc_serialize::json;
use ws;
use uuid::Uuid;

struct Server {
    receiver: mpsc::Receiver<String>,
}

struct SingleClientHandler {
    ws: ws::Sender,
    id: String,
    sender: mpsc::Sender<String>,
}

impl ws::Handler for SingleClientHandler {
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        self.id = Uuid::new_v4().to_simple_string();
        Ok(())
    }
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        self.sender.send(msg.as_text().unwrap().to_string());
        println!("Something sent");
        Ok(())
    }
}

pub fn start() {
    let (sender, receiver): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
    // let server = Server {
    //     receiver: receiver,
    // };
    thread::spawn(move || {
        println!("Initialized listener");
        while let Ok(msg) = receiver.recv() {
            println!("Got {}", msg);
        }
    });

    ws::listen("127.0.0.1:3012", |out| {
        SingleClientHandler{
            ws: out,
            sender: sender.clone(),
            id: "".to_string()
        }
    }).unwrap();
}
