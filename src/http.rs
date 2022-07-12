use reqwest::{Response, get};
use serde;

#[derive(serde::Deserialize)]
pub struct CodeResponse {
    pub message: String
}

pub struct Proxy {
    pub host: String,
    pub port: String,
}

pub async fn make_request(code: &String, proxy: String) -> String {
    let base_url: String = String::from("https://discordapp.com/api/v6/entitlements/gift-codes/");
    let url: String = base_url + &code;

    let http_proxy = reqwest::Proxy::http(proxy)
    .expect("Failed to construct a proxy");

    let client = reqwest::Client::builder()
        .proxy(http_proxy)
        .build()
        .expect("Failed to build http client");

    let response: Response = client.get(url)
        .send()
        .await
        .expect("Failed to send the request");

    let data: CodeResponse = response.json::<CodeResponse>()
        .await
        .expect("Failed to parse data as a json and deserialize into struct");

    return String::from(data.message);
}

pub async fn fetch_proxies() -> Option<Vec<Proxy>> {
    let mut proxies: Vec<Proxy> = Vec::new();
    let base_url: String = String::from("https://api.proxyscrape.com/v2/?request=displayproxies&protocol=http&timeout=10000&country=all&ssl=all&anonymity=all");

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
            host: String::from(proxy_vec[1]), 
            port: String::from(proxy_vec[0]) 
        };

        proxies.push(new_proxy);
    };

    return Some(proxies);
}