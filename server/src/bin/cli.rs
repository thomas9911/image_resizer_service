use std::path::PathBuf;

// use libresizer::image;
use libresizer::proto;
use image::io::Reader as ImageReader;

#[derive(Debug, Default)]
struct Config{
    input_image: PathBuf,
    output_image: PathBuf,
    method: proto::ResizeMethod,
    x: u32,
    y: u32,

}

const HELP: &'static str = r#"
convert image

usage: 
    cargo run --release --example cli -- INPUT OUTPUT X Y [METHOD]


METHOD can be one of:
- Fill
- Fit
- Limit
- Pad

"#;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cfg = Config::default();

    for (i, argument) in std::env::args().enumerate() {
        if argument == "--help" {
            println!("{}", HELP);
            return Ok(())
        }

        match i {
            1 => {cfg.input_image = argument.try_into().unwrap();}
            2 => {cfg.output_image = argument.try_into().unwrap();}
            3 => {cfg.x = argument.parse().unwrap();}
            4 => {cfg.y = argument.parse().unwrap();}
            5 => {cfg.method = argument.try_into().unwrap();}

            _ => ()
        };

    }

    dbg!(&cfg);

    let img = ImageReader::open(cfg.input_image)?.decode()?;
    let format = image::ImageFormat::from_path(&cfg.output_image)?;
    let out = libresizer::image::resize(&img, format, cfg.x, cfg.y, cfg.method)?;
    std::fs::write(cfg.output_image, out)?;

    Ok(())
}

