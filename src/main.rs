mod input;
use crate::input::Input;

use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{TextureCreator, WindowCanvas},
    ttf::FontStyle,
    video::WindowContext,
};
use std::path::Path;

fn main() -> Result<(), String> {
    let mut userinput: Vec<char> = Vec::new();
    let mut curent_cursor_index = 0;
    let mut user_in = Input {
        value: None,
        next: Vec::new(),
    };

    // SDL Config - Move in another module
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("Learninggggggg", 1000, 800)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font_path: &Path = Path::new(&"./src/montserrat/Montserrat-VariableFont_wght.ttf");

    let mut font = ttf_context.load_font(font_path, 128).unwrap();
    font.set_style(FontStyle::BOLD);
    let mut event_pump = sdl.event_pump().unwrap();
    let mut old = 35;

    // Infinite loop to keep display open
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
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
                    if user_in.value == None {
                        user_in.value = Some(ch[0]);
                    } else {
                        let _ = Input::push_trailing(&mut user_in, ch[0], curent_cursor_index);
                        curent_cursor_index += 1;
                    }

                    println!("User input is: {:?}", user_in);
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

                _ => {}
            }
        }
    }
    Ok(())
}

fn render(
    canvas: &mut WindowCanvas,
    texture_creator: &TextureCreator<WindowContext>,
    font: &sdl2::ttf::Font,
    input: &str,
    old: u32,
    operation: Keycode,
) -> Result<u32, String> {
    let color = Color::WHITE;
    canvas.set_draw_color(color);
    canvas.clear();

    let surface = font
        .render(input)
        .blended(Color::BLACK)
        .map_err(|e| e.to_string())?;

    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    let target;
    if operation == Keycode::Backspace {
        target = Rect::new(100 as i32, 0 as i32, old - 20 as u32, 40 as u32);
        canvas.copy(&texture, None, Some(target))?;
        canvas.present();
        return Ok(old - 20);
    } else {
        target = Rect::new(100 as i32, 0 as i32, old + 20 as u32, 40 as u32);
    }
    canvas.copy(&texture, None, Some(target))?;
    canvas.present();
    Ok(old + 20)
}
