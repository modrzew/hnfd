extern crate rustc_serialize;
extern crate ws;
extern crate mio;
extern crate uuid;

// mod models;
mod server;


fn main() {
    // let context = models::Context::new();
    // println!("I has {} cards", context.cards.len());

    server::start();
}
