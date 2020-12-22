extern crate serde_json;
#[macro_use]
extern crate tower_web;

use tower_web::ServiceBuilder;

mod routes;
mod types;

pub fn main() {
    let addr = "127.0.0.1:25580".parse().expect("Port already in use");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(routes::token::TokenResource)
        .run(&addr)
        .unwrap()
}