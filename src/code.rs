use crate::settings::{Settings};
use rand::{Rng};
use rand::distributions::Alphanumeric;
use serde::Deserialize;
use reqwest::{Response, Error};
use std::time::Duration;


pub fn generate_code(config: &Settings) -> String {
    let mut final_string: String = String::from("");
    let code_length: usize;
    
    match config.boost {
        true => code_length = 24 as usize,
        false => code_length = 16 as usize,
    };
    
    return rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(code_length)
        .map(char::from)
        .collect();
}

#[derive(Deserialize)]
pub struct CodeResponse {
    pub message: String
}

pub async fn check_code(code: &String, proxy: String) -> reqwest::StatusCode {
    let base_url: String = String::from("https://discordapp.com/api/v6/entitlements/gift-codes/");
    let url: String = base_url + &code;

    let http_proxy = reqwest::Proxy::all(proxy)
    .expect("Failed to construct a proxy");

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; rv:102.0) Gecko/20100101 Firefox/102.0")
        .proxy(http_proxy)
        .timeout(Duration::from_secs(5))
        .build()
        .expect("Failed to build http client");

    let response: Result<Response, Error> = client.get(url)
        .send()
        .await;

    let to_return: i32 = match response {
        Ok(..) => {
            return response.unwrap().status();
        },

        Err(..) => {
            return reqwest::StatusCode::REQUEST_TIMEOUT;
        }
    };
}