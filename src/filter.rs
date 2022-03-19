use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use std::borrow::BorrowMut;

pub fn sobel(img: DynamicImage) -> DynamicImage {
    let mut img_sobel_x = img.filter3x3(&[
        -1_f32, 0_f32, 1_f32, -2_f32, 0_f32, 2_f32, -1_f32, 0_f32, 1_f32,
    ]);
    let img_sobel_y = img.filter3x3(&[
        -1_f32, -2_f32, -1_f32, 0_f32, 0_f32, 0_f32, 1_f32, 2_f32, 1_f32,
    ]);
    for i in 0..img_sobel_x.width() {
        for j in 0..img_sobel_x.height() {
            calcul(
                i,
                j,
                img_sobel_x.get_pixel(i, j),
                img_sobel_y.get_pixel(i, j),
                img_sobel_x.borrow_mut(),
            );
        }
    }

    img_sobel_x
}

fn calcul(i: u32, j: u32, rgbx: Rgba<u8>, rgby: Rgba<u8>, rep: &mut DynamicImage) {
    use num_integer::Roots;

    let mut red = rgbx.0[0] as u32 * rgbx.0[0] as u32 + rgby.0[0] as u32 * rgby.0[0] as u32;
    let mut green = rgbx.0[1] as u32 * rgbx.0[1] as u32 + rgby.0[1] as u32 * rgby.0[1] as u32;
    let mut blue = rgbx.0[2] as u32 * rgbx.0[2] as u32 + rgby.0[2] as u32 * rgby.0[2] as u32;
    red = red.sqrt();
    green = green.sqrt();
    blue = blue.sqrt();
    let red = if red > 255 { 255 } else { red };
    let green = if green > 255 { 255 } else { green };
    let blue = if blue > 255 { 255 } else { red };
    let color = ((red + green + blue) / 3) as u8;
    rep.put_pixel(
        i,
        j,
        Rgba {
            0: [color, color, color, rgbx.0[3]],
        },
    )
}
