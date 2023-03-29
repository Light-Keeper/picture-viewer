use std::env;
use clap::Parser;
use image::{DynamicImage, GenericImageView};
use std::error::Error;
use dotenv::dotenv;
use serde_json::Value;

#[derive(Parser, Debug)]
struct Describe {
    path: String,
}

#[derive(Parser, Debug)]
struct Show {
    path: String,
}

#[derive(Parser, Debug)]
enum Command {
    /// command 1
    #[clap(name = "describe")]
    Describe(Describe),
    /// command 2
    #[clap(name = "show")]
    Show(Show),
}

#[derive(Parser, Debug)]
struct Args {
    /// command
    #[clap(subcommand)]
    cmd: Command,
}

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

fn load_image(path: String) -> Result<DynamicImage, Box<dyn Error>> {
    let content = if is_remote_url(&path) {
        load_from_url(path)?
    } else {
        load_from_file(path)?
    };

    let image = image::load_from_memory(&content)?;
    Ok(image)
}

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
    let args: Args = Args::parse();

    let result = match args.cmd {
        Command::Describe(d) => describe(d.path),
        Command::Show(s) => show(s.path),
    };

    if let Err(e) = result {
        eprintln!("{}", e);
    };
}
