pub mod cpu;
pub mod ppu;
pub mod memory;
pub mod register;
pub mod rom;
pub mod op;

use log::{info};
use std::cell::RefCell;
use std::rc::Rc;

use memory::{CPUMemory, PPURegister};
use {cpu::CPU, ppu::PPU};
use register::Register;

pub(crate) type RcRefCell<T> = Rc<RefCell<T>>;

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
    pub(crate) cpu: CPU,
    pub(crate) ppu: PPU,
}

impl Arch {
    pub fn new(rom: Vec<u8>, chr: Vec<u8>) -> Arch {
        info!("PPU Register init");
        let ppu_reg = Rc::new(RefCell::new(PPURegister::new()));
        info!("CPU Register init");
        let cpu_reg = Register::new();
        info!("Memory init");
        let memory = CPUMemory::new(rom, ppu_reg.clone());

        info!("CPU init");
        let cpu = CPU {
            register: cpu_reg,
            memory: memory,
        };

        info!("PPU init");
        let ppu = PPU::new(chr, ppu_reg);
        info!("Init done");
        Arch {
            cpu: cpu,
            ppu: ppu,
        }
    }

    pub fn frame(&self) {
        let addr = self.cpu.fetch();
        let opecode = op::Operation::new(addr);
        self.cpu.exec(&opecode);
        self.ppu.run(3*opecode.cycle);
    }
}
