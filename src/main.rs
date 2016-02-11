extern crate mio;
extern crate rand;
extern crate rustc_serialize;
extern crate uuid;
extern crate ws;

mod models;
mod server;
mod engine;
mod game;
mod messages;


fn main() {
    server::start();
}
