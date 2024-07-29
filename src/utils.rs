use crate::uwu_bytes::UwUBytes;

pub fn uwu_bytes_to_metadata_string(uwu_bytes: &UwUBytes) -> String {
    format!(
        "uwu-codec ({}) [{}]", 
        uwu_bytes.version, 
        uwu_bytes.file_type.clone().unwrap_or("none".into())
    )
}

pub fn uwu_bytes_to_separated_string(uwu_bytes: &UwUBytes, sep: &str) -> String {
    uwu_bytes.bytes.join(sep)
}

pub fn parse_metadata_string(metadata_string: &String) -> (u8, Option<String>) {
    let version = metadata_string.split("(").nth(1)
        .expect("Failed to parse version from uwu codec metadata string!").split(")").nth(0).unwrap().parse::<u8>()
        .expect("Failed to convert version from metadata string to u8!");

    let mut file_type = Some(
        metadata_string.split("[").nth(1).expect(
            "Failed to parse file type from metadata string!"
        ).split("]").nth(0).unwrap().to_string()
    );

    if file_type == Some("none".into()) {
        file_type = None;
    }

    (version, file_type)
}