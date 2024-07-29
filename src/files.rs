use std::{path::Path, fs, error::Error, env::var};

use uwu_codec::{multimedia::image_viewer::open_image, utils, uwu_bytes::UwUBytes, uwu_decode, uwu_encode};

const TEMP_DIR: &str = "/uwu-codec";

fn file_contents_to_uwu_bytes(file_contents: String) -> UwUBytes {
    let mut contents_lines = file_contents.lines();

    let metadata = contents_lines.next().expect("File contains nothing!").to_string();
    let uwu_bytes = contents_lines.next().expect("UwU bytes not found in file contents!");

    let (version, file_type) = utils::parse_metadata_string(&metadata);

    UwUBytes::from(
        version,
        uwu_bytes.to_string().split(",").map(|x| x.to_string()).collect(),
        file_type
    )
}

fn uwu_bytes_to_file_contents(uwu_bytes: &UwUBytes) -> String {
    utils::uwu_bytes_to_metadata_string(uwu_bytes) + "\n" + utils::uwu_bytes_to_separated_string(uwu_bytes, ",").as_str()
}

/*
fn open_unknown_file(file_type: &str, uwu_bytes: &UwUBytes) -> Result<(), Box<dyn Error>> {
    let output_file_path = get_path(Some(&format!("/owo.{}", file_type)));
    let output_file = Path::new(&output_file_path);

    let contents = match uwu_decode(uwu_bytes) {
        Ok(value) => value,
        Err(e) => return Err(format!("Failed to decode uwu bytes for unknown file contents! Error: {}", e).into())
    };

    fs::write(&output_file, contents).expect(
        "Failed to decode to output file!"
    );

    // actually opening it...
    let process = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/c")
            .arg(output_file)
            .spawn()
    } else { // If you want MacOS support, contribute please.
        Command::new("xdg-open")
            .arg(output_file)
            .spawn()
    };

    process.expect("Failed to open file!");

    Ok(())
}
 */

pub fn get_path(target_file: Option<&str>) -> String {
    let target_file = target_file.unwrap_or("".into()).to_owned();

    if cfg!(target_os = "windows") {
        var("AppData").expect("Failed to find Windows AppData environment variable!") + &(TEMP_DIR.to_owned() + &target_file).replace("/", r"\")
    } else { // If you want MacOS support, contribute please.
        var("HOME").expect("Failed to find HOME environment variable! Are you on Linux?") + "/.cache" + &TEMP_DIR + &target_file
    }
}

pub fn check_dir() {
    if !(fs::metadata(get_path(None)).is_ok()) {
        fs::create_dir(get_path(None)).expect("Couldn't create temporary directory.");
    }
}

pub fn convert_file<'a>(target_file: &'a Path, output_file: &'a Path) -> Result<(), Box<dyn Error>> {
    let target_file_extension = target_file.extension().expect("Failed to get target file extension!").to_str();
    let output_file_extension = output_file.extension().expect("Failed to get output file extension!").to_str();

    let target_file_uwu_bytes = match target_file_extension {
        Some("uwu") | Some("owo") => file_contents_to_uwu_bytes(fs::read_to_string(target_file).unwrap()),
        file_type => {
            let mut uwu_bytes = match uwu_encode(
                &fs::read(target_file).expect("Error occurred while reading target file!"), 2
            ) {
                Ok(value) => value,
                Err(e) => return Err(e)
            };
            uwu_bytes.file_type = Some(file_type.unwrap().to_string());
            uwu_bytes
        }
    };

    match output_file_extension {
        Some("uwu") | Some("owo") => {
            let contents = uwu_bytes_to_file_contents(&target_file_uwu_bytes);
            fs::write(output_file, contents).expect("Failed to encode uwu bytes to output file!")
        },
        _ => {
            let contents = uwu_decode(&target_file_uwu_bytes).unwrap();
            fs::write(output_file, contents).expect("Failed to decode to output file!")
        }
    }

    Ok(())
}

pub fn open_file(target_file: &Path) -> Result<(), Box<dyn Error>> {
    let target_file_extension = target_file.extension().expect("Failed to get target file extension!").to_str();

    let target_file_uwu_bytes = match target_file_extension {
        Some("uwu") | Some("owo") => file_contents_to_uwu_bytes(fs::read_to_string(target_file).unwrap()),
        _ => {
            return Err("target file is not an '.uwu' or '.owo' file.".into());
        }
    };

    let uwu_bytes_file_type = &target_file_uwu_bytes.file_type.clone().expect(
        "Target file type is unknown, so we can not open the file!"
    );

    match uwu_bytes_file_type.as_str() {
        "png"|"svg" => open_image(&target_file_uwu_bytes), 
        // TODO: Work on video player.
        other => Err(format!("The '{}' file type is not supported yet!", other).into())
    }
}