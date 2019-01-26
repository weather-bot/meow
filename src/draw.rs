use image;
use image::{imageops, ImageBuffer, Rgba};
use imageproc::drawing::draw_text_mut;
use rand::Rng;
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

//    The Image Outline for Corner Mode
//
//    800 x 800 input kitty image
//    +---------------------------+
//    | 800 x 130    Title        |
//    |                           |
//    +---------------------------+
//    |    Kitty Image            |
//    |                200x370    |
//    |200x130         +----------+
//    +------+         | temp     |
//    | loc  |         | humd     |
//    | time |         | overview |
//    +----------------+----------+

const IMG_WIDTH: u32 = 800;
const IMG_HEIGHT: u32 = 800;
const TITLE_HEIGHT: u32 = 130;
const INFO_BLOCK_WIDTH: u32 = 200;
const INFO_BLOCK_HEIGHT: u32 = 120;
const LEFT_CORNER_HEIGHT: u32 = 130;
const RIGHT_CORNER_HEIGHT: u32 = INFO_BLOCK_HEIGHT * 3 + 10;

pub fn draw_corner(
    image_path: &str,
    weather_info: &WeatherInfo,
    output_path: &str,
) -> Result<(), DrawError> {
    let background_color = Rgba([94u8, 94u8, 94u8, 100u8]);
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

    // Left corner background
    let bk_img_info_left =
        ImageBuffer::from_fn(INFO_BLOCK_WIDTH, LEFT_CORNER_HEIGHT, |_, _| {
            background_color
        });
    imageops::overlay(
        &mut origin_img,
        &bk_img_info_left,
        0,
        IMG_HEIGHT - BOTTOM_INFO_HEIGHT,
    );

    // Right corner background
    let bk_img_info_right =
        ImageBuffer::from_fn(INFO_BLOCK_WIDTH, RIGHT_CORNER_HEIGHT, |_, _| {
            background_color
        });
    imageops::overlay(
        &mut origin_img,
        &bk_img_info_right,
        IMG_WIDTH - INFO_BLOCK_WIDTH,
        IMG_HEIGHT - RIGHT_CORNER_HEIGHT,
    );

    // title
    let text_height = TITLE_HEIGHT - 20;
    let width = get_text_width(&weather_info.title, text_height);
    // title is short enough in a line
    if width < IMG_WIDTH {
        draw_text_mut(
            &mut origin_img,
            text_color,
            ((IMG_WIDTH - width) as f32 / 2.0).round() as u32,
            0,
            Scale::uniform(text_height as f32),
            &font,
            &weather_info.title,
        );
    } else {
        // Double the big title background
        imageops::overlay(&mut origin_img, &bg_img_title, 0, TITLE_HEIGHT);
        // print title in two lines
        let (first, last) = &weather_info.title.split_at(3 * 10);
        let width = get_text_width(first, text_height);
        draw_text_mut(
            &mut origin_img,
            text_color,
            ((IMG_WIDTH - width) as f32 / 2.0).round() as u32,
            0,
            Scale::uniform(text_height as f32),
            &font,
            first,
        );
        draw_text_mut(
            &mut origin_img,
            text_color,
            ((IMG_WIDTH - width) as f32 / 2.0).round() as u32,
            TITLE_HEIGHT,
            Scale::uniform(text_height as f32),
            &font,
            last,
        );
    }

    // Location
    let pos_x = 10;
    let pos_loc_y = IMG_HEIGHT - BOTTOM_INFO_HEIGHT;
    let location_font_size = 80;
    let width = get_text_width(&weather_info.location, location_font_size);
    check_width(width, INFO_BLOCK_WIDTH, "location")?;
    draw_text_mut(
        &mut origin_img,
        text_color,
        pos_x,
        pos_loc_y,
        Scale::uniform(location_font_size as f32),
        &font,
        &weather_info.location,
    );

    // Time
    let pos_time_y = pos_loc_y + location_font_size;
    let time_font_size = 50;
    let width = get_text_width(&weather_info.time, time_font_size);
    check_width(width, INFO_BLOCK_WIDTH, "time")?;
    draw_text_mut(
        &mut origin_img,
        text_color,
        pos_x,
        pos_time_y,
        Scale::uniform(time_font_size as f32),
        &font,
        &weather_info.time,
    );

    // Humidity
    let pos_x = IMG_WIDTH - INFO_BLOCK_WIDTH + 10;
    let pos_y = IMG_HEIGHT - RIGHT_CORNER_HEIGHT + 10;
    let water_drop_icon =
        image::load_from_memory(include_bytes!("../img/water_drop.png"))
            .unwrap()
            .to_rgba();
    imageops::overlay(&mut origin_img, &water_drop_icon, pos_x, pos_y);

    let pos_humd_x = pos_x + 48 + 10; // 48 is icon width
    let humd_font_size = 80.0;
    let humd_str = format!("{}%", &weather_info.humd);
    check_value(weather_info.humd, "humidity")?;
    draw_text_mut(
        &mut origin_img,
        text_color,
        pos_humd_x,
        pos_y,
        Scale::uniform(humd_font_size),
        &font,
        &humd_str,
    );

    // Temperature
    let pos_y = pos_y + INFO_BLOCK_HEIGHT;
    let thermometer_icon =
        image::load_from_memory(include_bytes!("../img/thermometer.png"))
            .unwrap()
            .to_rgba();
    imageops::overlay(&mut origin_img, &thermometer_icon, pos_x, pos_y);

    let pos_tmp_x = pos_x + 40 + 10; // 40 is icon width
    let temp_font_size = 80.0;
    let temp_str = format!("{}℃", &weather_info.temp);
    check_value(weather_info.temp, "temperature")?;
    draw_text_mut(
        &mut origin_img,
        text_color,
        pos_tmp_x,
        pos_y,
        Scale::uniform(temp_font_size),
        &font,
        &temp_str,
    );

    // Overview 1
    let pos_y = pos_y + INFO_BLOCK_HEIGHT;
    let ov1_font_size = 60;
    let width = get_text_width(&weather_info.overview, ov1_font_size);
    check_width(width, INFO_BLOCK_WIDTH, "overview")?;
    draw_text_mut(
        &mut origin_img,
        text_color,
        pos_x,
        pos_y,
        Scale::uniform(ov1_font_size as f32),
        &font,
        &weather_info.overview,
    );

    // Overview 2
    let pos_y = pos_y + ov1_font_size;
    let ov2_font_size = 60;
    let width = get_text_width(&weather_info.overview2, ov2_font_size);
    check_width(width, INFO_BLOCK_WIDTH, "overview2")?;
    draw_text_mut(
        &mut origin_img,
        text_color,
        pos_x,
        pos_y,
        Scale::uniform(ov2_font_size as f32),
        &font,
        &weather_info.overview2,
    );

    let _ = origin_img.save(Path::new(output_path)).unwrap();
    Ok(())
}

