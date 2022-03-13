use clap::{arg, ColorChoice, Parser};
use anyhow::{anyhow, Context, Result};
use std::io;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use image::io::Reader as ImageReader;


#[derive(Parser)]
struct Cli {
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,

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

/*
    let content = std::fs::read_to_string(args.path.clone())
        .with_context(|| format!("could not read file `{}`", args.path.to_str().unwrap()))?;
    println!("file content: {}", content);*/

    let img = ImageReader::open(args.path)?.decode()?;

    let res = img.grayscale();
    res.save(args.result);

    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        sleep(Duration::new(0, 20000000));
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
    println!("environement setup");
    Ok(())
}
