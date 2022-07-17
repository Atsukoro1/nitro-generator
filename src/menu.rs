use dialoguer::{
    theme,
    Select,
    Input,
    Confirm
};
use crate::proxy::{self, ProxySource};

pub struct Settings {
    pub boost: bool,
    pub code_count: u128,
    pub thread_count: u32,
    pub proxy_souce: proxy::ProxySource,
}

pub async fn draw_menu() -> Settings {
    if !Confirm::with_theme(&theme::ColorfulTheme::default()).with_prompt("Do you want to continue?").interact().unwrap() {
        std::process::exit(1);
    };

    let chosen_type: bool = match Select::with_theme(&theme::ColorfulTheme::default())
        .item("Boost")
        .item("Classic")
        .with_prompt("Select Nitro type")
        .interact()
        .unwrap() {
            0 => true,
            1 => false, 
            _ => true,
        };

    let chosen_count: u128 = Input::with_theme(&theme::ColorfulTheme::default())
        .with_prompt("How many codes you want to generate")
        .validate_with(|input: &String| -> Result<(), String> {
            match input.parse::<u128>() {
                Ok(..) => {
                    Ok(())
                }

                Err(..) => {
                    Err("This is not a valid count".to_string())
                }
            }
        })
        .interact()
        .unwrap()
        .parse::<u128>()
        .unwrap();

    let thread_count: u32 = Input::with_theme(&theme::ColorfulTheme::default())
        .with_prompt("How many threads do you want to use")
        .validate_with(|input: &String| -> Result<(), String> {
            match input.parse::<u32>() {
                Ok(..) => {
                    Ok(())

                }

                Err(..) => {
                    Err("Can't parse this value as an integer".to_string())
                }
            }
        })
        .interact()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let chosen_ps: ProxySource = match Select::with_theme(&theme::ColorfulTheme::default())
        .clear(true)
        .item("Proxyscrape")
        .item("Local")
        .item("Geonode")
        .interact()
        .unwrap() {
            0 => ProxySource::Proxyscrape,
            1 => ProxySource::Local,
            2 => ProxySource::Geonode,
            _ => ProxySource::Proxyscrape
    };

    Settings {
        proxy_souce: chosen_ps,
        boost: chosen_type,
        code_count: chosen_count,
        thread_count: thread_count
    }
}