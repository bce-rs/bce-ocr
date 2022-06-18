#![allow(unused_variables)]

use bce_ocr::token;
const SECRET: &str = include_str!("res/key.csv");

#[test]
fn acquire_access_token() {
    access_token();
}

pub fn access_token() -> String {
    let secret = SECRET.to_owned();
    let keys: Vec<&str> = secret.trim().split(",").collect();
    let api_key = keys[0];
    let secret_key = keys[1];

    let (access_token, _) = token::acquire_access_token(api_key, secret_key).unwrap();
    access_token
}
