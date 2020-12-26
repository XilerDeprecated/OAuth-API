use redis::{Commands, RedisResult, ToRedisArgs};
use uuid::Uuid;

use crate::types::code::OAuthCode;
use crate::types::status::RequestStatus;
use crate::types::token::*;
use crate::utils::env::get_env_var;
use crate::utils::request::create_new_user_token;

#[derive(Clone, Debug)]
pub struct TokenResource {
    pub(crate) connection_string: String,
}

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
            match redis::Client::open(&*format!("redis://{}/", get_env_var("REDIS_URL"))) {
                Ok(client) => match client.get_connection() {
                    Ok(mut conn) => {
                        let pass = get_env_var("REDIS_PASS");
                        let _: () = redis::cmd("AUTH").arg(pass).query(&mut conn).unwrap();
                        let data: RedisResult<String> = conn.get(body.code.to_redis_args());
                        match data {
                            Ok(user) => {
                                match Uuid::parse_str(&user) {
                                    Ok(uuid) => {
                                        let _ : () = conn.del(&body.code).unwrap();
                                        create_token_response(
                                            "Successfully created a new token",
                                            0,
                                            Some(create_new_user_token(body.app.clone(), uuid, &self.connection_string))
                                        )

                                    }
                                    Err(_) => create_invalid_token_response("Could not parse uuid", 500)
                                }
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

