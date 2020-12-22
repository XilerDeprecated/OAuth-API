#[derive(Serialize, Debug)]
pub struct RequestStatus {
    pub code: u16,
    pub message: &'static str,
}
