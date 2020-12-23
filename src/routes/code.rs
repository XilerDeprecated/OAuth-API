use rand::{distributions::Alphanumeric, Rng};
use redis::Commands;

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

            // TODO: MAKE THIS MORE SAFE WHEN REDIS IS DOWN
            let redis_client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
            let mut conn = redis_client.get_connection().unwrap();
            let _ : () = conn.set_ex(&code, &body.user, 300).unwrap();

            Ok(TokenResponse {
                status: RequestStatus {
                    message: "Successfully generated an auth code",
                    code: 201
                },
                data: Some(OAuthCode {
                    code: code
                })
            })
        }
    }
}
