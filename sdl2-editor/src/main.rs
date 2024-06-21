mod canvas;
mod input;
use canvas::render;

use input::Rope;
use sdl2::{
    event::Event,
    keyboard::{Keycode, Mod},
    pixels::Color,
};
use std::path::Path;

fn main() -> Result<(), String> {
    let mut data = input::Rope::new("").unwrap();
    let mut cursor_at = 0;

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
    let old = 15;

    canvas.set_draw_color(Color::WHITE);
    canvas.clear();
    canvas.present();

    // Infinite loop to keep display open
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                // write s#it
                Event::TextInput { text, .. } => {
                    // regarding tree
                    data = Rope::add_trailing(data.clone(), &text).unwrap();

                    let _ = render(
                        &mut canvas,
                        &texture_creator,
                        &font,
                        &data.traverse().unwrap(),
                        old,
                    );
                    cursor_at += 1;
                }
                // backspace
                Event::KeyDown {
                    keycode: Some(Keycode::Backspace),
                    ..
                } => {
                    // regarding tree

                    println!("From {:?} to {:?}", cursor_at - 2, cursor_at - 1);
                    let response = data.delete_between_index(cursor_at - 2, cursor_at - 1);
                    data = response.unwrap();
                    println!("{:?}", data);
                    let _ = render(
                        &mut canvas,
                        &texture_creator,
                        &font,
                        &data.traverse().unwrap(),
                        old,
                    );

                    cursor_at -= 1;
                }
                // moving cursor
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    cursor_at -= 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    cursor_at += 1;
                }
                Event::KeyDown {
                    timestamp,
                    keycode: Some(Keycode::S),
                    keymod,
                    ..
                } => {
                    if keymod.contains(Mod::LCTRLMOD) || keymod.contains(Mod::RCTRLMOD) {
                        println!("creating dir");
                        let new = data.traverse().unwrap().parse::<String>();
                        let filepath = format!(
                            "./src/files/{}_{}.txt",
                            timestamp.to_string(),
                            new.clone().unwrap()
                        );

                        let r = std::fs::write(filepath, new.unwrap());
                        println!("{:?}", r.unwrap());
                    }
                }
                // Quit stuff
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
