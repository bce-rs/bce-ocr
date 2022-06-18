use serde::Deserialize;
use thiserror::Error;

use std::{
    time::{Duration, SystemTime},
    error::Error as StdError,
};

/// returns (access_token, expires_date)
pub fn acquire_access_token(
    api_key: &str,
    secret_key: &str,
) -> Result<(String, SystemTime), Box<dyn StdError>> {
    let before_acquire = SystemTime::now();
    let result = ureq::post("https://aip.baidubce.com/oauth/2.0/token")
        .set("Content-Type", "application/x-www-form-urlencoded")
        .query("grant_type", "client_credentials")
        .query("client_id", api_key)
        .query("client_secret", secret_key)
        .call()?
        .into_string()?;

    let token: Result<Token, serde_json::Error> = serde_json::from_str(&result);

    if let Ok(token) = token {
        let access_token = token.access_token;
        let expires_time = before_acquire + Duration::from_secs(token.expires_in);

        Ok((access_token, expires_time))
    } else {
        let error: VerifyError = serde_json::from_str(&result).unwrap();
        Result::Err(error.into())
    }
}

#[derive(Deserialize)]
struct Token {
    /// Access Token is the only identity key when calling api.
    access_token: String,
    /// time in second when the access_token expires. now it will be 1 month
    expires_in: u64,
}

#[derive(Error, Deserialize, Debug)]
enum VerifyError {
    #[error("{0}")]
    error(String),
    #[error("{0}")]
    error_description(String),
}
