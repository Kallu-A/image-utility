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

/// Do the histogramm and take in account the RGB value
pub fn histogram_rgb(img: DynamicImage) -> Result<DynamicImage, anyhow::Error> {
    let mut vec_r = vec![0 as i32; 256];
    let mut vec_g = vec![0 as i32; 256];
    let mut vec_b = vec![0 as i32; 256];

    let mut min_value_r = 255 as u8;
    let mut min_value_g = 255 as u8;
    let mut min_value_b = 255 as u8;
    let mut pixel;
    for x in 0..img.width() {
        for y in 0..img.height() {
            pixel = img.get_pixel(x,y);
            if pixel.0[3] != 0 {
                min_value_r = min(min_value_r, pixel.0[0]);
                min_value_g = min(min_value_g, pixel.0[1]);
                min_value_b = min(min_value_b, pixel.0[2]);
                vec_r[pixel.0[0] as usize] += 1;
                vec_g[pixel.0[1] as usize] += 1;
                vec_b[pixel.0[2] as usize] += 1;
            }
        }
    }

    let mut g_max_r = 0;
    let mut g_max_g = 0;
    let mut g_max_b = 0;
    for i in 0..vec_r.len() {
        if vec_r[i] > g_max_r { g_max_r = vec_r[i]; }
        if vec_g[i] > g_max_g { g_max_g = vec_g[i]; }
        if vec_b[i] > g_max_b { g_max_b = vec_b[i]; }
    }
    let g_max_r = g_max_r - min_value_r as i32;
    let g_max_g = g_max_g - min_value_g as i32;
    let g_max_b = g_max_b - min_value_b as i32;
    let max = 200;
    let mut new_val_r;
    let mut new_val_g;
    let mut new_val_b;
    let mut color;
    let mut res = DynamicImage::new_rgb8(1024, 202);

    //line of white on top
    for x in 0..res.width() {
        res.put_pixel(x, 0, Rgba([255, 255, 255, 1]));
    }

    for y in 1..res.height() {
        res.put_pixel(0, y, Rgba([255, 255, 255, 1]));
        for index in 0..vec_r.len() {
            new_val_r = max * (vec_r[index] - min_value_r as i32) / (g_max_r - min_value_r as i32);
            new_val_g = max * (vec_g[index] - min_value_g as i32) / (g_max_g - min_value_g as i32);
            new_val_b = max * (vec_b[index] - min_value_b as i32) / (g_max_b - min_value_b as i32);

            // red
            color = if (new_val_r + 1) as u32 + (y-1) >= max as u32 { 0 } else { 255 };
            res.put_pixel((index as u32) * 4, y, Rgba([255, color, color, 1]) );

            // green
            color = if (new_val_g + 1) as u32 + (y-1) >= max as u32 { 0 } else { 255 };
            res.put_pixel((index as u32) * 4 + 1, y, Rgba([color, 255, color, 1]) );

            // blue
            color = if (new_val_b + 1) as u32 + (y-1) >= max as u32 { 0 } else { 255 };
            res.put_pixel((index as u32) * 4 + 2, y, Rgba([color, color, 255, 1]) );

            // space
            res.put_pixel((index  as u32) * 4 + 3, y,Rgba([255, 255, 255, 1]) );
        }

        res.put_pixel(1023, y, Rgba([255, 255, 255, 1]));
    }

    Ok(res)
}