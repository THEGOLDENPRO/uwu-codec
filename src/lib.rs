use std::error::Error;

use oem_cp::{decode_string_complete_table, code_table::{DECODING_TABLE_CP437, ENCODING_TABLE_CP437}, encode_string_checked};
use uwu_bytes::UwUBytes;

pub mod codecs;
pub mod uwu_bytes;
pub mod multimedia;

/// Encodes bytes to 😳 uwu bytes.
pub fn uwu_encode(bytes: &Vec<u8>, version: u8) -> Result<UwUBytes, Box<dyn Error>> {
    let uwu_bytes = match version {
        2 => codecs::v2::encode(bytes),
        _ => Err("Unknown version!".into())
    };

    match uwu_bytes {
        Ok(value) => {
            Ok(UwUBytes::from(version, value, None))
        },
        Err(e) => Err(e)
    }
}

/// Encodes string to 😳 uwu bytes.
pub fn uwu_encode_string(string: &str, version: u8) -> Result<UwUBytes, Box<dyn Error>> {
    uwu_encode(&encode_string_checked(string, &ENCODING_TABLE_CP437).unwrap(), version)
}

/// Decodes 😳 uwu bytes back to traditional bytes.
pub fn uwu_decode(uwu_bytes: &UwUBytes) -> Result<Vec<u8>, Box<dyn Error>> {
    match uwu_bytes.version {
        2 => codecs::v2::decode(&uwu_bytes.bytes),
        _ => Err("Unknown version!".into())
    }
}

/// Decodes 😳 uwu bytes back to a traditional string.
pub fn uwu_decode_string(uwu_bytes: &UwUBytes) -> Result<String, Box<dyn Error>> {
    match uwu_decode(uwu_bytes) {
        Ok(value) => Ok(decode_string_complete_table(value, &DECODING_TABLE_CP437)),
        Err(e) => Err(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let uwu_bytes_v2 = uwu_encode(&"jeff".bytes().collect::<Vec<u8>>(), 2);

        assert_eq!(uwu_bytes_v2.unwrap().bytes, vec!["ôwôOwOÒwÒ", "ôwôOwOôwô", "ôwôOwOW", "ôwôOwOW"]);
    }
}
