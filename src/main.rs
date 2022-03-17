mod histogram;
mod progress_bar_custom;

use std::env::current_dir;
use std::io;
use std::io::Write;

use crate::histogram::{histogram_gray, histogram_rgb};
use crate::progress_bar_custom::progresse_bar_custom::ProgressBarCustom;
use crate::Action::{
    Blur, Brighten, Contrast, Fliph, Flipv, Grayscale, Histogram, Resize, Resizeratio, Rotate180,
    Rotate270, Rotate90,
};
use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::DynamicImage;

/// Found a issue ? go here : "https://github.com/Kallu-A/image-utility"
#[derive(Parser)]
#[clap(name = "image-utility")]
#[clap(about = "Does some basic operation on an image", long_about = None)]
#[clap(version = "1.0", author = "Kallu. <lucas.aries@protonmail.com>")]
struct Cli {
    /// Path to the picture you want to transform
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
    /// Path where the result is of the transformation is save
    #[clap(parse(from_os_str))]
    result: std::path::PathBuf,
    /// Action to realise to the image
    #[clap(subcommand)]
    action: Action,
}

/// Action representing all the possible action of the cli tools
#[derive(Subcommand)]
enum Action {
    /// Perform a Gaussian blur with a sigma value who determined how much to blur it
    Blur,
    /// Resize a image without preserving the ratio at the new width and height
    Resize,
    /// Resize a image and preserve the ratio of the new width and height
    Resizeratio,
    /// Return the grayscale of the image (only gray use)
    Grayscale,
    /// Adjust the contrast by taking a value. Negative reduces the contrast positive increase it
    Contrast,
    /// Take a value it will be the value added to every color of the pixel (positive increase brightness / negative decrease)
    Brighten,
    /// Rotate 90° clockwise
    Rotate90,
    /// Rotate 180° clockwise
    Rotate180,
    /// Rotate 270° clockwise
    Rotate270,
    /// Flip the image vertically
    Flipv,
    /// Flip the image horizontally
    Fliph,
    /// Create the histogram of the image, `gray` parameter does the average of the RGB, `rgb` do 3 curves for each color
    Histogram,
}

/// Function who take a input
fn take_input(message: &str) -> String {
    let mut input = String::new();
    println!("{}", message);

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
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
        let input = take_input("Do you want to overwrite the existing file? `y/n`");

        // ask confirmation to overwrite
        if "Y" != input.to_uppercase() {
            return Err(anyhow!("can't save the result"));
        }
    }

    handler(args)?;
    Ok(())
}

/// Do the processing of the args after all the paths have been correctly tested
fn handler(args: Cli) -> Result<()> {
    action_do(&args)?;

    // display message to see the result
    if args.result.is_absolute() {
        println!("See result: \"file://{}\"", args.result.to_str().unwrap());
    } else {
        let path = current_dir().unwrap().join(&args.result);
        println!("See result: \"file://{}\"", path.to_str().unwrap());
    }
    Ok(())
}

fn action_do(args: &Cli) -> Result<()> {
    let pb = ProgressBarCustom::create();
    let img = ImageReader::open(&args.path)?.decode()?;

    let res = match args.action {
        Blur => blur_action(img, &pb)?,
        Resize => resize_action(img, &pb)?,
        Resizeratio => resize_ratio_action(img, &pb)?,
        Grayscale => grayscale_action(img, &pb)?,
        Contrast => constrast_action(img, &pb)?,
        Brighten => brighten_action(img, &pb)?,
        Rotate90 => rotate90_action(img, &pb)?,
        Rotate180 => rotate180_action(img, &pb)?,
        Rotate270 => rotate270_action(img, &pb)?,
        Flipv => flipv_action(img, &pb)?,
        Fliph => fliph_action(img, &pb)?,
        Histogram => histogram_action(img, &pb)?,
    };

    res.save(&args.result)?;
    pb.done();
    Ok(())
}

// All the subcommands actions below

/// Do the blur action
fn blur_action(img: DynamicImage, pb: &ProgressBarCustom) -> Result<DynamicImage, anyhow::Error> {
    let input = take_input("How much you want it blur: `[0.0; 40.0]`");
    pb.launch();
    Ok(img.blur(
        input
            .parse::<f32>()
            .with_context(|| format!("invalid sigma value, {}", input))?,
    ))
}

