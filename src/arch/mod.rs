pub mod cpu;
pub mod ppu;
pub mod memory;
pub mod register;
pub mod rom;
pub mod op;

use log::{info};
use std::ops::Add;

pub(crate) enum WriteAddr {
    Memory(usize), A, X, Y, PC, SP, P,
}

#[derive(Debug)]
pub(crate) enum Opeland {
    Address(u32),
    Value(u8),
    Accumulator,
    None,
}

pub struct Arch {
    pub cpu: cpu::CPU,
    pub ppu: ppu::PPU,
}

impl Arch {
    pub fn frame(&self) {
        let addr = self.cpu.fetch();
        let opecode = op::Operation::new(addr);
        self.cpu.exec(&opecode);
        self.ppu.run(3*opecode.cycle);
    }
}
