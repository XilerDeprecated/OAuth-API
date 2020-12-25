use rand::{distributions::Alphanumeric, Rng};
use redis::{Commands, RedisResult};

use crate::types::code::{OAuthCode, TokenResponse};
use crate::types::status::RequestStatus;
use crate::types::token::CodeToken;
use crate::utils::request::is_valid_code_request;
use crate::utils::env::get_env_var;

#[derive(Clone, Debug)]
pub struct CodeResource {
    pub(crate) connection_string: String,
}

fn create_token_response(message: &'static str, code: u16, data: Option<OAuthCode>) -> Result<TokenResponse, ()> {
    Ok(TokenResponse {
        status: RequestStatus { message, code },
        data,
    })
}

fn return_server_error(message: &'static str) -> Result<TokenResponse, ()> {
    create_token_response(message, 500, None)
}

impl_web! {
    impl CodeResource {
        #[post("/code")]
        #[content_type("application/json")]
        fn get_code(&self, body: CodeToken) -> Result<TokenResponse, ()> {
            if !is_valid_code_request(&body, &self.connection_string) {
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

            // TODO: GET URL FROM CONFIG
            // match redis::Client::open(&format!("redis://{}/", get_env_var("REDIS_URL"))) {
            match redis::Client::open("redis://127.0.0.1:6379/") {
                Ok(client) => match client.get_connection() {
                    Ok(mut conn) => {
                        let pass = get_env_var("REDIS_PASS");
                        let _: () = redis::cmd("AUTH").arg(pass).query(&mut conn).unwrap();
                        let set: RedisResult<String> = conn.set_ex(&code, &body.user, 300);
                        match set {
                            Ok(_) => create_token_response("Successfully generated an auth code", 201, Some(OAuthCode { code })),
                            Err(_) => return_server_error("Could not save code, please report this!")
                        }
                    }
                    Err(_) => return_server_error("Could not create connection with redis database")
                },
                Err(_) => return_server_error("Could not create redis client")
            }
        }
    }
}
