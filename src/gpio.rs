extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};

use std::thread::sleep;
use std::time::Duration;

pub fn power_relay_on_for(duration: u64) {
  let pin = Pin::new(21);
  pin.export();
  pin.set_direction(Direction::Out);
  println!("pin {} is on", pin.get_pin());

  pin.set_value(1);
  sleep(Duration::from_secs(duration));
  pin.set_value(0);
  println!("pin {} is off", pin.get_pin())
}

pub fn power_relay_on() {
  let pin = Pin::new(21);
  pin.export();
  pin.set_direction(Direction::Out);
  println!("pin {} is connected and direction set", pin.get_pin());

  let pinOn = pin.set_value(1);
  match pinOn {
    Ok(pinOn) => println!("All is well! {:?}", pinOn),
    Err(e) => println!("ERROR:: {:?}", e),
  }
}

pub fn power_relay_off() {
  let pin = Pin::new(21);
  pin.set_value(0);
  println!("pin {} is now turned off", pin.get_pin())
}
