pub mod cpu;
pub mod memory;
pub mod op;
pub mod ppu;
pub mod register;

use log::info;
use memory::{CPUMemory, PPURegister};
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::cell::RefCell;
use std::rc::Rc;

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
    pub fn new(rom: Vec<u8>, chr: Vec<u8>, canvas: RcRefCell<Canvas<Window>>) -> Arch {
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
        // info!("{:?}", opecode);
        self.cpu.exec(&opecode);
        self.ppu.run(3 * opecode.cycle);
    }
}

pub trait Accumulate {
    fn calc_add(&self, rhs: Self) -> Self;
    fn calc_sub(&self, rhs: Self) -> Self;
}

impl Accumulate for u8 {
    fn calc_add(&self, rhs: u8) -> u8 {
        self.checked_add(rhs).unwrap_or(0)
    }

    fn calc_sub(&self, rhs: u8) -> u8 {
        self.checked_sub(rhs).unwrap_or(u8::max_value())
    }
}

#[cfg(test)]
mod tests {
    use crate::arch::Accumulate;

    #[test]
    fn is_u8_add_flow() {
        let lhs = u8::max_value();
        assert_eq!(lhs.calc_add(1u8), 0u8);
    }

    #[test]
    fn is_not_u8_add_flow() {
        let lhs = 0u8;
        assert_eq!(lhs.calc_add(1u8), 1u8);
    }

    #[test]
    fn is_u8_sub_flow() {
        let lhs = 0u8;
        assert_eq!(lhs.calc_sub(1), u8::max_value());
    }

    #[test]
    fn is_not_u8_sub_flow() {
        let lhs = 10u8;
        assert_eq!(lhs.calc_sub(1), 9u8);
    }
}
