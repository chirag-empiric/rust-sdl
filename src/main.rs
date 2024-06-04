use std::path::Path;

use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{TextureCreator, WindowCanvas},
    video::WindowContext,
};

fn main() -> Result<(), String> {
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

    let font = ttf_context.load_font(font_path, 128).unwrap();
    let mut event_pump = sdl.event_pump().unwrap();

    let mut userinput: Vec<char> = Vec::new();

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
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Backspace),
                    ..
                } => {
                    //
                    userinput.pop();
                    let str = String::from_iter(userinput.iter());
                    let _ = render(&mut canvas, &texture_creator, &font, &str);
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
        .blended(Color::RGB(255, 0, 0))
        .map_err(|e| e.to_string())?;

    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    let target = Rect::new(100 as i32, 0 as i32, 200 as u32, 100 as u32);
    canvas.copy(&texture, None, Some(target))?;
    canvas.present();
    Ok(())
}
