use tokio;

mod output;
mod settings;
mod http;
mod utils;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    output::print_menu();
    
    let config: settings::Settings = settings::conf_settings();
    let code: String = utils::code_gen::generate_code(config);
    let response: String = http::make_request(&code, String::from("127.0.0.1")).await;

    if response == "Unknown Gift Code" {
        output::print_error(&code);
    } else if response == "The resource is being rate limited." {
        output::print_ratelimit(&code);
    } else {
        output::print_success(&code);
    }

    http::fetch_proxies().await;

    Ok(())
}