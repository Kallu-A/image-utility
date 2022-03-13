use std::cmp::min;
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

/// Do the histogram with a medium value of the rgb
pub fn histogram_gray(img: DynamicImage) -> Result<DynamicImage, anyhow::Error> {
    let mut vec = vec![0 as i32; 256];

    let mut min_value = 255 as i32;
    let mut pixel;
    let mut index;
    for x in 0..img.width() {
        for y in 0..img.height() {
            pixel = img.get_pixel(x,y);
            if pixel.0[3] != 0 {
                index = ((pixel.0[0] as u32 + pixel.0[1] as u32 + pixel.0[2] as u32 ) / 3 as u32) as u8;
                min_value = min(min_value, index.into());
                vec[ index as usize ] += 1;
            }
        }
    }

    let mut g_max = 0;
    for i in 0..vec.len() {
        if vec[i] > g_max {
            g_max = vec[i];
        }
    }
    let g_max = g_max - min_value;
    let max = 200;
    let mut new_val;
    let mut color;
    let mut res = DynamicImage::new_rgb8(514, 202);

    //line of white on top
    for x in 0..res.width() {
        res.put_pixel(x, 0, Rgba([255, 255, 255, 1]));
    }

    for y in 1..res.height() {
        res.put_pixel(0, y, Rgba([255, 255, 255, 1]));
        for index in 0..vec.len() {
            new_val = max * (vec[index] - min_value) / (g_max - min_value);
            color = if (new_val + 1) as u32 + (y-1) >= max as u32 { 20 } else { 255 };

            res.put_pixel((index as u32) * 2, y, Rgba([color, color, color, 1]) );
            res.put_pixel((index  as u32) * 2 + 1, y,Rgba([255, 255, 255, 1]) );
        }

        res.put_pixel(513, y, Rgba([255, 255, 255, 1]));
    }

    Ok(res)
}