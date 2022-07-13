use rand::{Rng};
use crate::settings::{Settings};

pub fn generate_code(config: &Settings) -> String {
    const CHARSET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
    let mut final_string: String = String::from("");
    let code_length: u8;
    
    match config.boost {
        true => code_length = 24,
        false => code_length = 16,
    };

    let mut rng = rand::thread_rng();

    for _x in 0..code_length {
        let number: usize = rng.gen_range(1, CHARSET.len() as usize);
        final_string.push_str(&CHARSET[number - 1..number]);
    };

    return final_string;
}