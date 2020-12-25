use postgres::{Client, NoTls};
use uuid::Uuid;

use crate::types::token::CodeToken;

fn handle_code_request_errors(message: &str) -> bool {
    println!("{}", message);
    false
}

pub fn is_valid_code_request(req: &CodeToken, connection_string: &str) -> bool {
    match Client::connect(&connection_string, NoTls) {
        Ok(mut client) => {
            match Uuid::parse_str(&req.user) {
                Ok(uuid) => match client.query("SELECT * \
                                      FROM tokens \
                                      WHERE definite = $1 \
                                        AND account = $2 \
                                        AND app = $3;",
                                               &[&req.token, &uuid, &req.app]) {
                    Ok(data) => data.len() >= 1,
                    Err(e) => handle_code_request_errors(&format!("Couldn't execute query. {}", e))
                },
                Err(e) => handle_code_request_errors(&format!("Couldn't parse UUID. {}", e))
            }
        }
        Err(e) => handle_code_request_errors(&format!("Couldn't connect to DB. {}", e))
    }
}