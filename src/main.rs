extern crate redis;
extern crate serde_json;
#[macro_use]
extern crate tower_web;

use tower_web::ServiceBuilder;

mod routes;
mod types;
mod utils;

#[derive(Debug, Clone)]
struct Root;

impl_web! {
    impl Root {
        #[post("/")]
        fn root(&self) -> Result<&'static str, ()> {
            Ok("Working properly")
        }
    }
}

pub fn main() {
    let addr = "127.0.0.1:25580".parse().expect("Port already in use");
    println!("Listening on http://{}", addr);

    let postgres_user = "arthur";
    let postgres_pass = "";
    let postgres_url = "no-proxy.xiler.net";
    let postgres_database = "xiler";
    let postgres_connection_string = format!("postgresql://{}:{}@{}:5432/{}", postgres_user, postgres_pass, postgres_url, postgres_database);

    ServiceBuilder::new()
        .resource(Root)
        .resource(routes::token::TokenResource)
        .resource(routes::code::CodeResource {
            connection_string: postgres_connection_string
        })
        .run(&addr)
        .unwrap()
}
