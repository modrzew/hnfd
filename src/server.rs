use std::thread;

use websocket::{Server, Message, Sender, Receiver};
use websocket::message::Type;
use websocket::header::WebSocketProtocol;


pub fn start() {
    let server = Server::bind("127.0.0.1:2794").unwrap();
    for connection in server {
        // OMG this shouldn't be separate thread for each connection
        thread::spawn(move || {
            let request = connection.unwrap().read_request().unwrap();
            let headers = request.headers.clone();

            request.validate().unwrap();

            let mut response = request.accept();
            let mut client = response.send().unwrap();
            let ip = client.get_mut_sender()
                .get_mut()
                .peer_addr()
                .unwrap();
            println!("Connection from {}", ip);
            let message = Message::text("Hello");
            client.send_message(&message).unwrap();
            let (mut sender, mut receiver) = client.split();
            let message = Message::close();
            sender.send_message(&message).unwrap();
            println!("Client {} disconnected", ip);
        });
    }
}
