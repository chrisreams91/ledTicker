use std::fs;
use std::io;
use std::iter::Iterator;
use std::path::Path;

//fix this
pub fn get_rgb_from_color(color: &str) -> &str {
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
        _ => color,
        // regex to max an RGB value else return 0,0,0
    }
}

pub fn read_directory_contents(dir: &Path) -> std::io::Result<()>{
    let mut x: [];
    for entry in fs::read_dir(dir)? {
        let dir = entry?;
        println!("{:?}", dir.path());
    }
    Ok(())
}

pub fn get_help() -> &'static str {
    "available routes: 

        PUT : /<image>/<duration>?<powerrelay>

        PUT : /scrolltext/<duration>&<text>?<powerrelay>&<color>&<backgroundcolor>&<outlinecolor>&<font>

        GET : /fonts

    image = image file name
        Images need to be on the Pi so send them to me if you would like it available.
    font = font file name
        like images it needs to be on Pi so send them to me if you would like more added other wise GET /fonts
    duration = seconds
    powerrelay = true to turn on party lights or whatever else is connected
    color / backgroundcolor / outlinecolor = Either a color string ex red blue green or a valid RGB value ex 255,255,255

    Everything after the ? are optional parameters.
    
        Power relay can be turned on with 'true' (ex: ?powerrelay=true&color=0,0,255)

        There are some preset colors (ex: red, blue, gree, purple)

    "
}
