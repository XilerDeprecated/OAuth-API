use super::super::types::code::{OAuthCode, TokenResponse};
use super::super::types::token::Token;
use super::super::types::status::{RequestStatus};

#[derive(Clone, Debug)]
pub struct CodeResource;

impl_web! {
    impl CodeResource {
        #[post("/code")]
        #[content_type("application/json")]
        fn get_code(&self, body: Token) -> Result<TokenResponse, ()> {
            // TODO: CREATE IN REDIS DB
            println!("Creating code for {}", &body.token);
            Ok(TokenResponse {
                status: RequestStatus {
                    message: "Successfully generated an auth code",
                    code: 0
                },
                data: OAuthCode {
                    code: "2547inaqslk4edlwduxydwbuyv8pj7r4o1fdz2cwukm82dd94rnf7lq0c83wouuw".to_string()
                }
            })
        }
    }
}
