pub fn get_rgb_from_color(color: &str) -> &str {
    match color {
        "red" => "255,0,0",
        "blue" => "0,0,255",
        "green" => "51,204,51",
        "darkgreen" => "0,77,28",
        "purple" => "140,26,255",
        "teal" => "51,204,184",
        "orange" => "255,178,55",
        "yellow" => "244,226,66",
        "black" => "0,0,0",
        "white" => "255,255,255",
        "pink" => "255,102,255",
        _ => color,
    }
}
