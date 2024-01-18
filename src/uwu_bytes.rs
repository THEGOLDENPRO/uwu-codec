use crate::uwu_decode;

pub struct UwUBytes {
    pub version: u8,
    pub bytes: Vec<String>,
    pub file_type: Option<String>
}

impl UwUBytes {
    pub fn from(version: u8, uwu_bytes: Vec<String>, file_type: Option<String>) -> Self {
        Self {
            version, bytes: uwu_bytes, file_type
        }
    }

    pub fn decode(self) -> Vec<u8> {
        uwu_decode(&self).unwrap()
    }
}