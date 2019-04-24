pub mod menu;
pub mod sprite_map;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use sdl2::render::TextureQuery;
use std::time::Duration;
use crate::parser;

pub fn run() {
    let arch = parser::parser("test0.nes").unwrap();

    let character = arch.ppu.sprite_flush();
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("", 712, 480)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let texture = sprite_map::generate_sprites(texture_creator, character);
    let TextureQuery { width, height, .. } = texture.query();


    canvas.set_draw_color(Color::RGB(0xC4, 0xC4, 0xC4));
    canvas.clear();

    menu::generate_menu(&mut canvas);
    canvas
        .copy(&texture, None, Rect::new(550, 10, width, height))
        .unwrap();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        arch.frame();

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
