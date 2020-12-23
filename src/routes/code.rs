use std::env;
use rand::{distributions::Alphanumeric, Rng};
use redis::{Commands, RedisResult};

use crate::types::code::{OAuthCode, TokenResponse};
use crate::types::status::RequestStatus;
use crate::types::token::CodeToken;
use crate::utils::request::is_valid_code_request;

#[derive(Clone, Debug)]
pub struct CodeResource;

impl_web! {
    impl CodeResource {
        #[post("/code")]
        #[content_type("application/json")]
        fn get_code(&self, body: CodeToken) -> Result<TokenResponse, ()> {
            if !is_valid_code_request(&body) {
                return Ok(TokenResponse {
                    status: RequestStatus {
                        message: "Invalid request parameters values",
                        code: 400
                    },
                    data: None
                });
            }

            let c: String = rand::thread_rng().sample_iter(&Alphanumeric).take(64).map(char::from).collect();
            let code: String = (&c).to_lowercase();

            match redis::Client::open("redis://127.0.0.1:6379/") {
                Ok(client) => match client.get_connection() {
                    Ok(mut conn) => {
                        let pass = match env::var("REDIS_PASS") {
                            Ok(val) => val,
                            Err(_) => "".to_string(),
                        };
                        let _: () = redis::cmd("AUTH").arg(pass).query(&mut conn).unwrap();
                        let set: RedisResult<String> = conn.set_ex(&code, &body.user, 300);
                        match set {
                            Ok(_) => Ok(TokenResponse {
                                status: RequestStatus {
                                    message: "Successfully generated an auth code",
                                    code: 201
                                },
                                data: Some(OAuthCode {
                                    code: code
                                })
                            }),
                            Err(_) => Ok(TokenResponse {
                                status: RequestStatus {
                                    message: "Could not create code",
                                    code: 500
                                },
                                data: None
                            })
                        }
                    }
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
