use std::env;

pub(crate) fn get_env_var(key: &str) -> String {
    env::var(key).expect(&format!("Invalid .env file, missing key '{}'!", key))
}