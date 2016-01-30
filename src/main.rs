extern crate rustc_serialize;
extern crate ws;
extern crate mio;
extern crate uuid;

mod models;
mod server;
mod engine;


fn main() {
    server::start();
}
