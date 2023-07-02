use std::fs;
use std::os::unix::prelude::ExitStatusExt;
use std::path::PathBuf;
use std::process::Command;

use log::{error, info};

pub(crate) fn generate_thumbnail(path: String) -> Result<String, String> {
    let thumbnail_path = path.clone();
    let thumbnail_file = make_thumbnail_path(thumbnail_path);

    if fs::metadata(thumbnail_file.clone().unwrap()).is_ok() {
        return Ok(thumbnail_file.unwrap().to_str().unwrap().to_string());
    }

    info!("generating thumbnail: {}", thumbnail_file.clone().unwrap().to_str().unwrap());

    let output = Command::new("ffmpeg")
        .arg("-y")
        .args(["-ss", "00:10:00"])
        .args(["-i", path.as_str()])
        .args(["-frames:v", "1"])
        .args(["-q:v", "2", thumbnail_file.unwrap().to_str().unwrap()])
        .output()
        .expect("failed to execute command");

    if output.status.into_raw() != 0 {
        error!("ffmpeg error: {}", String::from_utf8(output.stderr).unwrap());
    }

    return Ok(String::from_utf8(Vec::from(output.stdout.as_slice())).unwrap());
}

fn make_thumbnail_path(path: String) -> Result<PathBuf, String> {
    let split: Vec<&str> = path.split("/").collect();
    let mut dir_path = PathBuf::from("/");

    let mut i = 0;
    while i < split.len() - 1 {
        dir_path = dir_path.join(split.get(i).unwrap());
        i += 1;
    }

    dir_path = dir_path.join(".thumbnails");
    if fs::metadata(dir_path.clone()).is_err() {
        info!("creating directory: {}", dir_path.to_str().unwrap());
        let result = fs::create_dir(dir_path.clone());
        if result.is_err() {
            return Err("failed to create directory".to_string());
        }
    }

    let file_path = dir_path.join(format!("{}-thumbnail.jpg", split.last().unwrap()));
    Ok(file_path)
}