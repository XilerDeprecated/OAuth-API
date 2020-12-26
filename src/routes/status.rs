use crate::types::token::Token;
use crate::types::status::*;
use crate::utils::env::get_env_var;
use tower_web::codegen::futures::future::Either;
use tower_web::codegen::futures::future::Either::{A, B};

#[derive(Clone, Debug)]
pub struct StatusResource {
    pub(crate) connection_string: String,
}

impl_web! {
    impl StatusResource {
        #[get("/status")]
        #[content_type("application/json")]
        fn get_status(&self, body: Token) -> Result<Either<StatusData, InvalidRequest>, ()> {
            if body.token == get_env_var("STATUS_TOKEN") {
                Ok(A(StatusData {
                    site: StatusItem {
                        response_time: 0,
                        status: 200
                    },
                    api: StatusItem {
                        response_time: 0,
                        status: 200
                    },
                    redis: StatusItem {
                        response_time: 0,
                        status: 200
                    },
                    postgresql: StatusItem {
                        response_time: 0,
                        status: 200
                    },
                }))
            } else {
                Ok(B(InvalidRequest {
                    status: RequestStatus {
                        message: "An invalid token was provided",
                        code: 400
                    }
                }))
            }
        }
    }
}