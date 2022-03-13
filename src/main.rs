use std::env::current_dir;
use std::io;
use std::io::Write;
use std::time::Duration;

use anyhow::{anyhow, Result};
use clap::Parser;
use image::io::Reader as ImageReader;
use indicatif::ProgressStyle;
use crate::Action::{Blur, Brighten, Contrast, Flipv, GreyScale, Resize, Rotate180, Rotate270, Rotate90};

/// image-utiliy: Does some basic operation on an image
///
/// Author: Kallu <lucas.aries@protonmail.com>
/// Github: "https://github.com/Kallu-A/"
#[derive(Parser)]
struct Cli {
    /// Path to the picture
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
    /// Path where the result is save
    #[clap(parse(from_os_str))]
    result: std::path::PathBuf,
    /// Action to realise possible valures are :
    ///
    /// blur, resize, greyScale, contrast, brighten, rotate90, rotate180, rotate270, flipv
    action: Action
}

/// Action representing all the possible action of the cli tools
enum Action {
    Blur, Resize, GreyScale, Contrast, Brighten, Rotate90, Rotate180, Rotate270, Flipv
}

impl ::core::str::FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "blur" => Ok(Blur),
            "resize" => Ok(Resize),
            "greyscale" => Ok(GreyScale),
            "contrast" => Ok(Contrast),
            "brighten" => Ok(Brighten),
            "rotate90" => Ok(Rotate90),
            "rotate190" => Ok(Rotate180),
            "rotate270" => Ok(Rotate270),
            "flipv" => Ok(Flipv),
            _ => Err("Incorrect action".to_string())
        }
    }
}

fn main() -> Result<()> {
    let args: Cli = Cli::parse();

    // verify path to image exist
    if !std::path::Path::new(&args.path).exists() {
        return Err(anyhow!(
            "could not read file `{}`",
            args.path.to_str().unwrap()
        ));
    }

    // verify path to image result if file already exist ask for overwrite else cancel operation
    if std::path::Path::new(&args.result).exists() {
        println!("Do you want to overwrite the existing file? `y/n`");
        let mut input = String::new();

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        // ask confirmation to overwrite
        if "Y\n" != input.to_uppercase() {
            return Err(anyhow!("can't save the result"));
        }
    }

    handler(args)?;
    Ok(())
}

/// Do the processing of the args after all the paths have been correctly tested
fn handler(args: Cli) -> Result<()> {
    let pb = indicatif::ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(80));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(&["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"]),
    );
    pb.set_message("Calculating...");

    action_do(&args)?;

    pb.finish_with_message("Done ✅");
    // display message to see the result
    if args.result.is_absolute() {
        println!("See result : \"file://{}\"", args.result.to_str().unwrap());
    } else {
        let path = current_dir().unwrap().join(&args.result);
        println!("See result : \"file://{}\"", path.to_str().unwrap());
    }
    Ok(())
}

fn action_do(args: &Cli) -> Result<()> {
    let img = ImageReader::open(&args.path)?.decode()?;



    res.save(&args.result)?
}