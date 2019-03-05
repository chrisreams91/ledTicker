extern crate regex;

use regex::Regex;
use rocket::http::RawStr;

use std::fs;
use std::io::Result;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use std::vec::Vec;

pub fn parse_powerrelay(powerrelay: Option<&RawStr>) -> bool {
    powerrelay
        .map(|powerrelay| match powerrelay.as_str() {
            "true" => true,
            _ => false,
        })
        .unwrap_or_else(|| false)
}

pub fn parse_file(directory: &str, file_name: &RawStr) -> bool {
    let path = Path::new(directory);
    let contents = read_directory_contents(path).unwrap();
    let file_name = String::from(file_name.as_str());

    contents.contains(&file_name)
}

pub fn read_directory_contents(dir: &Path) -> Result<Vec<String>> {
    let paths = fs::read_dir(dir)?;

    Ok(paths
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path()
                    .with_extension("")
                    .file_name()
                    .and_then(|n| n.to_str().map(String::from))
            })
        })
        .collect::<Vec<String>>())
}

// make result throw err if not valid
pub fn get_rgb_from_color(color: &str) -> &str {
    let rgb_regex = Regex::new(r"^(?:(?:^|,\s*)([01]?\d\d?|2[0-4]\d|25[0-5])){3}$").unwrap();
    println!("{}", rgb_regex.is_match(color));

    match color {
        "red" => "255,0,0",
        "blue" => "0,0,255",
        "green" => "51,204,51",
        "darkgreen" => "0,77,28",
        "purple" => "140,26,255",
        "teal" => "51,204,184",
        "orange" => "255,178,55",
        "yellow" => "244,226,66",
        "black" => "0,0,0",
        "white" => "255,255,255",
        "pink" => "255,102,255",
        _ => {
            if rgb_regex.is_match(color) {
                color
            } else {
                "0,0,0"
            }
        }
    }
}

pub fn help() -> &'static str {
    "available routes: 

        GET : /fonts = available fonts

        GET : /images = available images

        GET : /gifs = available gifs

        PUT : /<image>/<duration>?<powerrelay>

        PUT : /scrolltext/<duration>&<text>?<powerrelay>&<color>&<backgroundcolor>&<outlinecolor>&<font>


    image = image file name - Images need to be on the Pi so send them to me if you would like it available.

    font = font file name - like images it needs to be on Pi so send them to me if you would like more added other wise GET /fonts

    duration = seconds

    powerrelay = true to turn on party lights or whatever else is connected

    color / backgroundcolor / outlinecolor = Either a color string ex red blue green or a valid RGB value ex 255,255,255

    Everything after the ? are optional parameters.
    
        Power relay can be turned on with 'true' (ex: ?powerrelay=true&color=0,0,255)

        There are some preset colors (ex: red, blue, gree, purple)

    "
}