// Image outline for Bottom-Mode
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
// |  Taipei    | Rainy    |         |        |
// |            |           |  87%    | 25°C   |
// |  Tomorrow  | Very Hot  |         |        |
// |  15:00     |           |         |        |
// +------------+-----------+---------+--------+

const BOTTOM_INFO_HEIGHT: u32 = 130;

pub fn draw_bottom(
    image_path: &str,
    weather_info: &WeatherInfo,
    output_path: &str,
) -> Result<(), DrawError> {
    let background_color = Rgba([94u8, 94u8, 94u8, 100u8]);
    let text_color = Rgba([255u8, 255u8, 255u8, 255u8]);
    let info_block_width: u32 = 200;

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
    // title is short enough in a line
    if width < IMG_WIDTH {
        draw_text_mut(
            &mut origin_img,
            text_color,
            ((IMG_WIDTH - width) as f32 / 2.0).round() as u32,
            0,
            Scale::uniform(text_height as f32),
            &font,
            &weather_info.title,
        );
    } else {
        // Double the big title background
        imageops::overlay(&mut origin_img, &bg_img_title, 0, TITLE_HEIGHT);
        // print title in two lines
        let (first, last) = &weather_info.title.split_at(3 * 10);
        let width = get_text_width(first, text_height);
        draw_text_mut(
            &mut origin_img,
            text_color,
            ((IMG_WIDTH - width) as f32 / 2.0).round() as u32,
            0,
            Scale::uniform(text_height as f32),
            &font,
            first,
        );
        draw_text_mut(
            &mut origin_img,
            text_color,
            ((IMG_WIDTH - width) as f32 / 2.0).round() as u32,
            TITLE_HEIGHT,
            Scale::uniform(text_height as f32),
            &font,
            last,
        );
    }

    // Location
    let bottom_pos_x = 10;
    let bottom_pos_loc_y = IMG_HEIGHT - BOTTOM_INFO_HEIGHT;
    let location_font_size = 80;
    let width = get_text_width(&weather_info.location, location_font_size);
    check_width(width, info_block_width, "location")?;
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
    let width = get_text_width(&weather_info.time, time_font_size);
    check_width(width, info_block_width, "time")?;
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
    let bottom_pos_x = bottom_pos_x + info_block_width;
    let bottom_pos_ov1_y = IMG_HEIGHT - BOTTOM_INFO_HEIGHT + 10;
    let ov1_font_size = 60;
    let width = get_text_width(&weather_info.overview, ov1_font_size);
    check_width(width, info_block_width, "overview")?;
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
    let width = get_text_width(&weather_info.overview2, ov2_font_size);
    check_width(width, info_block_width, "overview2")?;
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
    let bottom_pos_x = bottom_pos_x + info_block_width;
    let bottom_pos_humd_y = IMG_HEIGHT - BOTTOM_INFO_HEIGHT + 30;
    let water_drop_icon =
        image::load_from_memory(include_bytes!("../img/water_drop.png"))
            .unwrap()
            .to_rgba();
    imageops::overlay(
        &mut origin_img,
        &water_drop_icon,
        bottom_pos_x,
        bottom_pos_humd_y,
    );

    let bottom_pos_humd_x = bottom_pos_x + 48 + 10; // 48 is icon width
    let humd_font_size = 80.0;
    let humd_str = format!("{}%", &weather_info.humd);
    check_value(weather_info.humd, "humidity")?;
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
    let bottom_pos_x = bottom_pos_x + info_block_width;
    let bottom_pos_temp_y = IMG_HEIGHT - BOTTOM_INFO_HEIGHT + 30;
    let thermometer_icon =
        image::load_from_memory(include_bytes!("../img/thermometer.png"))
            .unwrap()
            .to_rgba();
    imageops::overlay(
        &mut origin_img,
        &thermometer_icon,
        bottom_pos_x,
        bottom_pos_temp_y,
    );

    let bottom_pos_x = bottom_pos_x + 40 + 10; // 40 is icon width
    let temp_font_size = 80.0;
    let temp_str = format!("{}℃", &weather_info.temp);
    check_value(weather_info.temp, "temperature")?;
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
    Ok(())
}

