use rstest::*;

use bce_ocr::acquire_access_token;
const SECRET: &str = include_str!("res/key.csv");

#[fixture]
pub fn access_token() -> String {
    let secret = SECRET.to_owned();
    let keys: Vec<&str> = secret.trim().split(",").collect();
    let api_key = keys[0];
    let secret_key = keys[1];

    let (access_token, _) = acquire_access_token(api_key, secret_key).unwrap();
    access_token
}
