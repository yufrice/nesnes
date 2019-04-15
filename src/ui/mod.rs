use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use std::time::Duration;

use crate::arch::Arch;

// ToDo テスト分離させてメインUIとPPUと連動させる
pub fn run(arch: Arch) {
    let character = arch.ppu.sprite_flush();
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("", 1024, 1024)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let col: [[u8; 3]; 4] = [
        [0x00u8, 0x00u8, 0x00u8],
        [0xCFu8, 0x29u8, 0x50u8],
        [0xFFu8, 0xFFu8, 0xFFu8],
        [0x99u8, 0xFFu8, 0xFCu8],
    ];

    const TEXTURE_WIDTH: u32 = 128;
    const TEXTURE_HEIGHT: u32 = 512;
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, TEXTURE_WIDTH, TEXTURE_HEIGHT)
        .unwrap();
    texture
        .with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for (sprite_idx, sprite) in character.chunks(16).enumerate() {
                let (pixel0, pixel1) = sprite.split_at(8);
                for (idx, (pix0, pix1)) in pixel0.iter().zip(pixel1).enumerate() {
                    for x in (0..8).rev() {
                        // N^3 -> N^1
                        fn sprite_next(idx: usize) -> usize {
                            const SPRITE_NEXT: usize = (128 * 3 * 8) as usize;
                            let idx_mod = idx % 15;
                            let sprite_x_idx = (idx - idx_mod) / 15;
                            idx_mod * 24 + sprite_x_idx * SPRITE_NEXT
                        }

                        let b = (((pix0 & 2u8.pow(x)) >> x) + ((pix1 & 2u8.pow(x)) >> x)) as usize;
                        let offset = sprite_next(sprite_idx) + pitch * idx + 3 * (7 - x) as usize;
                        buffer[offset] = col[b][0];
                        buffer[offset + 1] = col[b][1];
                        buffer[offset + 2] = col[b][2];
                    }
                }
            }
        })
        .unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas
        .copy(&texture, None, Some(Rect::new(000, 000, 256, 1024)))
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
        // The rest of the game loop goes here...
        arch.frame();

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
