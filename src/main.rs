mod parse_config;
mod decorations;
mod parse_arg;
mod graphics;
mod editor;
mod input;
mod utils;

use decorations::Decorations;
use editor::Editor;
use sdl2::{
    render::Canvas,
    ttf::{self, Sdl2TtfContext},
    video::Window,
    EventPump, Sdl, VideoSubsystem,
};

use std::{
    env, fs,
    path::{self, PathBuf},
    process,
};

use parse_config::Config;

pub const EXIT_SUCCESS: i32 = 0;
pub const EXIT_FAILURE: i32 = 1;

fn main() {
    let arg_parse: parse_arg::ArgParse = parse_arg::ArgParse::init(env::args());

    if arg_parse.arg("-h", Some("--help")) {
        help_msg();
        process::exit(EXIT_SUCCESS);
    } else if arg_parse.arg("-v", Some("--version")) {
        println!("Texed-0.0");
        process::exit(EXIT_SUCCESS);
    }

    let config_path: path::PathBuf = {
        if arg_parse.arg("-c", Some("--config")) {
            match arg_parse.get_arg_option("-c", Some("--config")) {
                Ok(path) => fs::canonicalize(path).expect("Directory not found!"),
                Err(_) => panic!("Expected a config path after -c or --config"),
            }
        } else {
            let home = env::var("HOME").expect("HOME env not found!");
            let mut config_path = path::PathBuf::from(home);
            config_path.push(".config/texed/config.toml");
            config_path
        }
    };

    let file_name = arg_parse.get_file_path();

    let file_name = {
        if file_name.is_err() {
            fs::File::create("new_file").unwrap();
            PathBuf::from("new_file")
        } else {
            file_name.unwrap()
        }
    };

    let config: &mut Config =
        &mut parse_config::load_config(config_path.into_os_string().into_string().unwrap().clone());

    let sdl: Sdl = sdl2::init().expect("SDL_Init failed");
    let video_subsystem: VideoSubsystem = sdl.video().unwrap();
    let mut ttf_context: Sdl2TtfContext = ttf::init().expect("TTF_Init failed");

    let window: Window = video_subsystem
        .window("texed", 600, 800)
        .allow_highdpi()
        .resizable()
        .build()
        .expect("Window creation failed");

    let mut canvas: Canvas<_> = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .unwrap();
    let mut editor = Editor::init(file_name.clone());
    let mut decorations = Decorations::init(&file_name.display().to_string());

    let mut event_pump: EventPump = sdl.event_pump().unwrap();

    let font = {
        let ttf_context_ = &mut ttf_context;
        ttf_context_
            .load_font("/usr/share/fonts/TTF/JetBrainsMonoNerdFont-Regular.ttf", 24)
            .expect("Failed to load font!")
    };

    graphics::run(&mut decorations, &mut editor, &mut canvas, &mut event_pump, config, &font);
}

fn help_msg() {
    println!("Texed [path] [options]\n");
    println!("Options:");
    println!("    -h, --help                    show this message");
    println!("    -v, --version                 show the texed version");
    println!("    -c, --config                  specify the config path");
}
