mod canvas;
// mod input;
use canvas::render;

use sdl2::{
    event::Event,
    keyboard::{Keycode, Mod},
    pixels::Color,
};
use std::path::Path;

fn main() -> Result<(), String> {
    let mut userinput: Vec<char> = Vec::new();

    // SDL Config - Move in another module
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("Rusty editor :)", 1000, 800)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font_path: &Path = Path::new(&"./src/montserrat/static/Montserrat-SemiBold.ttf");
    let font_size = 128;

    let font = ttf_context.load_font(font_path, font_size).unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let mut old = 15;

    canvas.set_draw_color(Color::WHITE);
    canvas.clear();
    canvas.present();

    // Infinite loop to keep display open
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::TextInput {
                    timestamp,
                    window_id,
                    text,
                } => {
                    println!("timestamp: {timestamp}, window_id:{window_id}, text: {text}");
                    let ch: Vec<char> = text.chars().collect();
                    userinput.push(ch[0]);
                    let str = String::from_iter(userinput.iter());
                    let old_val =
                        render(&mut canvas, &texture_creator, &font, &str, old, Keycode::A);

                    old = old_val.unwrap();
                    println!("old value: {}", old);
                    // regarding tree
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => {
                    //
                    userinput.push('\n');
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Backspace),
                    ..
                } => {
                    //
                    userinput.pop();
                    let str = String::from_iter(userinput.iter());
                    let old_val = render(
                        &mut canvas,
                        &texture_creator,
                        &font,
                        &str,
                        old,
                        Keycode::Backspace,
                    );
                    old = old_val.unwrap_or_else(|_| 20);
                    // regarding tree
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    //
                    println!("Back arrow");
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    //
                    println!("Forwared arrow key")
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    keymod,
                    ..
                } => {
                    if keymod.contains(Mod::LCTRLMOD) || keymod.contains(Mod::RCTRLMOD) {
                        println!("Ctrl + A detected");
                        // Your functionality for Ctrl + A
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::X),
                    keymod,
                    ..
                } => {
                    if keymod.contains(Mod::LCTRLMOD) {
                        break 'running;
                    }
                }
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
    }

    Ok(())
}
