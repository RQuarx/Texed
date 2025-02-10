use sdl2::pixels::Color;

pub fn hex_to_color(hex_input: &str) -> Color {
    let hex_input: String = hex_input
        .chars()
        .filter(|&c| c != ' ' && c != '#')
        .collect();

    if hex_input.len() != 6 {
        panic!("Invalid hex code length: {}", hex_input);
    }

    let r = u8::from_str_radix(&hex_input[0..2], 16).expect("Invalid hex value for: RR");
    let g = u8::from_str_radix(&hex_input[2..4], 16).expect("Invalid hex value for: GG");
    let b = u8::from_str_radix(&hex_input[4..6], 16).expect("Invalid hex value for: BB");
    Color::RGB(r, g, b)
}

pub fn is_whitespace(input_str: &str, nth: usize) -> bool {
    input_str.chars().nth(nth).unwrap().is_whitespace()
}