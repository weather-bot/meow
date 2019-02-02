use image;
use image::{imageops, ImageBuffer, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::{point, FontCollection, PositionedGlyph, Scale};
use std::fmt;
use std::path::Path;
use weather_info::WeatherInfo;

pub enum DrawError {
    WidthTooLong(u32, u32, String),
    ValueTooHigh(f64, f64, String),
}

impl fmt::Display for DrawError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DrawError::WidthTooLong(ref real, ref limit, ref case) => write!(f,
            "The width of the `{}` is {}. The max is only {}. Please consider a shorter `{}`.", case, real, limit, case),
            DrawError::ValueTooHigh(ref real, ref limit, ref case) => write!(f,
            "The value of `{}` is {}. The max is only {}. Please give a valid `{}`.", case, real, limit, case),
        }
    }
}

fn get_text_width(text: &str, height: u32) -> u32 {
    let font =
        Vec::from(include_bytes!("../font/NotoSansCJKtc-Light.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font)
        .unwrap()
        .into_font()
        .unwrap();

    let scale = Scale::uniform(height as f32);
    let v_metrics = font.v_metrics(scale);
    let offset = point(0.0, v_metrics.ascent);

    let glyphs: Vec<PositionedGlyph> =
        font.layout(text, scale, offset).collect();

    let mut min = 10000;
    let mut max = 0;
    for g in glyphs {
        if let Some(bb) = g.pixel_bounding_box() {
            if bb.min.x < min {
                min = bb.min.x;
            }
            if bb.max.x > max {
                max = bb.max.x;
            }
        }
    }
    (max - min) as u32
}

// Image outline for Bottom-Mode
//
// 800 x 800 input kitty image
// +-------------------------------------------+
// |   800 x 800                               |
// |                                           |
// |                                           |
// |                                           |
// |                                           |
// |                                           |
// |------------------------+                  |
// |   1/13 Taipei          |                  |
// |   overcast to tomorrow |                  |
// |------------------------+                  |
// |                                           |
// |                                           |
// |                           HOT [20°C] [60%]|
// +-------------------------------------------+

const IMG_WIDTH: u32 = 800;
const IMG_HEIGHT: u32 = 800;

pub fn draw_light(
    image_path: &str,
    weather_info: &WeatherInfo,
    output_path: &str,
) -> Result<(), DrawError> {
    // font type
    let font =
        Vec::from(include_bytes!("../font/NotoSansCJKtc-Light.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font)
        .unwrap()
        .into_font()
        .unwrap();

    // Open image and crop to the correct size
    let mut origin_img = image::open(image_path).unwrap();
    let mut origin_img =
        imageops::crop(&mut origin_img, 0, 0, IMG_WIDTH, IMG_HEIGHT).to_image();

    // all parameters
    let first_background_color = Rgba([20u8, 20u8, 20u8, 200u8]);
    let first_text_color = Rgba([255u8, 255u8, 255u8, 255u8]);

    let second_background_color = Rgba([255u8, 255u8, 255u8, 200u8]);
    let second_text_color = Rgba([0u8, 0u8, 0u8, 255u8]);

    let first_block_height = 160;
    let first_block_pos_y = 500;
    let first_padding_x = 20;

    let first_block_font_size = 70;

    let title_width =
        get_text_width(&weather_info.title, first_block_font_size);
    let title_pos_x = first_padding_x;
    let second_line_end_pos_x = title_pos_x + title_width;

    let time_width = get_text_width(&weather_info.time, first_block_font_size);
    let loc_width =
        get_text_width(&weather_info.location, first_block_font_size);

    let time_pos_x = first_padding_x;
    let loc_pos_x = time_pos_x + time_width + first_padding_x;
    let first_line_end_pos_x = loc_pos_x + loc_width;

    let second_block_height = 75;
    let second_block_font_size = 65;
    let second_line = format!(
        "{}｜{}｜{}℃｜溼{}%",
        &weather_info.overview,
        &weather_info.overview2,
        &weather_info.temp,
        &weather_info.humd
    );
    let second_padding_x = 10;
    let second_width =
        get_text_width(&second_line, second_block_font_size) + second_padding_x;
    let second_block_length = second_width + second_padding_x;
    let block_pos_x = IMG_WIDTH - second_width - second_padding_x;
    let second_pos_x = IMG_WIDTH - second_width;
    let second_pos_y = IMG_HEIGHT - second_block_height;

    // first block background
    let length = if first_line_end_pos_x > second_line_end_pos_x {
        first_line_end_pos_x
    } else {
        second_line_end_pos_x
    } + first_padding_x;
    let bk_img_info =
        ImageBuffer::from_fn(length, first_block_height, |_, _| {
            first_background_color
        });
    imageops::overlay(&mut origin_img, &bk_img_info, 0, first_block_pos_y);

    // second block background
    let bk_img_info = ImageBuffer::from_fn(
        second_block_length,
        second_block_height,
        |_, _| second_background_color,
    );
    imageops::overlay(&mut origin_img, &bk_img_info, block_pos_x, second_pos_y);

    // title
    draw_text_mut(
        &mut origin_img,
        first_text_color,
        title_pos_x,
        first_block_pos_y + first_block_height / 2,
        Scale::uniform(first_block_font_size as f32),
        &font,
        &weather_info.title,
    );

    // Time
    draw_text_mut(
        &mut origin_img,
        first_text_color,
        time_pos_x,
        first_block_pos_y,
        Scale::uniform(first_block_font_size as f32),
        &font,
        &weather_info.time,
    );

    // Location
    draw_text_mut(
        &mut origin_img,
        first_text_color,
        loc_pos_x,
        first_block_pos_y,
        Scale::uniform(first_block_font_size as f32),
        &font,
        &weather_info.location,
    );

    // second line
    draw_text_mut(
        &mut origin_img,
        second_text_color,
        second_pos_x,
        second_pos_y,
        Scale::uniform(second_block_font_size as f32),
        &font,
        &second_line,
    );

    let _ = origin_img.save(Path::new(output_path)).unwrap();
    Ok(())
}

fn check_width(real: u32, limit: u32, case: &str) -> Result<(), DrawError> {
    if real > limit {
        return Err(DrawError::WidthTooLong(real, limit, case.to_string()));
    }
    Ok(())
}

fn check_value(real: f64, case: &str) -> Result<(), DrawError> {
    let limit = 100.0;
    if real > limit {
        return Err(DrawError::ValueTooHigh(real, limit, case.to_string()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_value_failed() {
        assert!(check_value(120.0, "temp").is_err());
    }

    #[test]
    fn test_check_value_work() {
        assert!(check_value(100.0, "temp").is_ok());
    }

    #[test]
    fn test_check_width_failed() {
        let width_limit = 800;
        let width =
            get_text_width("This string is very very very very long!", 100);
        assert!(check_width(width, width_limit, "title").is_err());
    }

    #[test]
    fn test_check_width_work() {
        let width_limit = 800;
        let width = get_text_width("This string is short!", 100);
        assert!(check_width(width, width_limit, "title").is_ok());
    }
}
