extern crate rpi_led_matrix;
use std::path::Path;

use rpi_led_matrix::{LedCanvas, LedColor, LedFont, LedMatrix, LedMatrixOptions};

pub fn create_matrix() {
  let options = LedMatrixOptions::new();
  options.set_rows(16);
  options.set_chain_length(3);

  let matrix = LedMatrix::new(Some(options)).unwrap();
}

pub fn select_font(font: String) {
  let path = Path::new(&font);
  let newfont = LedFont::new(path);
}
