use reqwest::{get};
use core::panic;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path};
use std::vec;
use crate::output::{MessageType, get_message_type};
use std::thread::sleep;
use std::time::Duration;
use serde::Deserialize;

pub enum ProxySource {
    Proxyscrape,
    Geonode,
    Local
}

pub struct Proxy {
    pub host: String,
    pub port: String,
}

#[derive(Deserialize)]
struct GeonodeResponseField {
    ip: String,
    port: String
}

#[derive(Deserialize)]
struct GeonodeResponse {
    data: Vec<GeonodeResponseField>,
}

pub async fn fetch_proxies(source: &ProxySource) -> Option<Vec<Proxy>> {
    match source {
        ProxySource::Proxyscrape => {
            return Some(fetch_proxyscrape().await);
        },

        ProxySource::Local => {
            return Some(fetch_local().await);
        },

        ProxySource::Geonode => {
            return Some(fetch_geonode().await);
        }
    };
}


async fn fetch_geonode() -> Vec<Proxy> {
    let mut proxies: Vec<Proxy> = vec![];
    let base_url: String = String::from("https://proxylist.geonode.com/api/proxy-list?limit=2&page=100&sort_by=speed&sort_type=desc");

    let response: GeonodeResponse = get(base_url)
        .await
        .expect("Failed to return response")
        .json::<GeonodeResponse>()
        .await
        .expect("Failed parsing response as a JSON");   

    for item in response.data {
        proxies.push(
            Proxy {
                host: item.ip,
                port: item.port
            }
        );
    };
    
    return proxies;
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

        proxies.push(Proxy { 
            host: String::from(proxy_vec[0]), 
            port: String::from(proxy_vec[1]) 
        });
    };

    return proxies;
}

async fn fetch_local() -> Vec<Proxy> {
    let mut proxies: Vec<Proxy> = vec![];
    let path: &Path = Path::new("proxies.txt");

    let mut file = match File::open(&path) {
        Err(..) => {
            println!(
                "{} / Failed to get proxy.txt file, please create one!", 
                get_message_type(MessageType::Failure)
            );

            sleep(Duration::from_secs(3));
            panic!("Failed to get proxy file!");
        },

        Ok(file) => {
            file
        },
    };

    let mut prox_plain: String = String::new();
    match file.read_to_string(&mut prox_plain) {
        Err(..) => {
            println!(
                "{} / Can't open proxies.txt file, try running this executable with elevated permissions",
                get_message_type(MessageType::Failure)
            );

            panic!("Failed opening proxies.txt file");
        },

        Ok(..) => {
            let prox_iter = prox_plain.split("\n")
                .into_iter();

            for prox in prox_iter {
                let proxy_vec: Vec<&str> = prox.split(":").collect();

                proxies.push(Proxy 
                    { 
                        host: String::from(proxy_vec[0]), 
                        port: String::from(proxy_vec[1])
                    }
                );
            };

            return proxies;
        }
    };
}