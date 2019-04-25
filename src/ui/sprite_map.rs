use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;


use crate::arch::ppu::Pattern;

pub(crate) fn generate_sprites(
    texture_creator: TextureCreator<WindowContext>,
    chr: Pattern,
) -> Texture {
    let col: [[u8; 3]; 4] = [
        [0x00u8, 0x00u8, 0x00u8],
        [0xCFu8, 0x29u8, 0x50u8],
        [0xFFu8, 0xFFu8, 0xFFu8],
        [0x99u8, 0xFFu8, 0xFCu8],
    ];

    const TEXTURE_WIDTH: u32 = 128;
    const TEXTURE_HEIGHT: u32 = 340;
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, TEXTURE_WIDTH, TEXTURE_HEIGHT)
        .unwrap();
    texture
        .with_lock(None, |buffer: &mut [u8], _: usize| {
            for (sprite_idx, sprite) in chr.iter().enumerate() {
                for (idx, pixel) in sprite.iter().enumerate() {
                    let sprite_next = |idx, sprite_idx| -> usize {
                        let sprite_x_idx = sprite_idx % 16;
                        let sprite_y_idx = sprite_idx / 16;
                        let pixel_x_idx = idx % 8;
                        let pixel_y_idx = idx / 8;
                        sprite_x_idx * 3 * 8
                            + sprite_y_idx * 128 * 8 * 3
                            + pixel_x_idx * 3
                            + pixel_y_idx * 128 * 3
                    };
                    let offset = sprite_next(idx, sprite_idx);
                    let pixel = *pixel as usize;
                    buffer[offset] = col[pixel][0];
                    buffer[offset + 1] = col[pixel][1];
                    buffer[offset + 3] = col[pixel][2];
                }
            }
        })
        .unwrap();
    texture
}
