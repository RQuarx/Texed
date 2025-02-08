use sdl2::{pixels::Color, rect::Rect, render::Canvas, ttf::Font, video::Window};

use crate::graphics::Offset;

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

pub fn render_text(
    canvas: &mut Canvas<Window>,
    text: &str,
    font: &Font<'_, '_>,
    fg: Color,
    offset: Offset,
) {
    let texture_creator = canvas.texture_creator();

    let surface = font.render(text).blended(fg).unwrap();
    let texture = surface.as_texture(&texture_creator).unwrap();
    let target = Rect::new(
        offset.x as i32,
        offset.y as i32,
        surface.width(),
        surface.height(),
    );
    canvas.copy(&texture, None, Some(target)).unwrap();
}
