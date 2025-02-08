use std::{fs, path::PathBuf};

use sdl2::{pixels::Color, rect::Rect, render::Canvas, ttf::Font, video::Window};

use crate::{graphics::Offset, parse_config::Config, utils::hex_to_color};

pub struct CursorPos {
    pub x: usize,
    pub y: usize,
    pub max_x: usize,
}

impl CursorPos {
    pub fn is_zero(&self) -> bool {
        self.x == 0 && self.y == 0
    }
}

pub enum EditorMode {
    Insert,
    Normal,
    Visual,
    Replace,
    Command,
}

pub struct Editor {
    pub file_content: Vec<String>,
    pub cursor_pos: CursorPos,
    pub scroll_offset: u32,
    pub editor_mode: EditorMode,
}

impl Editor {
    pub fn init(file_path: PathBuf) -> Self {
        let file_path = file_path.canonicalize().expect("File path doesn't exist");
        let file_content = fs::read_to_string(&file_path).expect("File does not exist!");
        let file_content: Vec<String> = file_content
            .lines()
            .map(|line| format!("{} ", line))
            .collect();

        Editor {
            file_content,
            cursor_pos: CursorPos { x: 0, y: 0, max_x: 0 },
            scroll_offset: 0,
            editor_mode: EditorMode::Normal,
        }
    }

    pub fn render_loop(
        &self,
        canvas: &mut Canvas<Window>,
        config: &Config,
        font: &Font<'_, '_>,
        offset: Offset,
    ) {
        let mut y_offset: u32 = offset.y + 10;
        let last_rendered_index = std::cmp::min(
            self.scroll_offset as usize
                + (canvas.output_size().unwrap().1 / font.height() as u32) as usize,
            self.file_content.len(),
        );

        for (i, line) in self.file_content.iter().enumerate() {
            if i < self.scroll_offset as usize {
                continue;
            } else if i > last_rendered_index {
                continue;
            }
            println!("{}", i);
            let line_number_width =
                self.render_line_number(canvas, i, config, font, y_offset) + offset.x;
            Editor::render_text(
                canvas,
                line,
                hex_to_color(&config.colors.foreground),
                font,
                Offset {
                    x: line_number_width,
                    y: y_offset,
                },
            );
            y_offset += font.height() as u32;
            self.render_cursor(
                canvas,
                config,
                font,
                &self.cursor_pos,
                &Offset {
                    x: line_number_width,
                    y: offset.y + 10,
                },
            );
        }
    }

    fn render_line_number(
        &self,
        canvas: &mut Canvas<Window>,
        line_index: usize,
        config: &Config,
        font: &Font<'_, '_>,
        y_offset: u32,
    ) -> u32 {
        let line_number = if config.line_num.relative {
            (self.cursor_pos.y as i32 - line_index as i32).abs()
        } else {
            if config.line_num.zero_indexing {
                line_index as i32
            } else {
                line_index as i32 + 1
            }
        };

        let line_number_str = line_number.to_string();
        let max_digits = (self.file_content.len() as f32).log(10.0).ceil() as usize + 1;
        let padded_line_number_str = format!("{:>width$} ", line_number_str, width = max_digits);

        let color = if line_index == self.cursor_pos.y {
            hex_to_color(&config.colors.foreground)
        } else {
            hex_to_color(&config.colors.alt_foreground)
        };

        Editor::render_text(
            canvas,
            &padded_line_number_str,
            color,
            font,
            Offset { x: 0, y: y_offset },
        );

        font.size_of(&padded_line_number_str).unwrap().0 + 20
    }

    pub fn render_text(
        canvas: &mut Canvas<Window>,
        text: &str,
        color: Color,
        font: &Font<'_, '_>,
        offset: Offset,
    ) {
        let texture_creator = canvas.texture_creator();

        let surface = font.render(text).blended(color).unwrap();
        let texture = surface.as_texture(&texture_creator).unwrap();
        let target = Rect::new(
            offset.x as i32,
            offset.y as i32,
            surface.width(),
            surface.height(),
        );
        canvas.copy(&texture, None, Some(target)).unwrap();
    }

    fn render_cursor(
        &self,
        canvas: &mut Canvas<Window>,
        config: &Config,
        font: &Font<'_, '_>,
        cursor_pos: &CursorPos,
        offset: &Offset,
    ) {
        match self.editor_mode {
            EditorMode::Insert => {
                self.render_beam_cursor(canvas, config, font, cursor_pos, offset);
            }
            EditorMode::Normal | EditorMode::Command | EditorMode::Visual => {
                self.render_hollow_cursor(canvas, config, font, cursor_pos, offset);
            }
            EditorMode::Replace => {
                self.render_line_cursor(canvas, config, font, cursor_pos, offset);
            }
        }
    }

    fn render_beam_cursor(
        &self,
        canvas: &mut Canvas<Window>,
        config: &Config,
        font: &Font<'_, '_>,
        cursor_pos: &CursorPos,
        offset: &Offset,
    ) {
        let x = font
            .size_of(self.file_content[cursor_pos.y].split_at(cursor_pos.x).0)
            .unwrap()
            .0;
        let cursor_x = x + offset.x;
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

    fn render_line_cursor(
        &self,
        canvas: &mut Canvas<Window>,
        config: &Config,
        font: &Font<'_, '_>,
        cursor_pos: &CursorPos,
        offset: &Offset,
    ) {
        let x = font
            .size_of(self.file_content[cursor_pos.y].split_at(cursor_pos.x).0)
            .unwrap()
            .0;
        let font_size = font.size_of("_").unwrap();
        let cursor_x = x + offset.x;
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

    fn render_hollow_cursor(
        &self,
        canvas: &mut Canvas<Window>,
        config: &Config,
        font: &Font<'_, '_>,
        cursor_pos: &CursorPos,
        offset: &Offset,
    ) {
        let x = font
            .size_of(self.file_content[cursor_pos.y].split_at(cursor_pos.x).0)
            .unwrap()
            .0;
        let font_size = font.size_of("_").unwrap();
        let cursor_x = x + offset.x;
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
}
