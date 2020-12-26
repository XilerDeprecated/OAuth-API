#[derive(Debug, Extract, Serialize)]
pub struct OAuthCode {
    pub code: String,
}

#[derive(Debug, Response)]
#[web(status = "200")]
pub struct TokenResponse {
    pub status: crate::types::status::RequestStatus,
    pub data: Option<OAuthCode>,
}