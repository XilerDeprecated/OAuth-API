use super::super::types::code::OAuthCode;
use super::super::types::token::Token;

#[derive(Clone, Debug)]
pub struct CodeResource;

impl_web! {
    impl CodeResource {
        #[post("/code")]
        #[content_type("application/json")]
        fn get_code(&self, body: Token) -> Result<OAuthCode, ()> {
            // TODO: CREATE IN REDIS DB
            println!("Creating code for {}", &body.token);
            Ok(OAuthCode {
                code: "2547inaqslk4edlwduxydwbuyv8pj7r4o1fdz2cwukm82dd94rnf7lq0c83wouuw".to_string()
            })
        }
    }
}
