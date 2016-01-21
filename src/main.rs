extern crate rustc_serialize;
extern crate websocket;

use rustc_serialize::json;

mod models;
mod server;


// fn hello_world(_: &mut Request) -> IronResult<Response> {
//     Ok(Response::with((
//         status::Ok,
//         json::encode(&vec![1, 2, 3, 4]).unwrap(),
//     )))
// }

fn main() {
    let context = models::Context::new();
    println!("I has {} cards", context.cards.len());

    server::start();

    // println!("On 3000");
    // Iron::new(hello_world).http("localhost:3000").unwrap();
}
