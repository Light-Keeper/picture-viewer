use image::{DynamicImage, GenericImageView};
use std::error::Error;

fn is_remote_url(path: &str) -> bool {
    path.starts_with("http://") || path.starts_with("https://")
}

fn load_from_url(path: String) -> Result<Vec<u8>, Box<dyn Error>> {
    let response = reqwest::blocking::get(path)?;
    let content = response.bytes()?;
    Ok(content.to_vec())
}

fn load_from_file(path: String) -> Result<Vec<u8>, Box<dyn Error>> {
    let content = std::fs::read(path)?;
    Ok(content)
}

pub fn load_image(path: String) -> Result<DynamicImage, Box<dyn Error>> {
    let content = if is_remote_url(&path) {
        load_from_url(path)?
    } else {
        load_from_file(path)?
    };

    let image = image::load_from_memory(&content)?;
    Ok(image)
}
