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

#[derive(Serialize, Debug)]
pub struct StatusItem {
    pub response_time: usize,
    pub status: u16
}

#[derive(Debug, Response)]
#[web(status = "200")]
pub struct StatusData {
    pub site: StatusItem,
    pub api: StatusItem,
    pub redis: StatusItem,
    pub postgresql: StatusItem
}
