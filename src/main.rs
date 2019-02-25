#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::RawStr;
use std::process;
use std::process::Command;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

mod gpio;
mod util;

static mut BLOCKREQUESTS: bool = false;

#[put("/<image>/<duration>?<powerrelay>")]
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
                }
                sleep(Duration::from_secs(parsed_duration));
                println!("requests no longer blocked");
                BLOCKREQUESTS = false;
            });
            "Display Image Success"
        }
    }
}

#[put("/scrolltext?<color>&<backgroundcolor>&<outlinecolor>&<spacing>&<font>&<duration>&<text>")]
fn display_text(
    color: Option<&RawStr>,
    backgroundcolor: Option<&RawStr>,
    outlinecolor: Option<&RawStr>,
    spacing: Option<&RawStr>,
    font: Option<&RawStr>,
    duration: &RawStr,
    text: &RawStr,
) -> String {
    let color = color
        .map(|color| util::get_rgb_from_color(color))
        .unwrap_or_else(|| "255,255,255");

    let backgroundcolor = backgroundcolor
        .map(|backgroundcolor| util::get_rgb_from_color(backgroundcolor))
        .unwrap_or_else(|| "0,0,0");

    let outlinecolor = outlinecolor
        .map(|outlinecolor| util::get_rgb_from_color(outlinecolor))
        .unwrap_or_else(|| "0,0,0");

    let spacing = spacing
        .map(|spacing| spacing.as_str())
        .unwrap_or_else(|| "1");

    let font = font
        .map(|font| font.as_str())
        .unwrap_or_else(|| "8x13B.bdf");

    let text_decoded = text.percent_decode().unwrap();
    let mut clean_text_looped = String::new();
    for number in 1..duration.parse().unwrap() {
        clean_text_looped = clean_text_looped + "  " + &text_decoded;
    }

    let command = format!(
    "sudo /home/pi/rpi-rgb-led-matrix/examples-api-use/scrolling-text-example --led-chain=3 --led-slowdown-gpio=2 --led-pwm-lsb-nanoseconds 100 --led-show-refresh -y 10 -f /home/pi/rpi-rgb-led-matrix/fonts/{} -S {} -C {} -B {} -O {} {}", font, spacing, color, backgroundcolor, outlinecolor, clean_text_looped
    );

    let duration = duration.as_str().parse().unwrap();
    thread::spawn(move || {
        Command::new("sh").arg("-c").arg(command).spawn();
        sleep(Duration::from_secs(duration));
        println!("aborting process");
        process::abort();
    });

    format!(
        "color: {}, backgroundcolor: {}, outlinecolor: {}, spacing: {},durations: {}, text: {}",
        color, backgroundcolor, outlinecolor, spacing, duration, clean_text_looped
    )
}

#[put("/powerrelay/on")]
fn turn_power_relay_on() -> &'static str {
    gpio::power_relay_on();
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
                turn_power_relay_off
            ],
        )
        .launch();
}

// test this --led-pwm-lsb-nanoseconds 100

// try:

// --led-pwm-bits=<1..11>    : PWM bits (Default: 11).

// --led-slowdown-gpio=2   : Slowdown GPIO. Needed for faster Pis and/or slower panels (Default: 1).

// -f /home/pi/rpi-rgb-led-matrix/fonts/8x13.bdf // abs path to font
// -l 3 //loops
// -x // origin
// -y 9// origin
// -C 255,0,0 //color
// -B 0,0,255 // background
// -O 0,0,0 // outline color
// -S 1 //spacing
