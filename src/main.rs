extern crate redis;
extern crate serde_json;
#[macro_use]
extern crate tower_web;

use dotenv::dotenv;
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

    dotenv().expect(".env file not found");
    let postgres_connection_string = format!("postgresql://{}:{}@{}:5432/{}",
                                             utils::env::get_env_var("POSTGRES_USER"),
                                             utils::env::get_env_var("POSTGRES_PASS"),
                                             utils::env::get_env_var("POSTGRES_URL"),
                                             utils::env::get_env_var("POSTGRES_DATABASE"));

    ServiceBuilder::new()
        .resource(Root)
        .resource(routes::token::TokenResource {
            connection_string: postgres_connection_string.clone()
        })
        .resource(routes::code::CodeResource {
            connection_string: postgres_connection_string.clone()
        })
        .resource(routes::status::StatusResource {
            connection_string: postgres_connection_string.clone()
        })
        .run(&addr)
        .unwrap()
}
