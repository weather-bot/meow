extern crate image;
extern crate imageproc;
extern crate rusttype;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::path::Path;
use std::env;
use imageproc::drawing::draw_text_mut;
use image::{Rgba, ImageBuffer, imageops};
use rusttype::{FontCollection, Scale};

#[derive(Serialize, Deserialize)]
struct WeatherInfo {
    title: String,
    time: String,
    temp: f64,
    humd: f64,
    overview: String,
}

fn main() {

    if env::args().count() != 3 {
        panic!("./meow [image_path] [json]")
    }

    let image_path = env::args().nth(1).unwrap();
    let info_json = env::args().nth(2).unwrap();

    // 800x800 image input
    let mut origin_img = image::open(image_path).unwrap().to_rgba();

    // Big title background
    let bg_img_title = ImageBuffer::from_fn(800, 130, |x, y| Rgba([146u8, 146u8, 146u8, 70u8]));
    imageops::overlay(&mut origin_img, &bg_img_title, 0, 0);


    // Weather infomation background
    let bg_img_info = ImageBuffer::from_fn(250, 300, |x, y| Rgba([146u8, 146u8, 146u8, 70u8]));
    imageops::overlay(&mut origin_img, &bg_img_info, 550, 500);

    // font type
    let font = Vec::from(include_bytes!("../font/NotoSansCJKtc-Medium.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font)
        .unwrap()
        .into_font()
        .unwrap();

    // Parse json to WeatherInfo
    let weatherInfo: WeatherInfo = serde_json::from_str(&info_json).unwrap();
    let humd_str = format!("{}%", weatherInfo.humd);
    let temp_str = format!("{}°C", weatherInfo.temp);

    let output_path = Path::new("out.jpg");

    draw_text_mut(
        &mut origin_img,
        Rgba([255u8, 255u8, 255u8, 255u8]),
        40,
        0,
        Scale::uniform(120.0),
        &font,
        &weatherInfo.title,
    );

    let _ = origin_img.save(output_path).unwrap();
}
