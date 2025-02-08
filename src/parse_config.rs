use std::fs;

use serde::Deserialize;

#[derive(Deserialize)]
pub enum Position {
    Right,
    Down,
    Left,
    Up,
}

#[derive(Deserialize)]
pub enum CursorType {
    Hollow,
    Beam,
    Line,
    Box,
}

#[derive(Deserialize)]
#[warn(dead_code)]
pub struct Cursor {
    pub cursor_type: CursorType,
    pub cursor_thickness: i32,
}

#[derive(Deserialize)]
#[warn(dead_code)]
pub struct Colors {
    pub background: String,
    pub foreground: String,
    pub cursor: String,
    pub alt_foreground: String,
}

#[derive(Deserialize)]
#[warn(dead_code)]
pub struct LineNumber {
    pub relative: bool,
    pub zero_indexing: bool,
}

#[derive(Deserialize)]
#[warn(dead_code)]
pub struct Config {
    pub cursor: Cursor,
    pub colors: Colors,
    pub line_num: LineNumber,
}

pub fn load_config(config_path: String) -> Config {
    let config_str = fs::read_to_string(config_path).expect("Failed to read config!");
    toml::de::from_str(&config_str).expect("Failed to parse TOML file!")
}
