use clap::Parser;
use anyhow::{anyhow, Result};
use std::io;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use image::GenericImageView;
use image::io::Reader as ImageReader;
use indicatif::{ProgressBar, ProgressStyle};

/// Struct of the cli
#[derive(Parser)]
struct Cli {
    /// Path to where looking for the picture
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
    /// Path to where save the result
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

    let img = ImageReader::open(args.path)?.decode()?;

    //let res = img.grayscale();
    //res.save(args.result)?;
    /*

        for i in 0..100 {
            sleep(Duration::new(0, 20000000));
            pb.println(format!("[+] finished #{}", i));
            pb.inc(1);
        }
        */

    sleep(Duration::new(5, 0));
    pb.finish_with_message("Done ✅");
    Ok(())
}