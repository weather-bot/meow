use weather_info::WeatherInfo;
use std::path::Path;
use imageproc::drawing::draw_text_mut;
use image;
use image::{Rgba, ImageBuffer, imageops};
use rusttype::{FontCollection, Scale, point, PositionedGlyph};

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

    let glyphs: Vec<PositionedGlyph> = font.layout(text, scale, offset)
        .collect();

    let mut width = 0;
    for g in glyphs {
        if let Some(bb) = g.pixel_bounding_box() {
            width += bb.width();
        }
    }
    width as u32
}

//    The Image Outline
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
        ImageBuffer::from_fn(IMG_HEIGHT, TITLE_HEIGHT, |_, _| background_color);
    imageops::overlay(&mut origin_img, &bg_img_title, 0, 0);


    // Weather infomation background
    let bg_img_info = ImageBuffer::from_fn(
        CORNER_INFO_WIDTH,
        CORNER_INFO_HEIGHT,
        |_, _| background_color,
    );
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
