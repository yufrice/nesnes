use std::cell::RefCell;
use log::info;

pub struct PPU {
    /// CHR
    Pattern: Vec<u8>,
    NameTable0: [u8; 0x0400],
    NameTable1: [u8; 0x0400],
    NameTable2: [u8; 0x0400],
    NameTable3: [u8; 0x0400],
    Pallete: [u8; 0x0020],
    State: RefCell<PPUState>,
}


impl PPU {
    pub fn new(chr: Vec<u8>) -> PPU {
        let state = PPUState::new();
        PPU {
            Pattern: chr,
            NameTable0: [0x00; 0x0400],
            NameTable1: [0x00; 0x0400],
            NameTable2: [0x00; 0x0400],
            NameTable3: [0x00; 0x0400],
            Pallete: [0x00; 0x0020],
            State: RefCell::new(state),
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
            _ => unreachable!(),
        }

        // 341クロックで1line描写
        if state.borrow().Cycle >= 341 {
            state.borrow_mut().Cycle -= 341;
            state.borrow_mut().Line += 1;
        }

        info!("{:?}", state.borrow());

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

impl PPUState {
    fn new() -> PPUState {
        PPUState {
            Cycle: 0,
            Line: 0,
        }
    }
}