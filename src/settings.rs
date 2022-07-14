extern crate colored;

use std::{io, i8};
use colored::*;
use crate::output::{
    get_message_type,
    MessageType
};

pub struct Settings {
    pub boost: bool,
    pub code_count: u128,
    pub thread_count: u32,
    pub proxy_souce: Option<i32>,
}

pub fn conf_settings() -> Settings {
    let mut config: Settings = Settings{
        boost: false,
        code_count: 0,
        thread_count: 0,
        proxy_souce: None
    };

    let mut icode_type: String = String::new();
    let mut icode_count: String = String::new();
    let mut ithread_count: String = String::new();

    let code_gen_string: ColoredString = String::from(
        "What type of code to generate [boost/classic]: "
    ).bright_blue();
    let code_count_string: ColoredString = String::from(
        "How many codes do you want to generate [number]: "
    ).bright_blue();
    let thread_count_string: ColoredString = String::from(
        "How many threads do you want to use [number]"
    ).bright_blue();
    let proxy_source_string: ColoredString = String::from(
        "Select proxy source:\n[1] Proxyscrape\n[2] Local\n[3] Geonode"
    ).bright_blue();

    while icode_type.len() == 0 && (icode_type != "boost" || icode_type != "classic") {
        println!("{} / {}", get_message_type(MessageType::Info), code_gen_string);
        io::stdin()
            .read_line(&mut icode_type)
            .expect("Failed to read this line!");  
    };

    while config.proxy_souce == None {
        println!("{}", proxy_source_string);
        let mut temp: String = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read this line");

        config.proxy_souce = Some(temp.trim().parse::<i32>()
            .expect("Failed to parse proxy source")
        );
    };

    while ithread_count.len() == 0 {
        println!("{} / {}", get_message_type(MessageType::Info), thread_count_string);
        io::stdin()
            .read_line(&mut ithread_count)
            .expect("Failed to read this line!");
    };

    while icode_count == "" {
        println!("{} / {}", get_message_type(MessageType::Info), code_count_string);
        io::stdin()
            .read_line(&mut icode_count)
            .expect("Cannot read this line!");

        for character in icode_count.trim().chars() {
            if !character.is_numeric() {
                icode_count = String::from("");
                break;
            };
        };
    };

    config.code_count = icode_count
        .trim()
        .parse()
        .expect("Please enter a valid number!");

    match icode_type == "boost" {
        false => config.boost = false,
        true => config.boost = true
    }

    return config;
}