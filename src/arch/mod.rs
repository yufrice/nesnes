pub mod cpu;
pub mod memory;
pub mod op;
pub mod ppu;
pub mod register;

use log::info;
use std::cell::RefCell;
use std::rc::Rc;
use sdl2::render::Canvas;
use sdl2::video::Window;

use memory::{CPUMemory, PPURegister};

use register::Register;
use {cpu::CPU, ppu::PPU};
pub(crate) type RcRefCell<T> = Rc<RefCell<T>>;

pub(crate) enum WriteAddr {
    Memory(usize),
    None,
    a,
    x,
    y,
    pc,
    sp,
    p,
}

#[derive(Debug)]
pub(crate) enum Opeland {
    Address(u16),
    Value(u8),
    Accumulator,
    None,
}

pub struct Arch {
    pub(crate) cpu: CPU,
    pub(crate) ppu: PPU,
}

impl Arch {
    pub fn new(rom: Vec<u8>, chr: Vec<u8>,canvas: RcRefCell<Canvas<Window>>) -> Arch {
        info!("PPU Register init");
        let ppu_reg = Rc::new(RefCell::new(PPURegister::default()));
        info!("CPU Register init");
        let cpu_reg = Register::default();
        info!("Memory init");
        let memory = CPUMemory::new(rom, ppu_reg.clone());

        info!("CPU init");
        let cpu = CPU {
            register: cpu_reg,
            memory,
        };

        info!("PPU init");
        let ppu = PPU::new(chr, ppu_reg, canvas);
        info!("Init done");
        Arch { cpu, ppu }
    }

    pub fn frame(&self) {
        let addr = self.cpu.fetch();
        let opecode = op::Operation::new(addr);
        info!("{:?}", opecode);
        self.cpu.exec(&opecode);
        self.ppu.run(3 * opecode.cycle);
    }
}
