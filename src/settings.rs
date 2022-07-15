extern crate colored;

use std::{
    io::{self, Write, IoSlice, Read}, 
    i8, 
    str::FromStr
};
use colored::*;
use crate::{output::{
    get_message_type,
    MessageType
}, proxy::{ProxySource}};

pub struct Settings {
    pub boost: bool,
    pub code_count: u128,
    pub thread_count: u32,
    pub proxy_souce: ProxySource,
}

fn input<T: FromStr>(message: &str, default: T) -> T {
    println!("\n{} {}", get_message_type(MessageType::Info), message);

    let mut temp: String = String::new();
    std::io::stdin()
        .read_line(&mut temp)
        .unwrap();

    return temp.trim()
        .parse::<T>()
        .unwrap_or(default);
}

pub fn conf_settings() -> Settings {
    let mut config: Settings = Settings{
        boost: false,
        code_count: 0,
        thread_count: 0,
        proxy_souce: ProxySource::Proxyscrape
    };

    config.boost = input(
        "What type of code do you want to generate [boost = true/clasic = false]: ", 
        false,
    );

    config.code_count = input(
        "How many codes do you want to generate [number]: ",
        100
    );

    config.thread_count = input(
        "How many threads do you want to use [number]: ",
        10
    );

    config.proxy_souce = match input(
        "Select proxy source:\n[1] Proxyscrape\n[2] Local\n[3] Geonode", 
        1
    ) {
        1 => ProxySource::Proxyscrape,
        2 => ProxySource::Local,
        3 => ProxySource::Geonode,
        _ => ProxySource::Proxyscrape,
    };

    return config;
} 