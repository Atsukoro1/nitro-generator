use reqwest::{Response, get};
use serde;

#[derive(serde::Deserialize)]
pub struct CodeResponse {
    pub message: String
}

pub struct Proxy {
    pub host: String,
    pub port: u32
}

pub async fn make_request(code: &String, _proxy: &String) -> String {
    let base_url: String = String::from("https://discordapp.com/api/v6/entitlements/gift-codes/");
    let url: String = base_url + &code;

    let response: Response = get(url).await.unwrap();
    
    let data: CodeResponse = response.json::<CodeResponse>()
        .await
        .unwrap();

    return String::from(data.message);
}

pub async fn fetch_proxies() -> Vec<Proxy> {
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
        let proxyVec: Vec<&str> = item.split(":").collect::<Vec<&str>>();
        let mut newProx: Proxy = Proxy { host: String::from(""), port: 1 };

        newProx.host = proxyVec[0].to_string();
        newProx.port = proxyVec[1].to_string()
            .parse::<u32>()
            .unwrap();
        
        proxies.push(newProx);
    }

    return proxies;
}