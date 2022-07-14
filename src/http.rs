use reqwest::{Response, Error, get};
use serde;
use std::time::Duration;

#[derive(serde::Deserialize)]
pub struct CodeResponse {
    pub message: String
}

#[allow(dead_code)]
pub struct Proxy {
    pub host: String,
    pub port: String,
}

pub async fn make_request(code: &String, proxy: String) -> reqwest::StatusCode {
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
    // let data: CodeResponse = response.json::<CodeResponse>()
    //     .await
    //     .expect("Failed to parse data as a json and deserialize into struct");

    // println!("{}", data.message);
    // return data.message;
}

pub async fn fetch_proxies() -> Option<Vec<Proxy>> {
    let mut proxies: Vec<Proxy> = Vec::new();
    let base_url: String = String::from("https://api.proxyscrape.com/?request=displayproxies&proxytype=http&timeout=1500&ssl=yes");

    let response: String = get(base_url)
        .await
        .expect("Failed to return response")
        .text()
        .await
        .expect("Failed to parse response as text")
        .to_string();

    for item in response.split("\n") {
        if item.len() == 0 {
            continue;
        };

        let proxy_vec: Vec<&str> = item.split(":").collect::<Vec<&str>>();
        let new_proxy: Proxy = Proxy { 
            host: String::from(proxy_vec[0]), 
            port: String::from(proxy_vec[1]) 
        };

        proxies.push(new_proxy);
    };

    return Some(proxies);
}