// Image outline for Chinese-Mode
//
// 800 x 800 input Chinese image
// +-------------------------------------------+
// |   800 x 130                               |
// |   Greeting Title                          |
// |                                           |
// +-------------------------------------------+
// |                                           |
// |    Image                                  |
// |                                           |
// +-------------------------------------------+
// |      800 x 130(260) Weather Title         |
// |                                           |
// +------------+-----------+------------------+
// |  Taipei    | Rainy     |         |        |
// |            |           |  87%    | 25°C   |
// |  Tomorrow  | Very Hot  |         |        |
// |  15:00     |           |         |        |
// +------------+-----------+---------+--------+
//  800 x 130 Info, 200 each block

const WEATHER_TITLE_HEIGHT: u32 = 100;

pub fn draw_chinese(
    image_path: &str,
    weather_info: &WeatherInfo,
    output_path: &str,
) -> Result<(), DrawError> {
    let background_color = Rgba([94u8, 94u8, 94u8, 100u8]);
    let text_color = Rgba([255u8, 255u8, 255u8, 255u8]);
    let info_block_width: u32 = 200;

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

    // greeting title background
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

    let text_height = TITLE_HEIGHT - 10;

    // greeting title
    let choices = vec![
        "新年快樂，諸事吉祥",
        "新年快樂，諸事如意",
        "新年快樂，諸事圓融",
        "新年快樂，諸事順利",
        "新年快樂，珠圓玉潤",
        "新年快樂，合璧連珠",
        "新年快樂，朱玉滿堂",
        "新年快樂，福相如豬",
        "新年快樂，豬年吉祥",
        "新年快樂，豬事大吉",
        "豬入門，百福臻",
        "新年快樂，金豬獻吉",
        "新年快樂，金豬賀歲",
        "新年快樂，金豬獻瑞",
        "豬報平安，豬肥人富",
        "春花百開，珠豬引福",
        "新年快樂，吉祥如豬",
        "新年快樂，喜從豬來",
        "新年快樂，豬豬平安",
        "新年快樂，朱帨迎祥",
        "新年快樂，豬年好運",
        "新年快樂，金豬頌春",
        "豬行大運，豬旺旺來",
        "豬年到好運到",
    ];
    let greeting_title =
        choices[rand::thread_rng().gen_range(0, choices.len())];

    let width = get_text_width(greeting_title, text_height);

    draw_text_mut(
        &mut origin_img,
        text_color,
        ((IMG_WIDTH - width) as f32 / 2.0).round() as u32,
        0,
        Scale::uniform(text_height as f32),
        &font,
        greeting_title,
    );

    // weather title
    let text_height = WEATHER_TITLE_HEIGHT - 20;
    let width = get_text_width(&weather_info.title, text_height);
    // title is short enough in a line
    if width < IMG_WIDTH {
        let title_pos = IMG_HEIGHT - BOTTOM_INFO_HEIGHT - WEATHER_TITLE_HEIGHT;
        imageops::overlay(&mut origin_img, &bg_img_title, 0, title_pos);
        draw_text_mut(
            &mut origin_img,
            text_color,
            ((IMG_WIDTH - width) as f32 / 2.0).round() as u32,
            title_pos,
            Scale::uniform(text_height as f32),
            &font,
            &weather_info.title,
        );
    } else {
        let title_pos =
            IMG_HEIGHT - BOTTOM_INFO_HEIGHT - 2 * WEATHER_TITLE_HEIGHT;

        // Double the weather title background
        imageops::overlay(&mut origin_img, &bg_img_title, 0, title_pos);
        imageops::overlay(
            &mut origin_img,
            &bg_img_title,
            0,
            title_pos + WEATHER_TITLE_HEIGHT,
        );
        // print title in two lines
        let (first, last) = &weather_info.title.split_at(3 * 14);
        let width = get_text_width(first, text_height);
        draw_text_mut(
            &mut origin_img,
            text_color,
            ((IMG_WIDTH - width) as f32 / 2.0).round() as u32,
            title_pos,
            Scale::uniform(text_height as f32),
            &font,
            first,
        );
        draw_text_mut(
            &mut origin_img,
            text_color,
            ((IMG_WIDTH - width) as f32 / 2.0).round() as u32,
            title_pos + WEATHER_TITLE_HEIGHT,
            Scale::uniform(text_height as f32),
            &font,
            last,
        );
    }

    // Location
    let bottom_pos_x = 10;
    let bottom_pos_loc_y = IMG_HEIGHT - BOTTOM_INFO_HEIGHT;
    let location_font_size = 80;
    let width = get_text_width(&weather_info.location, location_font_size);
    check_width(width, info_block_width, "location")?;
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
    let width = get_text_width(&weather_info.time, time_font_size);
    check_width(width, info_block_width, "time")?;
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
    let bottom_pos_x = bottom_pos_x + info_block_width;
    let bottom_pos_ov1_y = IMG_HEIGHT - BOTTOM_INFO_HEIGHT + 10;
    let ov1_font_size = 60;
    let width = get_text_width(&weather_info.overview, ov1_font_size);
    check_width(width, info_block_width, "overview")?;
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
    let width = get_text_width(&weather_info.overview2, ov2_font_size);
    check_width(width, info_block_width, "overview2")?;
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
    let bottom_pos_x = bottom_pos_x + info_block_width;
    let bottom_pos_humd_y = IMG_HEIGHT - BOTTOM_INFO_HEIGHT + 30;
    let water_drop_icon =
        image::load_from_memory(include_bytes!("../img/water_drop.png"))
            .unwrap()
            .to_rgba();
    imageops::overlay(
        &mut origin_img,
        &water_drop_icon,
        bottom_pos_x,
        bottom_pos_humd_y,
    );

    let bottom_pos_humd_x = bottom_pos_x + 48 + 10; // 48 is icon width
    let humd_font_size = 80.0;
    let humd_str = format!("{}%", &weather_info.humd);
    check_value(weather_info.humd, "humidity")?;
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
    let bottom_pos_x = bottom_pos_x + info_block_width;
    let bottom_pos_temp_y = IMG_HEIGHT - BOTTOM_INFO_HEIGHT + 30;
    let thermometer_icon =
        image::load_from_memory(include_bytes!("../img/thermometer.png"))
            .unwrap()
            .to_rgba();
    imageops::overlay(
        &mut origin_img,
        &thermometer_icon,
        bottom_pos_x,
        bottom_pos_temp_y,
    );

    let bottom_pos_x = bottom_pos_x + 40 + 10; // 40 is icon width
    let temp_font_size = 80.0;
    let temp_str = format!("{}℃", &weather_info.temp);
    check_value(weather_info.temp, "temperature")?;
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
