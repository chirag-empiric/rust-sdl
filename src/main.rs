mod input;
use crate::input::Input;

use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{TextureCreator, WindowCanvas},
    sys::quad_t,
    ttf::FontStyle,
    video::WindowContext,
};
use std::path::Path;

fn main() -> Result<(), String> {
    let mut userinput: Vec<char> = Vec::new();
    let mut user_in = Input {
        value: None,
        next: Vec::new(),
    };

    // SDL Config - Move in another module
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("Learninggggggg", 600, 600)
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
                    let _ = render(&mut canvas, &texture_creator, &font, &str);

                    // regarding tree
                    if user_in.value == None {
                        user_in.value = Some(ch[0]);
                    } else {
                        let index_to_push = user_in.next.len();
                        let _ = Input::push_trailing(&mut user_in, ch[0], index_to_push);
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
                    let _ = render(&mut canvas, &texture_creator, &font, &str);
                    // regarding tree
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    //
                    println!("Back arrow key")
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
) -> Result<(), String> {
    let color = Color::RGB(255, 255, 255);
    canvas.set_draw_color(color);
    canvas.clear();

    let surface = font
        .render(input)
        .blended(Color::RGB(0, 0, 0))
        .map_err(|e| e.to_string())?;

    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    let target = Rect::new(100 as i32, 0 as i32, 60 as u32, 60 as u32);
    canvas.copy(&texture, None, Some(target))?;
    canvas.present();
    Ok(())
}
