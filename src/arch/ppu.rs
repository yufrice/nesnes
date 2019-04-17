use std::cell::RefCell;

use crate::arch::memory::{PPURegister, PPUMemory};
use crate::arch::RcRefCell;

pub(crate) struct PPU {
    /// CHR
    pub(crate) Pattern: Vec<u8>,
    pub(crate) State: RefCell<PPUState>,
    pub(crate) IOC: RcRefCell<PPURegister>,
}


impl PPU {
    pub fn new(chr: Vec<u8>, ioc: RcRefCell<PPURegister>) -> PPU {
        let state = PPUState::default();
        PPU {
            Pattern: chr,
            State: RefCell::new(state),
            /// I/O CPU Register
            IOC: ioc,
        }
    }

    pub fn run(&self, cycle: u32) {
        let state = &self.State;
        let line = state.borrow().Line;
        state.borrow_mut().Cycle += cycle;
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
        if state.borrow().Cycle >= 341 {
            state.borrow_mut().Cycle -= 341;
            state.borrow_mut().Line += 1;
        }
    }

    pub fn read(&self, adr: u8) -> &[u8] {
        let adr = adr as usize;
        if adr < 0x2000usize {
            &self.Pattern[adr..(adr + 0xFusize)]
        } else {
            unimplemented!()
        }
    }

    pub fn sprite_flush(&self) -> &Vec<u8> {
        &self.Pattern
    }

    pub fn sprite_generate(&self) {
        let PPUMemory(ref vram) = self.IOC.borrow().PPUDATA;
    }
}

#[derive(Debug)]
pub(crate) struct PPUState {
    Cycle: u32,
    Line: u32,
}

impl Default for PPUState {
    fn default() -> Self {
        Self { Cycle: 0, Line: 0 }
    }
}