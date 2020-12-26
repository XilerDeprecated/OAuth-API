use redis::{Commands, RedisResult, ToRedisArgs};

use crate::types::code::OAuthCode;
use crate::types::status::RequestStatus;
use crate::types::token::*;
use crate::utils::env::get_env_var;

#[derive(Clone, Debug)]
pub struct TokenResource;

fn create_token_response(message: &'static str, code: u16, data: Option<TokenData>) -> Result<TokenResponse, ()> {
    Ok(TokenResponse {
        status: RequestStatus { message, code },
        data,
    })
}

fn create_invalid_token_response(message: &'static str, code: u16) -> Result<TokenResponse, ()> {
    create_token_response(message, code, None)
}


impl_web! {
    impl TokenResource {
        #[post("/token")]
        #[content_type("application/json")]
        fn get_token(&self, body: OAuthCode) -> Result<TokenResponse, ()> {
            match redis::Client::open("redis://127.0.0.1:6379/") {
                Ok(client) => match client.get_connection() {
                    Ok(mut conn) => {
                        let pass = get_env_var("REDIS_PASS");
                        let _: () = redis::cmd("AUTH").arg(pass).query(&mut conn).unwrap();
                        let data: RedisResult<String> = conn.get(body.code.to_redis_args());
                        match data {
                            Ok(user) => {
                                let _ : () = conn.del(&body.code).unwrap();
                                // TODO: FETCH FROM DB
                                create_token_response(
                                    "Successfully created a new token",
                                    0,
                                    Some(TokenData {
                                        access_token: "4eabde45ff5626562cf8e9dc7c0abf8f.9c8d9442383b05476310e0e#66c66b94",
                                        app: "83ed1",
                                        refresh_token: "492e50f22d0941c54afea8465fe3813f.72f316244ee6fe236925c36#a2a9ffae",
                                        token_type: 1,
                                        expires_in: 604800,
                                    })
                                )
                            },
                            Err(_) => create_invalid_token_response("Could not find the requested code", 400)
                        }
                    },
                    Err(_) => create_invalid_token_response("Could not create connection with redis database", 500)
                },
                Err(_) => create_invalid_token_response("Could not create redis client", 500)
            }
        }
    }
}

