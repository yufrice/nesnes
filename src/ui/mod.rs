pub mod menu;
pub mod sprite_map;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use std::cell::RefCell;
use std::rc::Rc;

use crate::arch::Arch;
use crate::parser;

pub fn run() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("", 712, 480)
        .position_centered()
        .build()
        .unwrap();

    let canvas = Rc::new(RefCell::new(window.into_canvas().build().unwrap()));

    // nes側
    let (prg, chr) = parser::parser("test.nes").unwrap();
    let arch = Arch::new(prg, chr, canvas.clone());
    let character = arch.ppu.sprite_flush();

    let texture_creator = canvas.borrow().texture_creator();
    let texture = sprite_map::generate_sprites(texture_creator, character);
    let TextureQuery { width, height, .. } = texture.query();

    canvas
        .borrow_mut()
        .set_draw_color(Color::RGB(0xC4, 0xC4, 0xC4));
    canvas.borrow_mut().clear();

    menu::generate_menu(&mut canvas.borrow_mut());
    canvas
        .borrow_mut()
        .copy(&texture, None, Rect::new(550, 10, width, height))
        .unwrap();
    canvas.borrow_mut().present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_event() {
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

        canvas.borrow_mut().present();
    }
}