/// Do the resize action
fn resize_action(img: DynamicImage, pb: &ProgressBarCustom) -> Result<DynamicImage, anyhow::Error> {
    let input = take_input("new dimension : `width/height`");
    let input_vec = input.split('/').collect::<Vec<&str>>();

    if input_vec.len() != 2 {
        return Err(anyhow!("invalid arguments `width/height` got `{}`", input));
    }

    let nwidth = input_vec[0]
        .parse::<u32>()
        .with_context(|| format!("invalid format value, {}", input))?;
    let nheight = input_vec[1]
        .parse::<u32>()
        .with_context(|| format!("invalid format value, {}", input))?;

    let filter = filter_ask()?;
    pb.launch();
    Ok(img.resize_exact(nwidth, nheight, filter))
}

/// Do the resize_ratio action
fn resize_ratio_action(
    img: DynamicImage,
    pb: &ProgressBarCustom,
) -> Result<DynamicImage, anyhow::Error> {
    let input = take_input("new dimension : `width/height`");
    let input_vec = input.split('/').collect::<Vec<&str>>();

    if input_vec.len() != 2 {
        return Err(anyhow!("invalid arguments `width/height` got `{}`", input));
    }

    let nwidth = input_vec[0]
        .parse::<u32>()
        .with_context(|| format!("invalid format value, {}", input))?;
    let nheight = input_vec[1]
        .parse::<u32>()
        .with_context(|| format!("invalid format value, {}", input))?;

    let filter = filter_ask()?;
    pb.launch();
    Ok(img.resize(nwidth, nheight, filter))
}

/// Do the greyscale action
fn grayscale_action(
    img: DynamicImage,
    pb: &ProgressBarCustom,
) -> Result<DynamicImage, anyhow::Error> {
    pb.launch();
    Ok(img.grayscale())
}

/// Do the contrast action
fn constrast_action(
    img: DynamicImage,
    pb: &ProgressBarCustom,
) -> Result<DynamicImage, anyhow::Error> {
    let input = take_input("Adjust the contrast: `[-80.0; 200.0]`");
    pb.launch();
    Ok(img.adjust_contrast(
        input
            .parse::<f32>()
            .with_context(|| format!("invalid value, {}", input))?,
    ))
}

/// Do the brighten action
fn brighten_action(
    img: DynamicImage,
    pb: &ProgressBarCustom,
) -> Result<DynamicImage, anyhow::Error> {
    let input = take_input("Adjust the brightness: `[-255; 255]`");
    pb.launch();
    Ok(img.brighten(
        input
            .parse::<i32>()
            .with_context(|| format!("invalid value, {}", input))?,
    ))
}

/// Do the rotate90 action
fn rotate90_action(
    img: DynamicImage,
    pb: &ProgressBarCustom,
) -> Result<DynamicImage, anyhow::Error> {
    pb.launch();
    Ok(img.rotate90())
}

/// Do the rotate180 action
fn rotate180_action(
    img: DynamicImage,
    pb: &ProgressBarCustom,
) -> Result<DynamicImage, anyhow::Error> {
    pb.launch();
    Ok(img.rotate180())
}

/// Do the rotate270 action
fn rotate270_action(
    img: DynamicImage,
    pb: &ProgressBarCustom,
) -> Result<DynamicImage, anyhow::Error> {
    pb.launch();
    Ok(img.rotate270())
}

/// Do the flipv action
fn flipv_action(img: DynamicImage, pb: &ProgressBarCustom) -> Result<DynamicImage, anyhow::Error> {
    pb.launch();
    Ok(img.flipv())
}

/// Do the fliph action
fn fliph_action(img: DynamicImage, pb: &ProgressBarCustom) -> Result<DynamicImage, anyhow::Error> {
    pb.launch();
    Ok(img.fliph())
}

fn histogram_action(
    img: DynamicImage,
    pb: &ProgressBarCustom,
) -> Result<DynamicImage, anyhow::Error> {
    let input = take_input("Only gray color , or RGB ? `G/RGB`").to_uppercase();
    let res = if input == "RGB" {
        histogram_rgb(img)
    } else if input == "G" {
        histogram_gray(img)
    } else {
        return Err(anyhow!("Wrong arguments"));
    }
    .unwrap();
    pb.launch();

    Ok(res)
}

/// ask for a filter and return the filter
fn filter_ask() -> Result<FilterType, anyhow::Error> {
    let input = take_input("Filter: \n`1` = speed:fast, quality:low \n`2` = speed:medium quality:medium \n`3` = speed:slow quality:high");
    let filter_u8 = input
        .parse::<u8>()
        .with_context(|| format!("invalid filter value, {}", input))?;

    match filter_u8 {
        1 => Ok(FilterType::Nearest),
        2 => Ok(FilterType::CatmullRom),
        3 => Ok(FilterType::Gaussian),
        _ => Err(anyhow!("invalid filter value")),
    }
}
