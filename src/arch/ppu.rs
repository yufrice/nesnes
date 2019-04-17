use std::cell::RefCell;
use crate::arch::memory::PPURegister;
use crate::arch::RcRefCell;

pub(crate) struct PPU {
    /// CHR
    pub(crate) Pattern: Vec<u8>,
    pub(crate) NameTable0: [u8; 0x0400],
    pub(crate) NameTable1: [u8; 0x0400],
    pub(crate) NameTable2: [u8; 0x0400],
    pub(crate) NameTable3: [u8; 0x0400],
    pub(crate) Pallete: [u8; 0x0020],
    pub(crate) State: RefCell<PPUState>,
    pub(crate) IOC: RcRefCell<PPURegister>,
}


impl PPU {
    pub fn new(chr: Vec<u8>, ioc: RcRefCell<PPURegister>) -> PPU {
        let state = PPUState::default();
        PPU {
            Pattern: chr,
            NameTable0: [0x00; 0x0400],
            NameTable1: [0x00; 0x0400],
            NameTable2: [0x00; 0x0400],
            NameTable3: [0x00; 0x0400],
            Pallete: [0x00; 0x0020],
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
            0 => (),
            // pre render line VBLANk
            261 => (),
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