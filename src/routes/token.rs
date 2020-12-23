use redis::{Commands, ToRedisArgs, RedisResult};

use crate::types::code::OAuthCode;
use crate::types::status::RequestStatus;
use crate::types::token::*;

#[derive(Clone, Debug)]
pub struct TokenResource;

impl_web! {
    impl TokenResource {
        #[post("/token")]
        #[content_type("application/json")]
        fn get_token(&self, body: OAuthCode) -> Result<TokenResponse, ()> {
            match redis::Client::open("redis://127.0.0.1:6379/") {
                Ok(client) => match client.get_connection() {
                    Ok(mut conn) => {
                        let data: RedisResult<String> = conn.get(body.code.to_redis_args());
                        match data {
                            Ok(user) => {
                                let _ : () = conn.del(&body.code).unwrap();

                                println!("Fetching data for {}", user);

                                // TODO: FETCH FROM DB
                                Ok(TokenResponse {
                                    status: RequestStatus {
                                        message: "Successfully created a new token",
                                        code: 0
                                    },
                                    data: Some(TokenData {
                                        access_token: "4eabde45ff5626562cf8e9dc7c0abf8f.9c8d9442383b05476310e0e#66c66b94",
                                        app: "83ed1",
                                        refresh_token: "492e50f22d0941c54afea8465fe3813f.72f316244ee6fe236925c36#a2a9ffae",
                                        token_type: 1,
                                        expires_in: 604800,
                                    }),
                                })
                            },
                            Err(_) => Ok(TokenResponse {
                                status: RequestStatus {
                                    message: "Could not find the requested code",
                                    code: 400
                                },
                                data: None
                            })
                        }
                    },
                    Err(_) => Ok(TokenResponse {
                        status: RequestStatus {
                            message: "Could not create connection with redis database",
                            code: 500
                        },
                        data: None
                    })
                },
                Err(_) => Ok(TokenResponse {
                    status: RequestStatus {
                        message: "Could not create redis client",
                        code: 500
                    },
                    data: None
                })
            }
        }
    }
}

