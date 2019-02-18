#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use std::process::Command;

const baseCommand: &str = "./demo";

#[get("/<time>")]
fn tryingStuff(time: String) {
    let command = format!("echo {}", time);
    Command::new("sh").arg("-c").arg(command).spawn();
}

#[put("/doge/<time>")]
fn put_on(time: String) -> &'static str {
    let command = format!("./demo -t {} --led-rows=<16> -D 1 images/doge.ppm", time);

    Command::new("sh").arg("-c").arg(command).spawn();
    "PUT :: Doge success {}"
}

#[put("/dumpsterfire/<duration>")]
fn put_off(duration: String) -> &'static str {
    // let baseCommand = "./demo";
    let duration = format!("-t {}", duration);
    let rows = "--led-rows=<16>";
    let refresh = "--led-show-refresh";
    let dumpsterfire = "images/dumpsterfire.ppm";
    println!("{}", baseCommand);
    // Command::new("sh")
    //     .args(["-c", baseCommand, duration, rows, refresh, dumpsterfire])
    //     .spawn();

    "PUT :: Dumpster Fire Success"
}

fn main() {
    println!("Server started");
    rocket::ignite()
        .mount("/", routes![put_on, put_off, tryingStuff])
        .launch();
}
