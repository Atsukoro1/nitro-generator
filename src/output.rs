extern crate colored;

use colored::*;
use chrono;

fn local_time() -> ColoredString {
    return chrono::Local::now()
    .format("%H:%M:%S")
    .to_string()
    .bright_black()
}

pub fn print_success(code: String) {
    println!(
        "[{}] Code {} is valid, feel free to use it!", 
        local_time(),
        code.green()
    );
}

// pub fn print_ratelimit(code: String) {
//     println!(
//         "[{}] Code {} wasn't checked because of ratelimit, switching proxy!",
//         local_time(),
//         code.blue()
//     );
// }

// pub fn print_error(code: String) {
//     println!(
//         "[{}] Code {} is not valid!", 
//         local_time(),
//         code.red()
//     );
// }