use std::{io};
use rand::{Rng};
use reqwest::{get, Response};
use serde;
use tokio;

mod output;

struct Settings {
    boost: bool,
    code_count: u128
}

fn input() -> Settings {
    let mut config: Settings = Settings{
        boost: false,
        code_count: 0
    };

    let mut icode_type: String = String::from("");
    let mut icode_count: String = String::from("");

    println!("What type of codes to generate [boost/classic]: ");
    io::stdin()
        .read_line(&mut icode_type)
        .expect("Failed to read this line!");

    println!("How many codes do you want to generate [number]: ");
    io::stdin()
        .read_line(&mut icode_count)
        .expect("Cannot read this line!");

    config.code_count = icode_count
        .trim()
        .parse()
        .expect("Please enter a valid number!");

    if icode_type == "boost" {
        config.boost = true;
    } else {
        config.boost = false;
    }

    return config;
}

fn generate_code(config: Settings) -> String {
    const CHARSET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
    let mut final_string: String = String::from("");
    let code_length: u8;
    
    if config.boost {
        code_length = 24;
    } else {
        code_length = 16;
    }

    let mut rng = rand::thread_rng();
    for _x in 0..code_length {
        let number: usize = rng.gen_range(0, CHARSET.len() as usize);
        final_string.push_str(&CHARSET[number - 1..number]);
    }

    println!("{}", final_string);
    return final_string;
}

#[derive(serde::Deserialize)]
struct CodeResponse {
    message: String
}

// Unknown Gift Code
// The resource is being rate limited.
async fn check_code(code: &String) -> String {
    let base_url: String = String::from("https://discordapp.com/api/v6/entitlements/gift-codes/");
    let url: String = base_url + &code;

    let response: Response = get(url).await.unwrap();
    
    let data: CodeResponse = response.json::<CodeResponse>()
    .await.unwrap();

    return data.message;
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config: Settings = input();
    let code: String = generate_code(config);
    let response: String = check_code(&code).await;

    if response == "Unknown Gift Code" {
        output::print_success(code);
    } else {
        output::print_success(code);
    }

    Ok(())
}