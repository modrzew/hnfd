extern crate rustc_serialize;
extern crate ws;
extern crate mio;
extern crate uuid;

mod models;
mod server;
mod engine;
mod game;


fn main() {
    server::start();
}
