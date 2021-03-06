use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureQuery};
use sdl2::video::Window;

pub(crate) fn generate_menu(canvas: &mut Canvas<Window>) {
    // side
    canvas.set_draw_color(Color::RGB(0x45, 0x5A, 0x64));
    canvas.fill_rect(Rect::new(512, 0, 200, 480)).unwrap();

    // load font
    let ttf_context = sdl2::ttf::init().unwrap();
    let font = ttf_context
        .load_font("./resources/Roboto-Regular.ttf", 24)
        .unwrap();

    let texture_creator = canvas.texture_creator();
    canvas.set_draw_color(Color::RGB(0xFF, 0x00, 0xC4));

    let menu_item = ["[L]oad", "[R]eset", "[S]etting", "[Q]uit"];

    const PITCH: i32 = 26;
    for (idx, item) in menu_item.iter().enumerate() {
        let surface = font
            .render(item)
            .blended(Color::RGB(0xFF, 0xFF, 0xFF))
            .unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();

        let TextureQuery { width, height, .. } = texture.query();
        canvas
            .copy(
                &texture,
                None,
                Rect::new(512 + PITCH, 360 + (idx as i32 * PITCH), width, height),
            )
            .unwrap();
    }
}
