#[derive(Debug, Extract, Response)]
pub struct OAuthCode {
    pub code: String,
}