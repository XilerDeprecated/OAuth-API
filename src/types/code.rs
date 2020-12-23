#[derive(Debug, Extract, Serialize)]
pub struct OAuthCode {
    pub code: String,
}

#[derive(Debug, Response)]
#[web(status = "201")]
pub struct TokenResponse {
    pub status: super::super::types::status::RequestStatus,
    pub data: OAuthCode,
}