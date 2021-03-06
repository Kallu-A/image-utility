use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use std::cmp::min;

/// Do the histogram with a average value of the rgb
pub fn histogram_gray(img: DynamicImage) -> Result<DynamicImage, anyhow::Error> {
    let mut vec = vec![0_u32; 256];

    let mut min_value = 255_u8;
    let mut index;
    for pixel in img.pixels() {
        if pixel.2[3] != 0 {
            index = ((pixel.2[0] as u32 + pixel.2[1] as u32 + pixel.2[2] as u32) / 3_u32) as u8;
            min_value = min(min_value, index);
            vec[index as usize] += 1;
        }
    }

    let g_max = vec.iter().max().unwrap() - min_value as u32;
    let max = 200i32;
    let mut new_val: i32;
    let mut color;
    let mut res = DynamicImage::new_rgb8(514, 202);

    //margin top
    for x in 0..res.width() {
        res.put_pixel(x, 0, Rgba([255, 255, 255, 1]));
    }

    // fill the body
    for y in 1..res.height() {
        res.put_pixel(0, y, Rgba([255, 255, 255, 1])); // margin left

        for (index, item) in vec.iter().enumerate() {
            new_val = (max * (*item as i32 - min_value as i32) / (g_max as i32 - min_value as i32))
                as i32;
            color = if (new_val + 1) + y as i32 >= max {
                20
            } else {
                255
            };

            res.put_pixel((index as u32) * 2 + 1, y, Rgba([color, color, color, 1]));
            res.put_pixel((index as u32) * 2 + 2, y, Rgba([255, 255, 255, 1]));
        }

        // margin right
        res.put_pixel(513, y, Rgba([255, 255, 255, 1]));
    }

    Ok(res)
}

/// Do the histogramm and take in account the RGB value
pub fn histogram_rgb(img: DynamicImage) -> Result<DynamicImage, anyhow::Error> {
    let mut vec_r = vec![0_u32; 256];
    let mut vec_g = vec![0_u32; 256];
    let mut vec_b = vec![0_u32; 256];

    let mut min_value_r = 255_u8;
    let mut min_value_g = 255_u8;
    let mut min_value_b = 255_u8;

    for pixel in img.pixels() {
        if pixel.2[3] != 0 {
            min_value_r = min(min_value_r, pixel.2[0]);
            min_value_g = min(min_value_g, pixel.2[1]);
            min_value_b = min(min_value_b, pixel.2[2]);
            vec_r[pixel.2[0] as usize] += 1;
            vec_g[pixel.2[1] as usize] += 1;
            vec_b[pixel.2[2] as usize] += 1;
        }
    }

    let mut g_max_r = 0_u32;
    let mut g_max_g = 0_u32;
    let mut g_max_b = 0_u32;
    // manually look for the max for performance issue
    for i in 0..vec_r.len() {
        if vec_r[i] > g_max_r {
            g_max_r = vec_r[i];
        }
        if vec_g[i] > g_max_g {
            g_max_g = vec_g[i];
        }
        if vec_b[i] > g_max_b {
            g_max_b = vec_b[i];
        }
    }
    let g_max_r = g_max_r - min_value_r as u32;
    let g_max_g = g_max_g - min_value_g as u32;
    let g_max_b = g_max_b - min_value_b as u32;
    let max = 200;
    let mut new_val_r: i32;
    let mut new_val_g: i32;
    let mut new_val_b: i32;
    let mut color;
    let mut res = DynamicImage::new_rgb8(1024, 202);

    //margin top
    for x in 0..res.width() {
        res.put_pixel(x, 0, Rgba([255, 255, 255, 1]));
    }

    for y in 1..res.height() {
        // margin left
        res.put_pixel(0, y, Rgba([255, 255, 255, 1]));
        for index in 0..vec_r.len() {
            new_val_r = max * (vec_r[index] as i32 - min_value_r as i32)
                / (g_max_r as i32 - min_value_r as i32);
            new_val_g = max * (vec_g[index] as i32 - min_value_g as i32)
                / (g_max_g as i32 - min_value_g as i32);
            new_val_b = max * (vec_b[index] as i32 - min_value_b as i32)
                / (g_max_b as i32 - min_value_b as i32);

            // red
            color = if (new_val_r + 1) as i32 + y as i32 >= max as i32 {
                0
            } else {
                255
            };
            res.put_pixel((index as u32) * 4, y, Rgba([255, color, color, 1]));

            // green
            color = if (new_val_g + 1) as i32 + y as i32 >= max as i32 {
                0
            } else {
                255
            };
            res.put_pixel((index as u32) * 4 + 1, y, Rgba([color, 255, color, 1]));

            // blue
            color = if (new_val_b + 1) as i32 + y as i32 >= max as i32 {
                0
            } else {
                255
            };
            res.put_pixel((index as u32) * 4 + 2, y, Rgba([color, color, 255, 1]));

            // space
            res.put_pixel((index as u32) * 4 + 3, y, Rgba([255, 255, 255, 1]));
        }

        res.put_pixel(1023, y, Rgba([255, 255, 255, 1]));
    }

    Ok(res)
}
