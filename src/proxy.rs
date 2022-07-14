use reqwest::{get, Response};

pub enum ProxySource {
    Proxyscrape,
    ApiProxyFree,
    Geonode,
    Local
}

pub struct Proxy {
    pub host: String,
    pub port: String,
}

pub async fn fetch_proxies(source: ProxySource) -> Option<Vec<Proxy>> {
    let mut proxies: Vec<Proxy> = Vec::new();
    
    match source {
        ProxySource::Proxyscrape => {
            return Some(fetch_proxyscrape().await);
        }

        _ => {
            return None;
        }
    };
}

async fn fetch_proxyscrape() -> Vec<Proxy> {
    let mut proxies: Vec<Proxy> = vec![]; 
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

    return proxies;
}