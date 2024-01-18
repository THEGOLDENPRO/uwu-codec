use std::collections::HashMap;
use std::error::Error;

use maplit::hashmap;

fn get_table() -> HashMap<&'static str, &'static str> {
    hashmap! {
        "0" => "OwO",
        "1" => "ôwô",
        "2" => "W",
        "3" => "(ꈍᴗꈍ)",
        "4" => "ÖwÖ",
        "5" => "o",
        "6" => "ÒwÒ",
        "7" => "(〃￣ω￣〃)ゞ",
        "8" => "UwU",
        "9" => "( ੭•͈ω•͈)੭"
    }
}

pub fn encode(bytes: &Vec<u8>) -> Result<Vec<String>, Box<dyn Error>> {
    let owo_table = get_table();

    let mut uwu_bytes: Vec<String> = Vec::new();

    for byte in bytes.iter() {

        let mut uwu_byte = String::new();

        for char in byte.to_string().chars() {

            let uwu_char = match owo_table.get(char.to_string().as_str()) {
                None => {
                    return Err(
                        format!("Opposiey dasiy, the character '{}' failed to encode to an uwu byte!", char).into()
                    )
                },
                uwu_char => uwu_char.unwrap()
            };

            uwu_byte.extend(uwu_char.chars());

        }

        uwu_bytes.push(uwu_byte);

    }

    Ok(uwu_bytes)
}

pub fn decode(uwu_bytes: &Vec<String>) -> Result<Vec<u8>, Box<dyn Error>> { // If this stuff fails, I don't care not my problem.
    let owo_table: HashMap<&str, &str> = get_table();

    let mut bytes: Vec<u8> = Vec::new();

    for uwu_byte in uwu_bytes {
        let mut uwu_byte_transcoded = uwu_byte.to_string();

        for owo_char in owo_table.iter() {
            uwu_byte_transcoded = uwu_byte_transcoded.replace(owo_char.1, owo_char.0);
        }

        let uwu_byte_phrased = uwu_byte_transcoded.parse::<u8>();

        let uwu_byte_u8 = match uwu_byte_phrased {
            Ok(val) => val,
            Err(e) => {
                return Err(
                    format!(
                        "Invalid uwu byte '{}'. Transcoded uwu byte: '{}'. Couldn't decode with uwu codec v2! Error: {}", 
                        uwu_byte, uwu_byte_transcoded, e
                    ).into()
                )
            }
        };

        bytes.push(uwu_byte_u8);

    }

    Ok(bytes)
}