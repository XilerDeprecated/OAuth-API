#[derive(Debug, Extract, Serialize)]
pub struct OAuthCode {
    pub code: String,
}

#[derive(Debug, Response)]
#[web(status = "201")]
#[web(header(name = "access-control-allow-methods", value = "GET, POST"))]
#[web(header(name = "access-control-allow-headers", value = "authorization, cache-control, if-match, if-modified-since, if-none-match, if-unmodified-since, range"))]
#[web(header(name = "access-control-expose-headers", value = "content-length, content-type"))]
#[web(header(name = "access-control-allow-credentials", value = "true"))]
pub struct TokenResponse {
    pub status: super::super::types::status::RequestStatus,
    pub data: OAuthCode,
}