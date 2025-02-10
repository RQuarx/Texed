use sdl2::{pixels::Color, rect::Rect, render::Canvas, ttf::Font, video::Window};

use crate::{editor::CursorPos, graphics::Offset, parse_config::Config, utils::hex_to_color};

pub fn render_beam_cursor(
    canvas: &mut Canvas<Window>,
    config: &Config,
    font: &Font<'_, '_>,
    cursor_pos: &CursorPos,
    offset: &Offset,
) {
    let cursor_x = offset.x;
    let cursor_y = offset.y + (cursor_pos.y as u32 * font.height() as u32);

    canvas.set_draw_color(hex_to_color(&config.colors.cursor));
    let cursor_rect = Rect::new(
        cursor_x as i32,
        cursor_y as i32,
        config.cursor.cursor_thickness,
        font.height() as u32,
    );
    canvas.fill_rect(cursor_rect).unwrap();
}

pub fn render_line_cursor(
    canvas: &mut Canvas<Window>,
    config: &Config,
    font: &Font<'_, '_>,
    cursor_pos: &CursorPos,
    offset: &Offset,
) {
    let font_size = font.size_of("_").unwrap();
    let cursor_x = offset.x;
    let cursor_y = offset.y + font_size.1 + (cursor_pos.y as u32 * font.height() as u32);

    canvas.set_draw_color(hex_to_color(&config.colors.cursor));
    let cursor_rect = Rect::new(
        cursor_x as i32,
        cursor_y as i32,
        font_size.0,
        config.cursor.cursor_thickness,
    );
    canvas.fill_rect(cursor_rect).unwrap();
}

pub fn render_hollow_cursor(
    canvas: &mut Canvas<Window>,
    config: &Config,
    font: &Font<'_, '_>,
    cursor_pos: &CursorPos,
    offset: &Offset,
) {
    let font_size = font.size_of("_").unwrap();
    let cursor_x = offset.x;
    let cursor_y = offset.y + (cursor_pos.y as u32 * font.height() as u32);

    canvas.set_draw_color(hex_to_color(&config.colors.cursor));
    let cursor_rect = Rect::new(
        cursor_x as i32,
        cursor_y as i32,
        font_size.0,
        font.height() as u32,
    );
    canvas.draw_rect(cursor_rect).unwrap();
}

pub fn render_block_cursor(
    canvas: &mut Canvas<Window>,
    config: &Config,
    font: &Font<'_, '_>,
    cursor_pos: &CursorPos,
    offset: &Offset,
) {
    let font_size = font.size_of("_").unwrap();
    let cursor_x = offset.x;
    let cursor_y = offset.y + (cursor_pos.y as u32 * font.height() as u32);

    let color = hex_to_color(&config.colors.cursor);
    let transparency: u8 = (255.0 / config.colors.transparency).floor() as u8;

    canvas.set_draw_color(Color::RGBA(color.r, color.g, color.b, transparency));
    let cursor_rect = Rect::new(
        cursor_x as i32,
        cursor_y as i32,
        font_size.0,
        font.height() as u32,
    );
    canvas.fill_rect(cursor_rect).unwrap();
}
