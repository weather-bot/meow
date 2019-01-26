#[macro_use]
extern crate clap;
extern crate rand;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate image;
extern crate imageproc;
extern crate rusttype;
use std::process;

mod draw;
mod weather_info;

use clap::App;
use draw::{draw_bottom, draw_chinese, draw_corner};
use weather_info::WeatherInfo;

fn main() {
    let yml = load_yaml!("../cli.yml");
    let m = App::from_yaml(yml).get_matches();

    let input_image_path = if let Some(image_) = m.value_of("image") {
        image_
    } else {
        eprintln!("Not specified input image!");
        process::exit(1);
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
        eprintln!("Not specified weather info json!");
        process::exit(1);
    };

    match m.value_of("mode") {
        Some("corner-mode") => {
            match draw_corner(&input_image_path, &info, &output_image_path) {
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                }
                _ => {}
            }
        }
        Some("bottom-mode") => {
            match draw_bottom(&input_image_path, &info, &output_image_path) {
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                }
                _ => {}
            }
        }
        Some("chinese-mode") => {
            match draw_chinese(&input_image_path, &info, &output_image_path) {
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                }
                _ => {}
            }
        }
        _ => {
            eprintln!("Not specified mode!");
            process::exit(1);
        }
    }
    println!("Create Meow Done!");
    process::exit(0);
}
