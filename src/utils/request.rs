use crate::types::token::CodeToken;

pub fn is_valid_code_request(req: &CodeToken) -> bool {
    // TODO: CHECK DATABASE FOR THIS
    &req.user == "0257c994-54c1-4c5b-9ad6-2b95a5a93ffb" &&
        &req.app == "83ed1" &&
        &req.token == "492e50f22d0941c54afea8465fe3813f.72f316244ee6fe236925c36#a2a9ffae"
}