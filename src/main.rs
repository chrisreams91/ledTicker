#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rpi_led_matrix;

// use std::process::Command;

// use rpi_led_matrix::{LedCanvas, LedColor, LedFont, LedMatrix, LedMatrixOptions};

mod led_matrix;
mod parse_text;

#[put("/<image>/<duration>")]
fn display_image(duration: String, image: String) -> &'static str {
    let command = format!(
        "sudo /home/pi/rpi-rgb-led-matrix/examples-api-use/demo -t {} /
        --led-rows=16 --led-chain=3 --led-show-refresh --led-pwm-lsb-nanoseconds 100 /
        -D 1 /home/pi/images/{}.ppm",
        duration, image
    );
    println!("{}", command);
    // Command::new("sh").arg("-c").arg(command).spawn();
    "PUT :: Doge for success"
}

#[post("/<duration>")]
fn display_text(duration: String) -> &'static str {
    println!("{}", duration);
    "POST :: Display Text Success"
}

fn main() {
    println!("Server started");
    rocket::ignite()
        .mount("/", routes![display_image, display_text])
        .launch();
}

// sudo /home/pi/rpi-rgb-led-matrix/examples-api-use/demo -t 20 --led-rows=16 --led-chain=3 --led-show-refresh --led-slowdown-gpio=2 --led-pwm-lsb-nanoseconds 100 -D 1 /home/pi/images/dogeegod.ppm

// // sudo /home/pi/rpi-rgb-led-matrix/examples-api-use/demo -t 20 --led-rows=16 --led-chain=3 --led-show-refresh --led-pwm-lsb-nanoseconds 100 -D 1 /home/pi/images/dumpsterfire.ppm

// // try:

// // sudo apt-get remove bluez bluez-firmware pi-bluetooth triggerhappy pigpio

// // --led-pwm-bits=<1..11>    : PWM bits (Default: 11).

// // --led-slowdown-gpio=2   : Slowdown GPIO. Needed for faster Pis and/or slower panels (Default: 1).

// sudo /home/pi/rpi-rgb-led-matrix/examples-api-use/scrolling-text-example --led-chain=3 -f /home/pi/rpi-rgb-led-matrix/fonts/8x13.bdf  -l 3 -y 9

// -f /home/pi/rpi-rgb-led-matrix/fonts/8x13.bdf // abs path to font
// -l 3 //loops
// -x // origin
// -y 9 // origin
// -C 255,0,0 //color
// -B 0,0,255 // background
// -O 0,0,0 // outline color
// -S 1 //spacing
