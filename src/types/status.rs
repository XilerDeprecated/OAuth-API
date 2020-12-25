#[derive(Serialize, Debug)]
pub struct RequestStatus {
    pub code: u16,
    pub message: &'static str,
}

#[derive(Debug, Response)]
#[web(status = "400")]
pub struct InvalidRequest {
    pub status: RequestStatus,
}