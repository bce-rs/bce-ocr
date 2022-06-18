use bce_ocr::token::acquire_access_token;

const SECRET: &[u8] = include_bytes!("key.csv");

#[test]
fn test_acquire_access_token() -> String{
    let secret = String::from_utf8(SECRET.to_owned()).unwrap();
    let keys: Vec<&str> = secret.trim().split(",").collect();
    let api_key = keys[0];
    let secret_key = keys[1];

    let (access_token, expires_date) = acquire_access_token(api_key, secret_key).unwrap();
    access_token
}
