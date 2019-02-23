pub fn get_RGB_from_color(color: &str) -> &str {
    match color {
        "red" => "255,0,0",
        "blue" => "0,0,255",
        "green" => "51,204,51",
        "purple" => "140,26,255",
        "teal" => "51,204,184",
        "orange" => "255,178,55",
        _ => color,
    }
}