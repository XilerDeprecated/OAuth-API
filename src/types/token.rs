#[derive(Debug, Extract)]
pub struct Token {
    pub token: String,
}

#[derive(Debug, Extract)]
pub struct CodeToken {
    pub token: String,
    pub app: String,
    pub user: String,
}

#[derive(Serialize, Debug)]
pub struct TokenData {
    pub access_token: String,
    pub app: String,
    pub refresh_token: String,
    pub token_type: i16,
    pub expires_in: Option<u64>,
}

#[derive(Debug, Response)]
#[web(status = "200")]
pub struct TokenResponse {
    pub status: super::super::types::status::RequestStatus,
    pub data: Option<TokenData>,
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