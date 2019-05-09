use log::info;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use std::cell::RefCell;

use crate::arch::memory::{PPUMemory, PPURegister};
use crate::arch::RcRefCell;
use crate::{
    DISPLAY_HEIGHT, DISPLAY_SIZE, DISPLAY_SPRITE_WIDTH, DISPLAY_WIDTH, PATTERN_LENGTH, SPRITE,
    SPRITE_SIDE,
};

pub type Pattern = [[u8; SPRITE]; PATTERN_LENGTH];

pub(crate) struct PPU {
    /// CHR
    pub(crate) pattern0: Pattern,
    pub(crate) state: RefCell<PPUState>,
    pub(crate) display: RefCell<Texture>,
    pub(crate) ioc: RcRefCell<PPURegister>,
    pub(crate) canvas: RcRefCell<Canvas<Window>>,
}

impl PPU {
    pub fn new(
        chr: Vec<u8>,
        ioc: RcRefCell<PPURegister>,
        canvas: RcRefCell<Canvas<Window>>,
    ) -> PPU {
        fn parse_sprite(buffer: &mut [[u8; SPRITE]; PATTERN_LENGTH], chr: Vec<u8>) {
            // 16bit -> (8bit, 8bit) -> sprite
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
        let buffer = &mut [[0u8; SPRITE]; PATTERN_LENGTH];

        let texture_creator = canvas.borrow().texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(
                PixelFormatEnum::RGB24,
                DISPLAY_WIDTH as u32,
                DISPLAY_HEIGHT as u32,
            )
            .unwrap();

        parse_sprite(buffer, chr);
        PPU {
            pattern0: *buffer,
            state: RefCell::new(state),
            display: RefCell::new(texture),
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
                // 描画
                240 => self.flush_sprite(),
                241 => self.ioc.borrow().set_vblank(),
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
        // RGB 0x40色
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

        let vram = &self.ioc.borrow().ppudata;

        let line = self.state.borrow().line as usize / 8usize;
        self.display
            .borrow_mut()
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for idx in 0..DISPLAY_SPRITE_WIDTH {
                    let sprite_idx = line * DISPLAY_SPRITE_WIDTH + idx;
                    for (pixel_idx, pixel) in self.pattern0[vram.read(sprite_idx + 0x2000) as usize]
                        .iter()
                        .enumerate()
                    {
                        let sprite_x_idx = sprite_idx % DISPLAY_SPRITE_WIDTH;
                        let sprite_y_idx = line;
                        let pixel_x_idx = pixel_idx % SPRITE_SIDE;
                        let pixel_y_idx = pixel_idx / SPRITE_SIDE;

                        let offset = sprite_x_idx * SPRITE_SIDE * 3
                            + sprite_y_idx * 3 * DISPLAY_WIDTH * 8
                            + pixel_x_idx * 3
                            + pixel_y_idx * 3 * SPRITE_SIDE * DISPLAY_SPRITE_WIDTH;

                        // ラインとスプライト位置から属性テーブル参照するようにする
                        let color = color_palettes[*pixel as usize];

                        buffer[offset] = color[0];
                        buffer[offset + 1] = color[1];
                        buffer[offset + 2] = color[2];
                    }
                }
            })
            .unwrap();
    }

    fn flush_sprite(&self) {
        self.canvas
            .borrow_mut()
            .copy(
                &self.display.borrow(),
                None,
                Some(Rect::new(
                    0,
                    0,
                    2 * DISPLAY_WIDTH as u32,
                    2 * DISPLAY_HEIGHT as u32,
                )),
            )
            .unwrap();
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
