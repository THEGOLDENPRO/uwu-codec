use std::collections::HashMap;
use std::error::Error;

use maplit::hashmap;

fn get_table() -> HashMap<u8, String> {
    let mapping  = hashmap! {
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
    };

    let mut table: HashMap<u8, String> = HashMap::new();

    for num in 0..256 {
        let mut uwu_byte = String::new();

        for char in num.to_string().chars() {
            uwu_byte.push_str(mapping.get(char.to_string().as_str()).unwrap());
        }

        table.insert(num as u8, uwu_byte);
    }

    table
}

pub fn encode(bytes: &Vec<u8>) -> Result<Vec<String>, Box<dyn Error>> {
    let owo_table = get_table();

    let mut uwu_bytes: Vec<String> = Vec::new();

    for byte in bytes.iter() {

        let uwu_byte = match owo_table.get(byte) {
            None => {
                return Err(
                    format!("Opposiey dasiy, the byte '{}' failed to encode into an uwu byte!", byte).into()
                )
            },
            Some(uwu_byte) => uwu_byte
        };

        uwu_bytes.push(uwu_byte.to_owned());

    }

    Ok(uwu_bytes)
}

pub fn decode(uwu_bytes: &Vec<String>) -> Result<Vec<u8>, Box<dyn Error>> { // If this stuff fails, I don't care not my problem.
    let owo_table = get_table();
    let owo_table = owo_table.iter().map(|(k, v)| (v, k)).collect::<HashMap<&String, &u8>>();

    let mut bytes: Vec<u8> = Vec::new();

    for uwu_byte in uwu_bytes {
        let mut uwu_byte_transcoded = uwu_byte.to_string();

        let owo_char = match owo_table.get(&uwu_byte) {
            Some(value) => value,
            None => return Err(format!("Could not decode uwu byte '{}' with uwu-codec v2!", uwu_byte).into())
        };

        uwu_byte_transcoded = uwu_byte_transcoded.replace(uwu_byte, owo_char.to_string().as_str());

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