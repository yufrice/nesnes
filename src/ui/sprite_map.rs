use sdl2::render::{Texture,TextureCreator};
use sdl2::video::WindowContext;
use sdl2::pixels::PixelFormatEnum;

pub(crate) fn generate_sprite(texture_creator: TextureCreator<WindowContext>, chr: &Vec<u8>)
    -> Texture {
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
        .with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for (sprite_idx, sprite) in chr.chunks(16).enumerate() {
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
        });
        texture
}