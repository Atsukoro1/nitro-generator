extern crate colored;

use colored::*;
use chrono;
pub enum MessageType {
    Success,
    Failure,
    Warning, 
}

pub fn get_message_type(message_type: MessageType) -> ColoredString {
    match message_type {
        MessageType::Failure => String::from("[!]")
            .bright_red(),
        MessageType::Warning => String::from("[!]")
            .yellow(),
        MessageType::Success => String::from("[✔]")
            .green(),
    }
}

fn local_time() -> ColoredString {
    return chrono::Local::now()
    .format("%H:%M:%S")
    .to_string()
    .bright_black()
}

pub fn print_success(code: &String) {
    println!(
        "{} / {} Code {} is valid, feel free to use it!", 
        get_message_type(MessageType::Success),
        local_time(),
        code.green()
    );
}

pub fn print_ratelimit(code: &String) {
    println!(
        "{} / {} Code {} wasn't checked because of ratelimit, switching proxy!",
        get_message_type(MessageType::Warning),
        local_time(),
        code.blue()
    );
}

pub fn print_error(code: &String) {
    println!(
        "{} / {} Code {} is not valid!", 
        get_message_type(MessageType::Failure),
        local_time(),
        code.red()
    );
}

pub fn print_invalid_proxy(code: &String) {
    println!(
        "{} / {} Code {} cannot be verified, proxy is not working, switching!", 
        get_message_type(MessageType::Warning),
        local_time(),
        code.yellow()
    );
}