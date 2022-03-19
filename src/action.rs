use std::borrow::BorrowMut;
use crate::filter::sobel;
use crate::{action, histogram_gray, histogram_rgb, ProgressBarCustom};
use anyhow::{anyhow, Context};
use image::imageops::FilterType;
use image::DynamicImage;
use std::io;
use std::io::Write;

/// Function who take a input
pub fn take_input(message: &str) -> String {
    let mut input = String::new();
    println!("{}", message);

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

/// ask for a filter and return the filter
fn filter_ask_resize() -> Result<FilterType, anyhow::Error> {
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

// All the subcommands actions below

/// Do the blur action
pub fn blur(img: DynamicImage, pb: &mut ProgressBarCustom) -> Result<DynamicImage, anyhow::Error> {
    let input = take_input("How much you want it blur: `[0.0; 40.0]`");
    pb.launch();
    Ok(img.blur(
        input
            .parse::<f32>()
            .with_context(|| format!("invalid sigma value, {}", input))?,
    ))
}

/// Do the resize action
pub fn resize(
    img: DynamicImage,
    pb: &mut ProgressBarCustom,
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

    let filter = filter_ask_resize()?;
    pb.launch();
    Ok(img.resize_exact(nwidth, nheight, filter))
}

/// Do the resize_ratio action
pub fn resize_ratio(
    img: DynamicImage,
    pb: &mut ProgressBarCustom,
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

    let filter = filter_ask_resize()?;
    pb.launch();
    Ok(img.resize(nwidth, nheight, filter))
}

/// Do the greyscale action
pub fn grayscale(
    img: DynamicImage,
    pb: &mut ProgressBarCustom,
) -> Result<DynamicImage, anyhow::Error> {
    pb.launch();
    Ok(img.grayscale())
}

/// Do the contrast action
pub fn constrast(
    img: DynamicImage,
    pb: &mut ProgressBarCustom,
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
pub fn brighten(
    img: DynamicImage,
    pb: &mut ProgressBarCustom,
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
pub fn rotate90(
    img: DynamicImage,
    pb: &mut ProgressBarCustom,
) -> Result<DynamicImage, anyhow::Error> {
    pb.launch();
    Ok(img.rotate90())
}

/// Do the rotate180 action
pub fn rotate180(
    img: DynamicImage,
    pb: &mut ProgressBarCustom,
) -> Result<DynamicImage, anyhow::Error> {
    pb.launch();
    Ok(img.rotate180())
}

/// Do the rotate270 action
pub fn rotate270(
    img: DynamicImage,
    pb: &mut ProgressBarCustom,
) -> Result<DynamicImage, anyhow::Error> {
    pb.launch();
    Ok(img.rotate270())
}

/// Do the flipv action
pub fn flipv(img: DynamicImage, pb: &mut ProgressBarCustom) -> Result<DynamicImage, anyhow::Error> {
    pb.launch();
    Ok(img.flipv())
}

/// Do the fliph action
pub fn fliph(img: DynamicImage, pb: &mut ProgressBarCustom) -> Result<DynamicImage, anyhow::Error> {
    pb.launch();
    Ok(img.fliph())
}

pub fn histogram(
    img: DynamicImage,
    pb: &mut ProgressBarCustom,
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

/// Do the invert
pub fn invert(
    mut img: DynamicImage,
    pb: &mut ProgressBarCustom,
) -> Result<DynamicImage, anyhow::Error> {
    pb.launch();
    img.invert();
    Ok(img)
}

/// Apply a filter
pub fn filter3x3(
    img: DynamicImage,
    pb: &mut ProgressBarCustom,
) -> Result<DynamicImage, anyhow::Error> {
    let input = take_input(
        "Wich filter you want ?\
    \n 'low-pass': reduce high frequency\
    \n 'high-pass': reduce low frequency\
    \n 'sobel-x': detect the vertical border\
    \n 'sobel-y': detect the horizontal border\
    \n 'sobel-xy': detect vertical and horizontal border",
    )
    .to_uppercase();
    let filter = match input.as_str() {
        "LOW-PASS" => &[
            1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32, 1_f32,
        ],
        "HIGH-PASS" => &[
            -1_f32, -1_f32, -1_f32, -1_f32, 16_f32, -1_f32, -1_f32, -1_f32, -1_f32,
        ],
        "SOBEL-X" => &[
            -1_f32, 0_f32, 1_f32, -2_f32, 0_f32, 2_f32, -1_f32, 0_f32, 1_f32,
        ],
        "SOBEL-Y" => &[
            -1_f32, -2_f32, -1_f32, 0_f32, 0_f32, 0_f32, 1_f32, 2_f32, 1_f32,
        ],
        "SOBEL-XY" => {
            pb.launch();
            return Ok(sobel(img));
        }
        _ => return Err(anyhow!("Wrong filter")),
    };
    pb.launch();
    Ok(img.filter3x3(filter))
}

/// For the edit mode allow to loop on the match
pub fn edit(
    mut img: DynamicImage,
    pb: &mut ProgressBarCustom,
    show: bool,
) -> Result<DynamicImage, anyhow::Error> {
    let msg = if show {
        "Action to do:\
    \n 'cancel': leave without save\
    \n 'exit': leave and save\
    \n 'help': show all the action\
    \n '*': action you want to do"
    } else {
        "enter your action"
    };
    let input = take_input(msg).to_uppercase();
    match input.as_str() {
        "CANCEL" => return Err(anyhow!("cancel action")),
        "EXIT" => return Ok(img),
        "HELP" => {
            println!("see here for all actions: 'https://github.com/Kallu-A/image-utility#possible-actions'");
            img = edit(img, pb, true)?;
        }
        _ => {
            match input.as_str() {
                "BLUR" => img = action::blur(img, pb)?,
                "BRIGHTEN" => img = action::brighten(img, pb)?,
                "CONTRAST" => img = action::constrast(img, pb)?,
                "FILTER" => img = action::filter3x3(img, pb)?,
                "FLIPH" => img = action::fliph(img, pb)?,
                "FLIPV" => img = action::flipv(img, pb)?,
                "GRAYSCALE" => img = action::grayscale(img, pb)?,
                "HISTOGRAM" => img = action::histogram(img, pb)?,
                "INVERT" => img = action::invert(img, pb)?,
                "RESIZE" => img = action::resize(img, pb)?,
                "RESIZERATIO" => img = action::resize_ratio(img, pb)?,
                "ROTATE90" => img = action::rotate90(img, pb)?,
                "ROTATE180" => img = action::rotate180(img, pb)?,
                "ROTATE270" => img = action::rotate270(img, pb)?,
                _ => println!("Invalid action. please retry"),
            };
            pb.done();
            let mut pb = ProgressBarCustom::create();
            img = edit(img, pb.borrow_mut(), false)?
        }
    }
    Ok(img)
}
