use image;
use image::{imageops, ImageBuffer, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::{point, FontCollection, PositionedGlyph, Scale};
use std::path::Path;
use weather_info::WeatherInfo;

fn get_text_width(text: &str, height: u32) -> u32 {
    let font =
        Vec::from(include_bytes!("../font/NotoSansCJKtc-Medium.ttf") as &[u8]);
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
            if bb.min.x < min { min = bb.min.x;}
            if bb.max.x > max { max = bb.max.x;}
        }
    }
    (max - min) as u32
}

//    The Image Outline for Corner Mode
//
//    800 x 800 input kitty image
//    +---------------------------+
//    | 800 x 130    Title        |
//    |                           |
//    +---------------------------+
//    |                300 x 350  |
//    |       Kitty    +----------+
//    |       Image    | time     |
//    |                | temp     |
//    |                | humd     |
//    |                | overview |
//    +----------------+----------+

const IMG_WIDTH: u32 = 800;
const IMG_HEIGHT: u32 = 800;
const TITLE_HEIGHT: u32 = 130;
const CORNER_INFO_WIDTH: u32 = 300;
const CORNER_INFO_HEIGHT: u32 = 350;

pub fn draw_corner(
    image_path: &str,
    weather_info: &WeatherInfo,
    output_path: &str,
) {
    let background_color = Rgba([146u8, 146u8, 146u8, 100u8]);
    let text_color = Rgba([255u8, 255u8, 255u8, 255u8]);

    // Weather Info Rect
    let pos_x = IMG_WIDTH - CORNER_INFO_WIDTH;
    let mut pos_y = IMG_HEIGHT - CORNER_INFO_HEIGHT;

    // Open image and crop to the correct size
    let mut origin_img = image::open(image_path).unwrap();
    let mut origin_img =
        imageops::crop(&mut origin_img, 0, 0, IMG_WIDTH, IMG_HEIGHT).to_image();

    // Big title background
    let bg_img_title =
        ImageBuffer::from_fn(IMG_WIDTH, TITLE_HEIGHT, |_, _| background_color);
    imageops::overlay(&mut origin_img, &bg_img_title, 0, 0);

    // Weather infomation background
    let bg_img_info =
        ImageBuffer::from_fn(CORNER_INFO_WIDTH, CORNER_INFO_HEIGHT, |_, _| {
            background_color
        });
    imageops::overlay(&mut origin_img, &bg_img_info, pos_x, pos_y);

    // font type
    let font =
        Vec::from(include_bytes!("../font/NotoSansCJKtc-Medium.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font)
        .unwrap()
        .into_font()
        .unwrap();

    // weather information text
    let humd_str = format!("濕度:{}%", weather_info.humd);
    let temp_str = format!("溫度:{}°C", weather_info.temp);
    let overview_str = format!("天氣:{}", weather_info.overview);

    // title
    let text_height = TITLE_HEIGHT - 20;
    let width = get_text_width(&weather_info.title, text_height);
    draw_text_mut(
        &mut origin_img,
        text_color,
        ((IMG_WIDTH - width) as f32 / 4.0).round() as u32,
        0,
        Scale::uniform(text_height as f32),
        &font,
        &weather_info.title,
    );

    // time
    let pos_x = pos_x + 20;
    let text_height = 80;
    draw_text_mut(
        &mut origin_img,
        text_color,
        pos_x,
        pos_y,
        Scale::uniform(text_height as f32),
        &font,
        &weather_info.time,
    );

    // temperature
    pos_y += text_height + 5;
    draw_text_mut(
        &mut origin_img,
        text_color,
        pos_x,
        pos_y,
        Scale::uniform(text_height as f32),
        &font,
        &temp_str,
    );

    // humidity
    pos_y += text_height + 5;
    draw_text_mut(
        &mut origin_img,
        text_color,
        pos_x,
        pos_y,
        Scale::uniform(text_height as f32),
        &font,
        &humd_str,
    );

    // overview
    pos_y += text_height + 5;
    draw_text_mut(
        &mut origin_img,
        text_color,
        pos_x,
        pos_y,
        Scale::uniform(text_height as f32),
        &font,
        &overview_str,
    );

    let _ = origin_img.save(Path::new(output_path)).unwrap();
}

// Image outline for Buttom-Mode
//
// 800 x 800 input kitty image
// +-------------------------------------------+
// |   800 x 130                               |
// |   Title                                   |
// |                                           |
// +-------------------------------------------+
// |                                           |
// |                                           |
// |   Kitty Image                             |
// |                                           |
// |                                           |
// | 800 x 130 Info, 200 each block            |
// +------------+-----------+------------------+
// |  Taipei    | Rainny    |         |        |
// |            |           |  87%    | 25°C   |
// |  Tomorrow  | Very Hot  |         |        |
// |  15:00     |           |         |        |
// +------------+-----------+---------+--------+

const BOTTOM_INFO_HEIGHT: u32 = 130;

pub fn draw_bottom(
    image_path: &str,
    weather_info: &WeatherInfo,
    output_path: &str,
) {
    let background_color = Rgba([94u8, 94u8, 94u8, 100u8]); // More black

    let text_color = Rgba([255u8, 255u8, 255u8, 255u8]);

    // font type
    let font =
        Vec::from(include_bytes!("../font/NotoSansCJKtc-Medium.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font)
        .unwrap()
        .into_font()
        .unwrap();

    // Open image and crop to the correct size
    let mut origin_img = image::open(image_path).unwrap();
    let mut origin_img =
        imageops::crop(&mut origin_img, 0, 0, IMG_WIDTH, IMG_HEIGHT).to_image();

    // Big title background
    let bg_img_title =
        ImageBuffer::from_fn(IMG_WIDTH, TITLE_HEIGHT, |_, _| background_color);
    imageops::overlay(&mut origin_img, &bg_img_title, 0, 0);

    // Weather info background at bottom
    let bk_img_info =
        ImageBuffer::from_fn(IMG_WIDTH, BOTTOM_INFO_HEIGHT, |_, _| {
            background_color
        });
    imageops::overlay(
        &mut origin_img,
        &bk_img_info,
        0,
        IMG_HEIGHT - BOTTOM_INFO_HEIGHT,
    );

    // title
    let text_height = TITLE_HEIGHT - 20;
    let width = get_text_width(&weather_info.title, text_height);
    draw_text_mut(
        &mut origin_img,
        text_color,
        ((IMG_WIDTH - width) as f32 / 4.0).round() as u32,
        0,
        Scale::uniform(text_height as f32),
        &font,
        &weather_info.title,
    );

    // Location
    let bottom_pos_x = 10;
    let bottom_pos_loc_y = IMG_HEIGHT - BOTTOM_INFO_HEIGHT;
    let location_font_size = 80;
    draw_text_mut(
        &mut origin_img,
        text_color,
        bottom_pos_x,
        bottom_pos_loc_y,
        Scale::uniform(location_font_size as f32),
        &font,
        &weather_info.location,
    );

    // Time
    let bottom_pos_time_y = bottom_pos_loc_y + location_font_size;
    let time_font_size = 50;
    draw_text_mut(
        &mut origin_img,
        text_color,
        bottom_pos_x,
        bottom_pos_time_y,
        Scale::uniform(time_font_size as f32),
        &font,
        &weather_info.time,
    );

    // Overview 1
    let bottom_pos_x = bottom_pos_x + 200;
    let bottom_pos_ov1_y = IMG_HEIGHT - BOTTOM_INFO_HEIGHT + 10;
    let ov1_font_size = 60;
    draw_text_mut(
        &mut origin_img,
        text_color,
        bottom_pos_x,
        bottom_pos_ov1_y,
        Scale::uniform(ov1_font_size as f32),
        &font,
        &weather_info.overview,
    );

    // Overview 2
    let bottom_pos_ov2_y = bottom_pos_ov1_y + ov1_font_size;
    let ov2_font_size = 60;
    draw_text_mut(
        &mut origin_img,
        text_color,
        bottom_pos_x,
        bottom_pos_ov2_y,
        Scale::uniform(ov2_font_size as f32),
        &font,
        &weather_info.overview2,
    );

    // Humidity
    let bottom_pos_x = bottom_pos_x + 200;
    let bottom_pos_humd_y = IMG_HEIGHT - BOTTOM_INFO_HEIGHT + 30;
    let water_drop_icon = image::open("img/water_drop.png").unwrap().to_rgba();
    imageops::overlay(
        &mut origin_img,
        &water_drop_icon,
        bottom_pos_x,
        bottom_pos_humd_y,
    );

    let bottom_pos_humd_x = bottom_pos_x + 48 + 10; // 48 is icon width
    let humd_font_size = 80.0;
    let humd_str = format!("{}%", &weather_info.humd);
    draw_text_mut(
        &mut origin_img,
        text_color,
        bottom_pos_humd_x,
        bottom_pos_humd_y,
        Scale::uniform(humd_font_size),
        &font,
        &humd_str,
    );

    // Temperature
    let bottom_pos_x = bottom_pos_x + 200;
    let bottom_pos_temp_y = IMG_HEIGHT - BOTTOM_INFO_HEIGHT + 30;
    let thermometer_icon =
        image::open("img/thermometer.png").unwrap().to_rgba();
    imageops::overlay(
        &mut origin_img,
        &thermometer_icon,
        bottom_pos_x,
        bottom_pos_temp_y,
    );

    let bottom_pos_x = bottom_pos_x + 40 + 10; // 40 is icon width
    let temp_font_size = 80.0;
    let temp_str = format!("{}℃", &weather_info.temp);
    draw_text_mut(
        &mut origin_img,
        text_color,
        bottom_pos_x,
        bottom_pos_temp_y,
        Scale::uniform(temp_font_size),
        &font,
        &temp_str,
    );

    let _ = origin_img.save(Path::new(output_path)).unwrap();
}
