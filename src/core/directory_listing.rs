use std::fs;

use base64::{engine, Engine};
use log::info;
use serde::Serialize;
use strum::VariantNames;

use crate::core::enums::subtitle::SubtitleExtension;
use crate::core::enums::video::VideoExtension;
use crate::core::helpers::ffmpeg::generate_thumbnail;

#[derive(Serialize, Clone)]
pub(crate) struct DirectoryListing {
    pub(crate) items: Vec<Item>,
    pub(crate) total: usize,
}

#[derive(Serialize, Clone)]
pub(crate) struct Item {
    id: String,
    path: String,
    name: String,
    thumbnail: Option<String>,
    is_file: bool,
    is_video: bool,
    is_subtitle: bool,
}

pub(crate) fn discover_files(path: &str) -> DirectoryListing {
    let mut items: Vec<Item> = vec![];
    let paths = fs::read_dir(path).unwrap();

    for path in paths {
        if path.is_ok() {
            let path = path.unwrap();
            let item_type = path.file_type().unwrap();

            if item_type.is_file() {
                let path = path.path();
                let ext = path.extension();

                if ext.is_none() {
                    info!("file doesn't have ext, skipping: {}", path.to_str().unwrap());
                    continue;
                }

                let extension = ext.unwrap().to_str().unwrap();
                let item_path = String::from(path.to_str().unwrap());
                let is_video = VideoExtension::VARIANTS.contains(&extension);
                let thumbnail = if is_video { Some(generate_thumbnail(item_path.clone()).unwrap()) } else { None };

                items.push(Item {
                    id: engine::general_purpose::URL_SAFE.encode(item_path.clone()),
                    name: item_path.split("/").last().unwrap().to_string(),
                    path: item_path.clone(),
                    is_file: item_type.is_file(),
                    is_video,
                    thumbnail,
                    is_subtitle: SubtitleExtension::VARIANTS.contains(&extension),
                });
            }
        }
    }

    DirectoryListing {
        total: items.len(),
        items,
    }
}
