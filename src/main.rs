#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::RawStr;
use rocket::http::Status;

use std::path::Path;
use std::process::Command;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

mod gpio;
mod util;

static mut BLOCKREQUESTS: bool = false;

#[put("/scrollimage/<image>/<duration>?<powerrelay>")]
fn display_image(image: &RawStr, duration: &RawStr, powerrelay: Option<&RawStr>) -> &'static str {
    let valid_image = util::parse_file("./images", image);
    let valid_duration = duration.as_str().parse::<u64>().is_ok();

    //https://rocket.rs/v0.4/guide/state/
    unsafe {
        if BLOCKREQUESTS {
            println!("requests are being blocked");
            "Display Image Failure :: Requests pending"
        } else if valid_image && valid_duration {
            BLOCKREQUESTS = true;
            let powerrelay = util::parse_powerrelay(powerrelay);
            let parsed_duration = duration.as_str().parse().unwrap();
            let command = format!(
            "sudo /home/pi/rpi-rgb-led-matrix/examples-api-use/demo -t {} --led-rows=16 --led-chain=3 --led-slowdown-gpio=2 --led-pwm-lsb-nanoseconds 150 -D 1 /home/pi/images/{}.ppm",
            parsed_duration, image
            );

            thread::spawn(move || {
                // Command::new("sh").arg("-c").arg(command).spawn();
                if powerrelay {
                    gpio::power_relay_on_for(parsed_duration);
                } else {
                    sleep(Duration::from_secs(parsed_duration));
                }
                println!("requests no longer blocked");
                BLOCKREQUESTS = false;
            });

            "Display Image Success"
        } else {
            println!("Invalid Request :: Bad Parameters");
            "Invalid Request :: Bad Parameters"
        }
    }
}

#[put("/gif/<gif>/<duration>?<powerrelay>")]
fn display_gif(gif: &RawStr, duration: &RawStr, powerrelay: Option<&RawStr>) -> &'static str {
    let valid_gif = util::parse_file("./gifs", gif);
    let valid_duration = duration.as_str().parse::<u64>().is_ok();

    unsafe {
        if BLOCKREQUESTS {
            println!("requests are being blocked");
            "Display Gif Failure :: Requests pending"
        } else if valid_gif && valid_duration {
            BLOCKREQUESTS = true;
            let powerrelay = util::parse_powerrelay(powerrelay);
            let parsed_duration = duration.as_str().parse().unwrap();

            let base_command = format!(
                "sudo timeout {} /home/pi/rpi-rgb-led-matrix/utils/led-image-viewer",
                parsed_duration
            );

            // premade or alow req params to set all args
            let args = match gif.as_str() {
                "pacman" => " --led-rows=64 -C --led-chain=3 /home/pi/gifs/pacman.gif",
                "nyancat" => " --led-rows=64 -C --led-chain=3 /home/pi/gifs/nyancat.gif",
                "ditto" => " --led-slowdown-gpio=2 --led-rows=32 -C --led-chain=1 --led-brightness=40 /home/pi/gifs/ditto.gif",
                "mariobanana" => " --led-rows=64 -C --led-chain=3 /home/pi/gifs/mariobanana.gif",
                "flexdumpster" => " --led-rows=16 -C --led-chain=3 /home/pi/gifs/flexdumpster.gif",
                _ => ""
            };
            let command = format!("{}{}", base_command, args);

            // some gifs take longer than other to load set timeout?
            thread::spawn(move || {
                println!("{}", command);
                Command::new("sh").arg("-c").arg(command).spawn();
                if powerrelay {
                    gpio::power_relay_on_for(parsed_duration);
                } else {
                    sleep(Duration::from_secs(parsed_duration));
                }
                println!("requests no longer blocked");
                BLOCKREQUESTS = false;
            });
            "Display GIF Success"
        } else {
            println!("Invalid Request :: Bad Parameters");
            "Invalid Request :: Bad Parameters"
        }
    }
}

