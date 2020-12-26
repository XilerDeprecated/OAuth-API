use postgres::{Client, NoTls};
use uuid::Uuid;

use crate::types::token::CodeToken;
use crate::types::token::TokenData;
use std::time::SystemTime;

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

pub fn create_new_user_token(app: String, uuid: Uuid, connection_string: &str) -> TokenData {
    match Client::connect(&connection_string, NoTls) {
        Ok(mut client) => match client.query(
            "SELECT * \
            FROM get_or_create_user_app_token(\
                CAST($1 AS varchar(15)), \
                CAST($2 AS uuid), \
                CAST(1 AS smallint));",
            &[&app, &uuid]) {

            Ok(data) => {
                let mut fetched: Option<TokenData> = None;
                for row in data {
                    let access_token: String = row.get(0);
                    let app: String = row.get(1);
                    let refresh_token: String = row.get(2);
                    let token_type: i16 = row.get(3);
                    let expiry_date: Option<SystemTime> = row.get(4);

                    fetched = Some(TokenData { access_token, app, refresh_token, token_type,
                        expires_in: match expiry_date {
                            Some(date) => Some(match date.duration_since(SystemTime::now()) {
                                Ok(n) => n.as_secs(),
                                Err(_) => 0,
                            }),
                            _ => None
                        },
                    })
                }

                if let Some(data) = fetched {
                    data
                } else {
                    panic!("No data was fetched!")
                }
            },
            Err(e) => panic!("Couldn't execute query. {}", e)
        },
        Err(e) => panic!("Couldn't connect to DB. {}", e)
    }
}
