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

#[put("/<duration>/<image>?<powerrelay>")]
fn display_image(duration: &RawStr, image: &RawStr, powerrelay: Option<&RawStr>) -> &'static str {
    let powerrelay = powerrelay
        .map(|powerrelay| match powerrelay.as_str() {
            "true" => true,
            _ => false,
        })
        .unwrap_or_else(|| false);

    unsafe {
        if BLOCKREQUESTS {
            println!("requests are being blocked");
            "Display Image Failure :: Requests pending"
        } else {
            BLOCKREQUESTS = true;
            let parsed_duration = duration.as_str().parse().unwrap();
            let command = format!(
            "sudo /home/pi/rpi-rgb-led-matrix/examples-api-use/demo -t {} --led-rows=16 --led-chain=3 --led-slowdown-gpio=2 --led-pwm-lsb-nanoseconds 150 -D 1 /home/pi/images/{}.ppm",
            duration, image
            );

            thread::spawn(move || {
                Command::new("sh").arg("-c").arg(command).spawn();
                if powerrelay {
                    gpio::power_relay_on_for(parsed_duration);
                } else {
                    sleep(Duration::from_secs(parsed_duration));
                }
                println!("requests no longer blocked");
                BLOCKREQUESTS = false;
            });
            "Display Image Success"
        }
    }
}

#[put("/scrolltext/<duration>/<text>?<powerrelay>&<color>&<backgroundcolor>&<outlinecolor>&<font>")]
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
    let powerrelay = powerrelay
        .map(|powerrelay| match powerrelay.as_str() {
            "true" => true,
            _ => false,
        })
        .unwrap_or_else(|| false);

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
    unsafe {
        if BLOCKREQUESTS {
            println!("requests are being blocked");
            "Display Text Failure :: Requests pending"
        } else {
            BLOCKREQUESTS = true;
            let parsed_duration = duration.as_str().parse().unwrap();

            let command = format!(
            "sudo timeout {} /home/pi/rpi-rgb-led-matrix/examples-api-use/scrolling-text-example --led-chain=3 --led-slowdown-gpio=2 --led-pwm-lsb-nanoseconds 100 --led-show-refresh -y 10 -f /home/pi/rpi-rgb-led-matrix/fonts/{} -l 1 -C {} -B {} -O {} {}", parsed_duration, font, color, backgroundcolor, outlinecolor, clean_text_looped
            );

            thread::spawn(move || {
                Command::new("sh").arg("-c").arg(command).spawn();
                if powerrelay {
                    gpio::power_relay_on_for(parsed_duration);
                } else {
                    sleep(Duration::from_secs(parsed_duration));
                }
                BLOCKREQUESTS = false;
                println!("aborting process");
            });
            "Display text sucess"
        }
    }
}

#[get("/")]
fn help() -> &'static str {
    "temp"
}

#[get("/<folder>")]
fn get_folder_contents(folder: &RawStr) -> String {
    let path = Path::new(folder.as_str());

    match util::read_directory_contents(path) {
        Ok(folder) => {
            let response = format!("Available {}: \n", path.to_str().unwrap());
            response + &folder.join("\n")
        }
        Err(e) => {
            println!("Error reading from socket stream: {}", e);
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

fn main() {
    println!("Server started");
    rocket::ignite()
        .mount(
            "/",
            routes![
                display_image,
                display_text,
                turn_power_relay_on,
                turn_power_relay_off,
                help,
                get_folder_contents
            ],
        )
        .launch();
}
