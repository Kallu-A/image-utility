mod action;
mod filter;
mod histogram;
mod progress_bar_custom;

use std::borrow::BorrowMut;
use std::env::current_dir;

use crate::action::take_input;
use crate::histogram::{histogram_gray, histogram_rgb};
use crate::progress_bar_custom::progresse_bar_custom::ProgressBarCustom;
use crate::Action::{
    Blur, Brighten, Contrast, Edit, Filter, Fliph, Flipv, Grayscale, Histogram, Invert, Resize,
    Resizeratio, Rotate180, Rotate270, Rotate90,
};
use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use image::io::Reader as ImageReader;

#[derive(Parser)]
#[clap(name = "image-utility")]
#[clap(about = "Does some basic operation on an image \nFound a a issue ? go here: 'https://github.com/Kallu-A/image-utility'", long_about = None)]
#[clap(version = "0.2.0", author = "Kallu. <lucas.aries@protonmail.com>")]
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
    /// Filter the image with the specified mask 3x3
    Filter,
    /// Invert the color of the image
    Invert,
    /// Allows to perform multiple action on an image
    Edit,
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
    let mut pb_ref = ProgressBarCustom::create();
    let pb = pb_ref.borrow_mut();
    let img = ImageReader::open(&args.path)?.decode()?;

    let res = match args.action {
        Blur => action::blur(img, pb)?,
        Resize => action::resize(img, pb)?,
        Resizeratio => action::resize_ratio(img, pb)?,
        Grayscale => action::grayscale(img, pb)?,
        Contrast => action::constrast(img, pb)?,
        Brighten => action::brighten(img, pb)?,
        Rotate90 => action::rotate90(img, pb)?,
        Rotate180 => action::rotate180(img, pb)?,
        Rotate270 => action::rotate270(img, pb)?,
        Flipv => action::flipv(img, pb)?,
        Fliph => action::fliph(img, pb)?,
        Histogram => action::histogram(img, pb)?,
        Filter => action::filter3x3(img, pb)?,
        Invert => action::invert(img, pb)?,
        Edit => action::edit(img, pb, true)?,
    };

    res.save(&args.result)?;
    pb.done();
    Ok(())
}
