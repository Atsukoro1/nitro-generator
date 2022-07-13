use tokio;

mod output;
mod settings;
mod http;
mod utils;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut proxies: Vec<http::Proxy> = vec![];
    let mut generated: Option<u128> = Some(1);

    output::print_menu();
    let config: settings::Settings = settings::conf_settings();

    while config.code_count >= generated.unwrap() { 
        // Fetch proxies when proxy list is empty
        if proxies.len() == 0 {
            proxies = http::fetch_proxies()
                .await
                .expect("Failed to fetch proxies");
        };

        // Generate code
        let code: String = utils::code_gen::generate_code(&config);

        // Check the code
        let proxy: &http::Proxy = proxies.first().unwrap();

        println!("{}", proxy.host);
        println!("{}", proxy.port);

        let proxy_string: String = format!("http://{}:{}", proxy.host, proxy.port);
        let response: String = http::make_request(&code, proxy_string).await;

        let unknown: String = String::from("Unknown Gift Code");
        let ratelimit: String = String::from("The resource is being rate limited.");
    
        if response.eq("Unknown Gift Code") {
            output::print_error(&code);
        } else if response.eq("The resource is being rate limited.") {
            output::print_ratelimit(&code);
            proxies.drain(0..1);
        } else {
            output::print_success(&code);
        };

        generated = Some(generated.unwrap() + 1);
    };

    Ok(())
}