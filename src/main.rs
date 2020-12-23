extern crate serde_json;
#[macro_use]
extern crate tower_web;

use tower_web::ServiceBuilder;
use tower_web::middleware::cors::{CorsBuilder, AllowedOrigins, CorsMiddleware};
use std::time::Duration;

mod routes;
mod types;

pub fn main() {
    let addr = "127.0.0.1:25580".parse().expect("Port already in use");
    println!("Listening on http://{}", addr);

    let cors = CorsBuilder::new()
        .allow_origins(AllowedOrigins::Any { allow_null: false })
        .max_age(Duration::new(300, 0))
        .build();

    ServiceBuilder::new()
        .resource(routes::token::TokenResource)
        .resource(routes::code::CodeResource)
        .middleware(CorsMiddleware::from(cors))
        .run(&addr)
        .unwrap()
}