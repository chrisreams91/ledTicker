extern crate regex;

use regex::Regex;
use rocket::http::RawStr;

use std::fs;
use std::io::Result;
use std::path::Path;
use std::vec::Vec;

pub fn parse_powerrelay(powerrelay: Option<&RawStr>) -> bool {
    powerrelay
        .map(|powerrelay| match powerrelay.as_str() {
            "true" => true,
            _ => false,
        })
        .unwrap_or_else(|| false)
}

pub fn is_valid_file(directory: &str, file_name: &RawStr) -> bool {
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

// make result throw err if not valid ?
pub fn get_rgb_from_color(color: &str) -> &str {
    let rgb_regex = Regex::new(r"^(?:(?:^|,\s*)([01]?\d\d?|2[0-4]\d|25[0-5])){3}$").unwrap();

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

// refactor parse color or default - pass in default color
pub fn parse_color_or_black(color: Option<&RawStr>) -> &str {
    color
        .map(|color| get_rgb_from_color(color))
        .unwrap_or_else(|| get_rgb_from_color("black"))
}

pub fn help() -> &'static str {
    "Available Routes: 


GET : /images = available images

GET : /gifs = available gifs

GET : /fonts = available fonts


PUT : /scrollimage/<image>/<duration>?<powerrelay>

    ex: /scrollimage/dogeegod/30/?powerrelay=true

PUT : /gif/<gif>/<duration>?<powerrelay>

    ex:  /gif/flexparrot/15

PUT : /scrolltext/<duration>&<text>?<powerrelay>&<color>&<backgroundcolor>&<outlinecolor>&<font>

    ex: /scrolltext/matt is a dum dum head /10?color=black&outlinecolor=orange

PUT : /powerrelay/<duration>

    ex: /powerrelay/30


image / gif = file name, these need to be on the Pi so send them to me if you would like it available or you can put it on yourself. They must be in .ppm or .gif formats respectively

text = the actual text you want displayed, it will loop until the duration is over

duration = seconds


Everything after the ? are optional parameters and must be set by param=value 
    ex powerrelay=true

powerrelay = true to turn on party lights or whatever else is connected

color / backgroundcolor / outlinecolor = Either a color string such as red blue green ( only a handful of commons ones will work ) 
or a valid RGB value ex: 255,127,80

font = font file name - like images it needs to be on Pi so send them to me if you would like more added other wise GET /fonts


"
}
