#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::RawStr;
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

#[get("/")]
fn help() -> &'static str {
    "temp"
}

#[get("/fonts")]
fn get_fonts() -> &'static str {
    let path = Path::new("./fonts");
    let contents = util::read_directory_contents(path);
    "temp"
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
                get_fonts
            ],
        )
        .launch();
}

// test this --led-pwm-lsb-nanoseconds 100

// try:

// --led-pwm-bits=<1..11>    : PWM bits (Default: 11).

// -f /home/pi/rpi-rgb-led-matrix/fonts/8x13.bdf // abs path to font
// -l 3 //loops
// -x // origin
// -y 9// origin
// -C 255,0,0 //color
// -B 0,0,255 // background
// -O 0,0,0 // outline color
// -S 1 //spacing

// sudo /home/pi/rpi-rgb-led-matrix/exampe/scrolling-text-example --led-chain=3 --led-slowdown-gpio=2 --led-pwm-lsb-nanoseconds 100 --led-show-refresh -l 5 -y 10 -f /home/pi/rpi-rgb-led-matrix/fonts/8x13B.bdf -C 0,0,255 FLEX VERSION 1.2.47 WAS RELEASED
