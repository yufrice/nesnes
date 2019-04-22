use crate::arch::memory::{PPUMemory, PPURegister};
use crate::arch::RcRefCell;
use log::info;
use std::cell::RefCell;

/// Sprite(0x40) x 0x200
pub type Pattern = [[u8; 0x40]; 0x200];

pub(crate) struct PPU {
    /// CHR
    pub(crate) pattern0: Pattern,
    pub(crate) state: RefCell<PPUState>,
    pub(crate) display: Display,
    pub(crate) ioc: RcRefCell<PPURegister>,
}

// 実機はプールしないでレンダリングしてる
// 多分render実装したら消える
pub(crate) struct Display {
    pub(crate) sprite: RefCell<[u8; 0x3C0]>,
    pub(crate) attribute: RefCell<[u8; 240]>,
}

impl Default for Display {
    fn default() -> Self {
        Self {
            sprite: RefCell::new([0x00; 0x3C0]),
            attribute: RefCell::new([0x00; 240]),
        }
    }
}

impl PPU {
    pub fn new(chr: Vec<u8>, ioc: RcRefCell<PPURegister>) -> PPU {
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
            display: Display::default(),
            /// I/O CPU Register
            ioc,
        }
    }

    pub fn run(&self, cycle: u32) {
        let state = &self.state;
        let line = state.borrow().line;
        state.borrow_mut().cycle += cycle;
        match line {
            // background
            // sprites generate
            // 8 x 8 x 8
            0...239 if line % 8 == 0 => self.sprite_generate(),
            // pre render line VBLANk
            240...261 => (),
            _ => (),
        }

        // 341クロックで1line描写
        if state.borrow().cycle >= 341 {
            state.borrow_mut().cycle -= 341;
            state.borrow_mut().line += 1;
        }
    }

    pub fn read(&self, adr: u8) -> &[u8] {
        let adr = adr as usize;
        if adr < 0x2000usize {
            //&self.pattern[adr..(adr + 0xFusize)]
            unimplemented!()
        } else {
            unimplemented!()
        }
    }

    pub fn sprite_flush(&self) -> Pattern {
        self.pattern0
    }

    // generateじゃなくてrender的になるはず
    pub fn sprite_generate(&self) {
        let PPUMemory(ref vram) = self.ioc.borrow().ppudata;
        info!("{}", self.state.borrow().line);
        let line = self.state.borrow().line as usize / 8usize;
        let range = line * 8..(line + 1) * 8;
        for line in range {
            self.display.sprite.borrow_mut()[line] = vram.borrow()[line];
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