mod args;
mod loader;

use crate::loader::load_image;
use clap::Parser;
use dotenv::dotenv;
use image::GenericImageView;
use serde_json::Value;
use std::env;
use std::error::Error;

fn show(path: String) -> Result<(), Box<dyn Error>> {
    let image = load_image(path)?;
    let rows = 40;
    let cols = 2 * image.width() * rows / image.height();

    asciify::AsciiBuilder::new_from_image(image)
        .set_resize((cols, rows))
        .to_std_out(true);

    Ok(())
}

fn describe(path: String) -> Result<(), Box<dyn Error>> {
    let key = env::var("AZURE_COMPUTER_VISION_API_KEY")?;
    let endpoint = env::var("AZURE_COMPUTER_VISION_API_ENDPOINT")?;

    let client = reqwest::blocking::Client::new();
    let response = client
        .post(&format!("{}/vision/v3.0/describe", endpoint))
        .header("Ocp-Apim-Subscription-Key", key)
        .header("Content-Type", "application/json")
        .body(format!("{{\"url\": \"{}\"}}", path))
        .send()?;

    let val: Value = serde_json::from_str(&response.text()?)?;
    println!("{}", val["description"]["captions"][0]["text"]);
    Ok(())
}

fn main() {
    dotenv().ok();
    let args = args::Args::parse();

    let result = match args.cmd {
        args::Command::Describe(d) => describe(d.path),
        args::Command::Show(s) => show(s.path),
    };

    if let Err(e) = result {
        eprintln!("{}", e);
    };
}
