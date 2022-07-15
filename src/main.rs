use tokio;

mod output;
mod settings;
mod proxy;
mod code;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut proxies: Vec<proxy::Proxy> = vec![];
    let mut generated: Option<u128> = Some(1);

    output::print_menu();
    let config: settings::Settings = settings::conf_settings();

    while config.code_count >= generated.unwrap() { 
        // Fetch proxies when proxy list is empty
        if proxies.len() == 0 {
            proxies = proxy::fetch_proxies(&config.proxy_souce)
                .await
                .expect("Failed to fetch proxies");
        };

        // Generate code
        let code: String = code::generate_code(&config);

        // Check the code
        let proxy: &proxy::Proxy = proxies.first().unwrap();

        println!("{}", proxy.host);
        println!("{}", proxy.port);

        let proxy_string: String = format!("{}:{}", proxy.host, proxy.port);
        let response: reqwest::StatusCode = code::check_code(&code, &proxy_string).await;

        match response {
            reqwest::StatusCode::OK => {
                output::print_success(&code);
            },

            reqwest::StatusCode::NOT_FOUND => {
                output::print_error(&code);
            }

            reqwest::StatusCode::TOO_MANY_REQUESTS => {
                output::print_ratelimit(&code);
                proxies.drain(0..1);
            }

            _ => {
                output::print_invalid_proxy(&code);
                proxies.drain(0..1);
            }
        }

        generated = Some(generated.unwrap() + 1);
    };

    Ok(())
}