extern crate rustc_serialize;
extern crate iron;

use iron::prelude::*;
use iron::status;
use rustc_serialize::json;

mod models;



fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((
        status::Ok,
        json::encode(&vec![1, 2, 3, 4]).unwrap(),
    )))
}

fn main() {
    let context = models::initialize_context();
    println!("I has {} cards", context.cards.len());

    // println!("On 3000");
    // Iron::new(hello_world).http("localhost:3000").unwrap();
}
