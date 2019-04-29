use log::info;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::cell::RefCell;

use crate::arch::memory::{PPUMemory, PPURegister};
use crate::arch::RcRefCell;

/// Sprite(0x40) x 0x200
pub type Pattern = [[u8; 0x40]; 0x200];

pub(crate) struct PPU {
    /// CHR
    pub(crate) pattern0: Pattern,
    pub(crate) state: RefCell<PPUState>,
    pub(crate) display: RefCell<[u8; 61440]>,
    pub(crate) ioc: RcRefCell<PPURegister>,
    pub(crate) canvas: RcRefCell<Canvas<Window>>,
}

impl PPU {
    pub fn new(
        chr: Vec<u8>,
        ioc: RcRefCell<PPURegister>,
        canvas: RcRefCell<Canvas<Window>>,
    ) -> PPU {
        fn parse_sprite(buffer: &mut [[u8; 0x40]; 0x200], chr: Vec<u8>) {
            for (sprite_idx, sprite) in chr.chunks(16).enumerate() {
                let (pixel0, pixel1) = sprite.split_at(8);
                for (idx, (pix0, pix1)) in pixel0.iter().zip(pixel1).enumerate() {
                    // 上位ビットから値を算出
                    for x in (0..8).rev() {
                        let b = ((pix0 & 2u8.pow(x)) >> x) + 2 * ((pix1 & 2u8.pow(x)) >> x);
                        buffer[sprite_idx][8 * idx + (7 - x) as usize] = b;
                    }
                }
            }
        }
        let state = PPUState::default();
        let buffer = &mut [[0u8; 0x40]; 0x200];
        parse_sprite(buffer, chr);
        PPU {
            pattern0: *buffer,
            state: RefCell::new(state),
            display: RefCell::new([0; 61440]),
            /// I/O CPU Register
            ioc,
            canvas,
        }
    }

    pub fn run(&self, cycle: u32) {
        let state = &self.state;
        let line = state.borrow().line;
        state.borrow_mut().cycle += cycle;

        // 341クロックで1line描写
        if state.borrow().cycle >= 341 {
            state.borrow_mut().line += 1;
            match line {
                0...239 if line % 8 == 0 => self.sprite_generate(),
                262 => state.borrow_mut().line = 0,
                _ => (),
            };

            state.borrow_mut().cycle -= 341;
        }
    }

    pub fn read(&self, adr: u8) {
        let adr = adr as usize;
        if adr < 0x2000usize {
            unimplemented!()
        }
    }

    pub fn sprite_flush(&self) -> Pattern {
        self.pattern0
    }

    pub(crate) fn sprite_generate(&self) {
        static color_palettes: [[u8; 0x03]; 0x40] = [
            [84, 84, 84],
            [0, 30, 116],
            [8, 16, 144],
            [48, 0, 136],
            [68, 0, 100],
            [92, 0, 48],
            [84, 4, 0],
            [60, 24, 0],
            [32, 42, 0],
            [8, 58, 0],
            [0, 64, 0],
            [0, 60, 0],
            [0, 50, 60],
            [0, 0, 0],
            [0, 0, 0],
            [0, 0, 0],
            [152, 150, 152],
            [8, 76, 196],
            [48, 50, 236],
            [92, 30, 228],
            [136, 20, 176],
            [160, 20, 100],
            [152, 34, 32],
            [120, 60, 0],
            [84, 90, 0],
            [40, 114, 0],
            [8, 124, 0],
            [0, 118, 40],
            [0, 102, 120],
            [0, 0, 0],
            [0, 0, 0],
            [0, 0, 0],
            [236, 238, 236],
            [76, 154, 236],
            [120, 124, 236],
            [176, 98, 236],
            [228, 84, 236],
            [236, 88, 180],
            [236, 106, 100],
            [212, 136, 32],
            [160, 170, 0],
            [116, 196, 0],
            [76, 208, 32],
            [56, 204, 108],
            [56, 180, 204],
            [60, 60, 60],
            [0, 0, 0],
            [0, 0, 0],
            [236, 238, 236],
            [168, 204, 236],
            [188, 188, 236],
            [212, 178, 236],
            [236, 174, 236],
            [236, 174, 212],
            [236, 180, 176],
            [228, 196, 144],
            [204, 210, 120],
            [180, 222, 120],
            [168, 226, 144],
            [52, 226, 180],
            [160, 214, 228],
            [160, 162, 160],
            [0, 0, 0],
            [0, 0, 0],
        ];

        const SPRITE_WIDTH: u32 = 32;
        const SPRITE_HEIGHT: u32 = 30;
        const WIDTH: u32 = 8 * SPRITE_WIDTH;
        const HEIGHT: u32 = 8 * SPRITE_HEIGHT;

        let PPUMemory(ref vram) = self.ioc.borrow().ppudata;
        let line = self.state.borrow().line as usize / 8usize;
        let range = line * 8..(line + 1) * 8;

        for line in range {
            // マジックナンバー化してるとこなおす
            for idx in 0..32 {
                let sprite_idx = line * 32 + idx;
                // カラーパレット読んでRGB行列に落とす
                // plt = line->ram->plt
                for (pixel_idx, pixel) in self.pattern0[vram.borrow()[line] as usize]
                    .iter()
                    .enumerate()
                {
                    let sprite_x_idx = sprite_idx % 32;
                    let sprite_y_idx = line;
                    let pixel_x_idx = pixel_idx % 8;
                    let pixel_y_idx = pixel_idx / 8;

                    let offset = sprite_x_idx * 24
                        + sprite_y_idx * 3 * 8 * 32 * 8
                        + pixel_x_idx * 3
                        + pixel_y_idx * 3 * 8 * 32;
                    let color = color_palettes[*pixel as usize];

                    self.display.borrow_mut()[offset] = color[0];
                    self.display.borrow_mut()[offset+1] = color[1];
                    self.display.borrow_mut()[offset|2] = color[2];
                }
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct PPUState {
    cycle: u32,
    line: u32,
}

impl Default for PPUState {
    fn default() -> Self {
        Self { cycle: 0, line: 0 }
    }
}

pub(crate) fn position_map(idx: i32) -> (usize, usize) {
    let x_idx = idx % 32 * 8 * 3;
    let y_idx = idx / 32 * 8 * 3 * 32;
    (x_idx as usize, y_idx as usize)
}
