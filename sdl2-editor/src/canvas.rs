use sdl2::{
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{TextureCreator, WindowCanvas},
    video::WindowContext,
};

pub fn render(
    canvas: &mut WindowCanvas,
    texture_creator: &TextureCreator<WindowContext>,
    font: &sdl2::ttf::Font,
    input: &str,
    old: u32,
    operation: Keycode,
) -> Result<u32, String> {
    canvas.clear();

    println!("USer input is: {}", input);
    let surface = font
        .render(input)
        .blended(Color::BLACK)
        .map_err(|e| e.to_string())?;

    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    let target;
    if operation == Keycode::Backspace {
        target = Rect::new(100 as i32, 0 as i32, old - 15 as u32, 40 as u32);
        canvas.copy(&texture, None, Some(target))?;
        canvas.present();
        return Ok(old - 15);
    } else {
        target = Rect::new(100 as i32, 0 as i32, old + 15 as u32, 40 as u32);
    }
    canvas.copy(&texture, None, Some(target))?;
    canvas.present();
    Ok(old + 15)
}
