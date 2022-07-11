use reqwest::{Response, get};
use serde;

#[derive(serde::Deserialize)]
struct CodeResponse {
    message: String
}

pub async fn make_request(code: &String, _proxy: &String) -> String {
    let base_url: String = String::from("https://discordapp.com/api/v6/entitlements/gift-codes/");
    let url: String = base_url + &code;

    let response: Response = get(url).await.unwrap();
    
    let data: CodeResponse = response.json::<CodeResponse>()
    .await.unwrap();

    return String::from(data.message);
}