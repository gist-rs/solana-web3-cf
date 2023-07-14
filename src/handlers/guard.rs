use cookie::Cookie;
use serde::{Deserialize, Serialize};
use std::str::{self, FromStr};
use thiserror::Error;
use worker::{console_log, Headers, Request};

trait Maybe {
    fn get_some(&self, key: &str) -> Option<String>;
}

impl Maybe for Headers {
    fn get_some(&self, key: &str) -> Option<String> {
        self.get(key).ok().unwrap_or(None)
    }
}

pub const PHANTOM_SESSION_KEY_NAME: &str = "phantom::session";

#[derive(Serialize, Deserialize, Debug)]
pub struct Web3Token {
    pub wallet_address: String,
    pub phantom: PhantomMobile,
}

#[derive(Debug, Error)]
#[error("expected valid phantom, got \"{0}\"")]
pub struct InvalidDataError(pub String);

impl FromStr for Web3Token {
    fn from_str(cookie_str: &str) -> Result<Self, Self::Err> {
        let pubkey_session = cookie_str.split('|').collect::<Vec<_>>();
        let user_pubkey = pubkey_session[0].to_owned();
        let session = pubkey_session[1].to_owned();
        let data_string = pubkey_session[2].to_owned();
        let data = serde_json::from_str(data_string.as_str())
            .map_err(|_| InvalidDataError(data_string))?;

        Ok(Web3Token {
            wallet_address: user_pubkey,
            phantom: PhantomMobile { session, data },
        })
    }
    type Err = InvalidDataError;
}

#[derive(Serialize, Deserialize, Debug)]

pub struct PhantomMobile {
    pub session: String,
    pub data: PhantomConnectionData,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PhantomConnectionData {
    pub app_url: String,
    pub timestamp: i64,
    pub chain: String,
    pub cluster: String,
}

pub fn extract_web3_token(req: &Request) -> Result<Web3Token, InvalidDataError> {
    let cookie_string = req
        .headers()
        .get_some("Cookie")
        .expect("ERROR: expect cookie.");

    console_log!("cookie_string: {cookie_string:?}");

    let mut cookies = cookie_string
        .split('|')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .flat_map(Cookie::parse_encoded);

    let cookie = cookies
        .find(|e| e.name() == PHANTOM_SESSION_KEY_NAME)
        .expect("ERROR: expect session.");

    let cookie_str = cookie.value();
    Web3Token::from_str(cookie_str)
}
