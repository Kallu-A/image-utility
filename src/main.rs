use std::env::current_dir;
use std::io;
use std::io::Write;
use std::time::Duration;

use clap::Parser;
use anyhow::{anyhow, Result};
use image::io::Reader as ImageReader;
use indicatif::ProgressStyle;

/// Image Process: Does some basic operation on an image
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
}

fn main() -> Result<()> {
    let args:Cli = Cli::parse();

    // verify path to image exist
    if !std::path::Path::new(&args.path).exists() {
        return Err(anyhow!("could not read file `{}`", args.path.to_str().unwrap()));
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
            .tick_strings(&[
                "⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"
            ]),
    );
    pb.set_message("Calculating...");


    let img = ImageReader::open(&args.path)?.decode()?;
    let res = img.grayscale();
    res.save(&args.result)?;


    pb.finish_with_message("Done ✅");
    // display message to see the result
    if args.result.is_absolute() {
        println!("See result : \"file://{}\"", args.result.to_str().unwrap());
    } else {
        let path = current_dir().unwrap().join(args.result);
        println!("See result : \"file://{}\""
                 , path.to_str().unwrap());
    }
    Ok(())
}