use crate::{editor::Editor, EXIT_SUCCESS};
use sdl2::{event::Event, keyboard::Scancode, EventPump};

pub fn event_handler(event_pump: &mut EventPump, editor: &mut Editor) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => std::process::exit(EXIT_SUCCESS),
            Event::TextInput { text, .. } => {
                editor.file_content[editor.cursor_pos.y].insert_str(editor.cursor_pos.x, &text);
                editor.cursor_pos.x += 1;
                return true;
            }
            Event::KeyDown { scancode, .. } => {
                return input_handler(editor, scancode.unwrap());
            }
            Event::Window { .. } => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }
    false
}

pub fn input_handler(editor: &mut Editor, scancode: Scancode) -> bool {
    let cursor = &mut editor.cursor_pos;
    match scancode {
        Scancode::Backspace => {
            if cursor.is_zero() {
                return true;
            }

            if cursor.x > 0 {
                editor.file_content[cursor.y].remove(cursor.x - 1);
                cursor.x -= 1;
            }

            if cursor.x == 0 && cursor.y > 0 {
                cursor.x = editor.file_content[cursor.y - 1].len();
            }
            true
        },
        Scancode::Right => {
            true
        }
        _ => false,
    }
}
