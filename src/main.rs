#[macro_use]
extern crate clap;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate image;
extern crate imageproc;
extern crate rusttype;

mod draw;
mod weather_info;

use clap::App;
use draw::{draw_bottom, draw_corner};
use weather_info::WeatherInfo;

fn main() {
    let yml = load_yaml!("../cli.yml");
    let m = App::from_yaml(yml).get_matches();

    let input_image_path = if let Some(image_) = m.value_of("image") {
        image_
    } else {
        panic!("Not specified input image!");
    };
    let output_image_path = if let Some(output_) = m.value_of("output") {
        output_
    } else {
        "out.jpg"
    };
    let info: WeatherInfo = if let Some(info_json) = m.value_of("info_json") {
        // Parse json to WeatherInfo
        serde_json::from_str(&info_json).unwrap()
    } else {
        panic!("Not specified weather info json!");
    };

    match m.value_of("mode") {
        Some("corner-mode") => {
            draw_corner(&input_image_path, &info, &output_image_path);
        }
        Some("bottom-mode") => {
            draw_bottom(&input_image_path, &info, &output_image_path);
        }
        _ => panic!("Not specified mode!"),
    }
    println!("Create Meow Done!");
}
