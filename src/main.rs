mod histogram;

use std::env::current_dir;
use std::io;
use std::io::Write;
use std::time::Duration;

use crate::histogram::{histogram_gray, histogram_rgb};
use crate::Action::{
    Blur, Brighten, Contrast, Fliph, Flipv, GrayScale, Histogram, Resize, Rotate180, Rotate270,
    Rotate90,
};
use anyhow::{anyhow, Context, Result};
use clap::Parser;
use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::DynamicImage;
use indicatif::{ProgressBar, ProgressStyle};

/// Does some basic operation on an image
///
/// Author: Kallu <lucas.aries@protonmail.com>
/// Github: "https://github.com/Kallu-A/"
///
/// Found a issue ? go here : "https://github.com/Kallu-A/image-utility"
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
    /// blur, resize, greyScale, contrast, brighten, rotate90, rotate180, rotate270, flipv, fliph, histogram
    action: Action,
}

struct ProgressBarCustom {
    bar: ProgressBar,
}

/// Handle all action about the progressbar
impl ProgressBarCustom {
    /// create the progressbar
    pub fn create() -> ProgressBarCustom {
        let pb = indicatif::ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::with_template("{spinner:.blue} {msg}")
                .unwrap()
                .tick_strings(&["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"]),
        );
        ProgressBarCustom { bar: pb }
    }

    /// launch the progressbar
    fn launch(&self) {
        self.bar.set_message("Calculating...");
        self.bar.enable_steady_tick(Duration::from_millis(80));
    }

    /// End the progressbar
    fn done(&self) {
        self.bar.finish_with_message("Done ✅");
    }
}

/// Action representing all the possible action of the cli tools
enum Action {
    Blur,
    Resize,
    GrayScale,
    Contrast,
    Brighten,
    Rotate90,
    Rotate180,
    Rotate270,
    Flipv,
    Fliph,
    Histogram,
}

impl ::core::str::FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "blur" => Ok(Blur),
            "resize" => Ok(Resize),
            "grayscale" => Ok(GrayScale),
            "contrast" => Ok(Contrast),
            "brighten" => Ok(Brighten),
            "rotate90" => Ok(Rotate90),
            "rotate180" => Ok(Rotate180),
            "rotate270" => Ok(Rotate270),
            "flipv" => Ok(Flipv),
            "fliph" => Ok(Fliph),
            "histogram" => Ok(Histogram),
            _ => Err("Incorrect actions possibles are: blur, resize, greyScale, contrast, brighten, rotate90, rotate180, rotate270, flipv, fliph, histogram".to_string()),
        }
    }
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
        GrayScale => grayscale_action(img, &pb)?,
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

    pb.launch();
    Ok(img.resize(nwidth, nheight, FilterType::CatmullRom))
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
