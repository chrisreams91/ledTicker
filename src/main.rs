#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::RawStr;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

mod gpio;
mod util;

static mut blockRequests: bool = false;

#[put("/<image>/<duration>")]
fn display_image(duration: String, image: String) -> &'static str {
    unsafe {
        if blockRequests {
            println!("requests are being blocked");
            "Display Image Failure :: Requests pending"
        } else {
            blockRequests = true;
            // gpio::power_on_for(duration.parse().unwrap());
            let command = format!(
            "sudo /home/pi/rpi-rgb-led-matrix/examples-api-use/demo -t {} --led-rows=16 --led-chain=3 --led-slowdown-gpio=2 --led-pwm-lsb-nanoseconds 100  --led-show-refresh -D 1 /home/pi/images/{}.ppm",
            duration, image
            );
            sleep(Duration::from_secs(duration.parse().unwrap()));
            // Command::new("sh").arg("-c").arg(command).spawn();
            blockRequests = false;
            println!("requests no longer blocked");
            "Display Image Success"
        }
    }
}

#[post("/<image>/<duration>")]
fn display_text(duration: String, image: String) -> &'static str {
    let command = format!(
        "sudo /home/pi/rpi-rgb-led-matrix/examples-api-use/demo -t {} --led-rows=16 --led-chain=3 --led-slowdown-gpio=2 --led-pwm-lsb-nanoseconds 100  --led-show-refresh -D 1 /home/pi/images/{}.ppm",
        duration, image
    );
    // Command::new("sh").arg("-c").arg(command).spawn();

    gpio::power_relay_on();
    sleep(Duration::from_secs(duration.parse().unwrap()));
    gpio::power_relay_off();
    "POST :: Display Text Success"
}

// #[put("/scrolltext/<color>&<>")]
// fn test(color: Option<&RawStr>) {
//     color
//         .map(|color| color.to_string())
//         .unwrap_or_else(|| "Hello!".into());
// }

#[get("/hello?wave&<name>")]
fn hello(name: Option<&RawStr>) -> String {
    name.map(|name| format!("Hi, {}!", name))
        .unwrap_or_else(|| "Hello!".into())
}

#[put("/powerrelay/on")]
fn turn_power_relay_on() -> &'static str {
    gpio::power_relay_on();
    "GPIO for lights turned on"
}

#[put("/powerrelay/off")]
fn turn_power_relay_off() -> &'static str {
    gpio::power_relay_off();
    "GPIO for lights turned off"
}

fn main() {
    println!("{}", util::get_RGB_from_color("green"));
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