#[put("/scrolltext/<text>/<duration>?<powerrelay>&<color>&<backgroundcolor>&<outlinecolor>&<font>")]
fn display_text(
    duration: &RawStr,
    text: &RawStr,
    color: Option<&RawStr>,
    backgroundcolor: Option<&RawStr>,
    outlinecolor: Option<&RawStr>,
    font: Option<&RawStr>,
    powerrelay: Option<&RawStr>,
    // speed: Option<&RawStr>
) -> &'static str {
    let valid_duration = duration.as_str().parse::<u64>().is_ok();

    unsafe {
        if BLOCKREQUESTS {
            println!("requests are being blocked");
            "Display Text Failure :: Requests pending"
        } else if valid_duration {
            BLOCKREQUESTS = true;
            let powerrelay = util::parse_powerrelay(powerrelay);
            let parsed_duration = duration.as_str().parse().unwrap();

            let color = color
                .map(|color| util::get_rgb_from_color(color))
                .unwrap_or_else(|| "255,255,255");

            let backgroundcolor = backgroundcolor
                .map(|backgroundcolor| util::get_rgb_from_color(backgroundcolor))
                .unwrap_or_else(|| "0,0,0");

            let outlinecolor = outlinecolor
                .map(|outlinecolor| util::get_rgb_from_color(outlinecolor))
                .unwrap_or_else(|| "0,0,0");

            let font = font
                .map(|font| font.as_str())
                .unwrap_or_else(|| "8x13B.bdf");

            let text_decoded = text.percent_decode().unwrap();
            let mut clean_text_looped = String::new();
            for _number in 1..duration.parse().unwrap() {
                clean_text_looped = clean_text_looped + "  " + &text_decoded;
            }

            let command = format!(
            "sudo timeout {} /home/pi/rpi-rgb-led-matrix/examples-api-use/scrolling-text-example --led-chain=3 --led-slowdown-gpio=2 --led-pwm-lsb-nanoseconds 100 --led-show-refresh -y 10 -f /home/pi/rpi-rgb-led-matrix/fonts/{} -l 1 -C {} -B {} -O {} {}", parsed_duration, font, color, backgroundcolor, outlinecolor, clean_text_looped
            );

            thread::spawn(move || {
                // Command::new("sh").arg("-c").arg(command).spawn();
                if powerrelay {
                    gpio::power_relay_on_for(parsed_duration);
                } else {
                    sleep(Duration::from_secs(parsed_duration));
                }
                println!("aborting process");
                BLOCKREQUESTS = false;
            });
            "Display text sucess"
        } else {
            println!("Invalid Request :: Bad Parameters");
            "Invalid Request :: Bad Parameters"
        }
    }
}

#[get("/")]
fn help() -> &'static str {
    util::help()
}

//error handling ?
#[get("/<folder>")]
fn get_folder_contents(folder: &RawStr) -> String {
    let path = Path::new(folder.as_str());

    match util::read_directory_contents(path) {
        Ok(folder) => {
            let response = format!("Available {}: \n", path.to_str().unwrap());
            response + &folder.join("\n")
        }
        Err(e) => {
            println!("{}", e);
            String::from("Error: This folder does not exist")
        }
    }
}

#[put("/powerrelay/on?<duration>")]
fn turn_power_relay_on(duration: &RawStr) -> &'static str {
    let parsed_duration = duration.as_str().parse().unwrap();
    gpio::power_relay_on_for(parsed_duration);
    "GPIO for power relay turned on"
}

#[put("/powerrelay/off")]
fn turn_power_relay_off() -> &'static str {
    gpio::power_relay_off();
    "GPIO for power relay turned off"
}

#[put("/blockrequests")]
fn flip_block_requests() -> &'static str {
    unsafe {
        BLOCKREQUESTS = !BLOCKREQUESTS;
    }
    "global var BLOCKREQUESTS flipped"
}

fn main() {
    println!("Server started");
    rocket::ignite()
        .mount(
            "/",
            routes![
                display_image,
                display_text,
                display_gif,
                turn_power_relay_on,
                turn_power_relay_off,
                get_folder_contents,
                help,
                flip_block_requests
            ],
        )
        .launch();
}
