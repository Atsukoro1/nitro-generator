use std::{io};

pub struct Settings {
    pub boost: bool,
    pub code_count: u128
}


pub fn conf_settings() -> Settings {
    let mut config: Settings = Settings{
        boost: false,
        code_count: 0
    };

    let mut icode_type: String = String::from("");
    let mut icode_count: String = String::from("");

    println!("What type of codes to generate [boost/classic]: ");
    io::stdin()
        .read_line(&mut icode_type)
        .expect("Failed to read this line!");

    println!("How many codes do you want to generate [number]: ");
    io::stdin()
        .read_line(&mut icode_count)
        .expect("Cannot read this line!");

    config.code_count = icode_count
        .trim()
        .parse()
        .expect("Please enter a valid number!");

    if icode_type == "boost" {
        config.boost = true;
    } else {
        config.boost = false;
    }

    return config;
}