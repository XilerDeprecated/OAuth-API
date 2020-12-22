#[derive(Debug, Extract)]
pub struct OAuthCode {
    pub code: String,
}

#[derive(Debug, Extract)]
pub struct Token {
    pub token: String,
}

#[derive(Serialize, Debug)]
pub struct TokenData {
    pub access_token: &'static str,
    pub app: &'static str,
    pub refresh_token: &'static str,
    pub token_type: i8,
    pub expires_in: u32,
}

#[derive(Debug, Response)]
#[web(status = "201")]
pub struct TokenResponse {
    pub status: super::super::types::status::RequestStatus,
    pub data: TokenData,
}

#[derive(Debug, Response)]
#[web(status = "201")]
pub struct TokenRevokeResponse {
    access_token: &'static str,
    app: &'static str,
    refresh_token: &'static str,
    token_type: i8,
    expires_in: u32,
}