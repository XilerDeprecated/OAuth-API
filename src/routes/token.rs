use super::super::types::token::*;
use super::super::types::status::RequestStatus;

#[derive(Clone, Debug)]
pub struct TokenResource;

impl_web! {
    impl TokenResource {
        #[post("/token")]
        #[content_type("application/json")]
        fn get_token(&self, body: OAuthCode) -> Result<TokenResponse, ()> {
            // TODO: FETCH FROM DB
            println!("Provided code: {}", body.code);
            Ok(TokenResponse {
                status: RequestStatus {
                    message: "Successfully created a new token",
                    code: 0
                },
                data: TokenData {
                    access_token: "4eabde45ff5626562cf8e9dc7c0abf8f.9c8d9442383b05476310e0e#66c66b94",
                    app: "83ed1",
                    refresh_token: "492e50f22d0941c54afea8465fe3813f.72f316244ee6fe236925c36#a2a9ffae",
                    token_type: 1,
                    expires_in: 604800,
                },
            })
        }

        #[post("/token/revoke")]
        #[content_type("application/json")]
        fn revoke_token(&self, body: Token) -> Result<TokenResponse, ()> {
            // TODO: FETCH FROM DB & GENERATE NEW TOKEN
            println!("Revoking token: {}", body.token);
            Ok(TokenResponse {
                status: RequestStatus {
                    message: "Successfully created a new token",
                    code: 0
                },
                data: TokenData {
                    access_token: "4eabde45ff5626562cf8e9dc7c0abf8f.9c8d9442383b05476310e0e#66c66b94",
                    app: "83ed1",
                    refresh_token: "492e50f22d0941c54afea8465fe3813f.72f316244ee6fe236925c36#a2a9ffae",
                    token_type: 1,
                    expires_in: 604800,
                },
            })
        }
    }
}

