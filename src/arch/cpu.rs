use crate::arch::memory::*;
use crate::arch::op::{AddressingMode, OPCode, Operation};
use crate::arch::register::*;
use crate::arch::Opeland;

pub struct CPU {
    pub(crate) register: Register,
    pub(crate) memory: CPUMemory,
}

impl CPU {

    pub(crate) fn fetch(&self) -> u8 {
        let addr = 0x8000u16 + self.register.PC.get();
        self.register.pc_increment();
        self.memory.read(addr as usize)
    }

    pub(crate) fn stack_push(&self) {
        let pc = self.register.PC.get() as u8;
        let count = (0x100 | u16::from(self.register.SP.get())) - 1;
        self.memory.WRAM.borrow_mut()[count as usize] = pc;
        self.register.SP.set(count as u8);

    }

    pub(crate) fn exec(&self, opcode: &Operation) {
        let opeland = self.get_opeland(&opcode.mode);
        match (&opcode.op, opeland) {
            // Flag
            (OPCode::CLC, Opeland::None)
            | (OPCode::SEC, Opeland::None)
            | (OPCode::CLI, Opeland::None)
            | (OPCode::SEI, Opeland::None)
            | (OPCode::CLD, Opeland::None)
            | (OPCode::SED, Opeland::None)
            | (OPCode::CLV, Opeland::None) => self.flag_op(&opcode.op),

            // Aレジスタ Acc
            (OPCode::ADC, opeland)
            | (OPCode::SBC, opeland)
            | (OPCode::ASL, opeland)
            | (OPCode::LSR, opeland)
            | (OPCode::ROL, opeland)
            | (OPCode::ROR, opeland) => self.acc_op(
                &opcode.op,
                match opeland {
                    Opeland::Value(val) => val,
                    Opeland::Address(adr) => self.memory.read(adr as usize),
                    _ => unreachable!(),
                },
            ),

            // compare
            (OPCode::CMP, opeland)
            | (OPCode::CPX, opeland)
            | (OPCode::CPY, opeland) => self.compare_op(
                &opcode.op,
                match opeland {
                    Opeland::Value(val) => val,
                    Opeland::Address(adr) => self.memory.read(adr as usize),
                    _ => unreachable!(),
                },

            ),

            // (in,de) crement
            (OPCode::INC, opeland)
            | (OPCode::DEC, opeland)
            | (OPCode::INX, opeland)
            | (OPCode::INY, opeland)
            | (OPCode::DEY, opeland) => self.register_acc_op(&opcode.op, opeland),

            // Logic
            (OPCode::AND, opeland) | (OPCode::ORA, opeland) | (OPCode::EOR, opeland) => self
                .logic_op(
                    &opcode.op,
                    match opeland {
                        Opeland::Value(val) => val,
                        Opeland::Address(adr) => self.memory.read(adr as usize),
                        _ => unreachable!(),
                    },
                ),

            // Load
            (OPCode::LDA, opeland) | (OPCode::LDX, opeland) | (OPCode::LDY, opeland) => {
                self.load_op(&opcode.op, opeland)
            }

            // Store
            (OPCode::STA, opeland) | (OPCode::STX, opeland) | (OPCode::STY, opeland) => self
                .store_op(
                    &opcode.op,
                    match opeland {
                        Opeland::Address(adr) => adr as usize,
                        _ => unreachable!(),
                    },
                ),

            // Jump
            (OPCode::JMP, opeland)
            | (OPCode::JSR, opeland)
            | (OPCode::RTS, opeland)
            | (OPCode::RTI, opeland) => self.jump_op(
                &opcode.op,
                match opeland {
                    Opeland::Address(adr) => adr,
                    _ => unreachable!(),
                },
            ),

            // Copy
            (OPCode::TAX, Opeland::None)
            | (OPCode::TAY, Opeland::None)
            | (OPCode::TSX, Opeland::None)
            | (OPCode::TXA, Opeland::None)
            | (OPCode::TXS, Opeland::None)
            | (OPCode::TYA, Opeland::None) => self.copy_op(&opcode.op),

            // Branch
            (OPCode::BCC, opeland)
            | (OPCode::BCS, opeland)
            | (OPCode::BEQ, opeland)
            | (OPCode::BMI, opeland)
            | (OPCode::BNE, opeland)
            | (OPCode::BPL, opeland)
            | (OPCode::BVC, opeland)
            | (OPCode::BVS, opeland) => self.branch_op(
                &opcode.op,
                match opeland {
                    Opeland::Address(adr) => adr,
                    _ => unreachable!(),
                },
            ),

            _ => unimplemented!("{:?}", opcode.op),
        }
    }

    pub(crate) fn get_opeland(&self, mode: &AddressingMode) -> Opeland {
        match mode {
            AddressingMode::Implied => Opeland::None,
            AddressingMode::Accumulator => Opeland::Accumulator,
            AddressingMode::ZeroPage => {
                let addr = self.fetch();
                Opeland::Address(u16::from(addr))
            }
            AddressingMode::ZeroPageX => {
                let addr = self.fetch();
                let x = self.register.X.get();
                Opeland::Address(u16::from(addr + x))
            }
            AddressingMode::ZeroPageY => {
                let addr = self.fetch();
                let y = self.register.Y.get();
                Opeland::Address(u16::from(addr + y))
            }
            AddressingMode::Relative => {
                let addr0 = u16::from(self.fetch());
                let addr1 = self.register.PC.get();
                // 補数表現
                let addr = if addr0 < 0x80 {
                    addr0 + addr1
                } else {
                    addr0 + addr1 - 256
                };
                Opeland::Address(addr as u16)
            }
            AddressingMode::Absolute => {
                let addr_low = u16::from(self.fetch());
                let addr_high = u16::from(self.fetch()).rotate_left(8);
                let addr = addr_high + addr_low;
                Opeland::Address(addr as u16)
            }
            AddressingMode::AbsoluteX => {
                let addr_low = u16::from(self.fetch());
                let addr_high = u16::from(self.fetch()).rotate_left(8);
                let x = u16::from(self.register.X.get());
                let addr = addr_high + addr_low + x;
                Opeland::Address(addr as u16)
            }
            AddressingMode::AbsoluteY => {
                let addr_low = u16::from(self.fetch());
                let addr_high = u16::from(self.fetch()).rotate_left(8);
                let y = u16::from(self.register.Y.get());
                let addr = addr_high + addr_low + y;
                Opeland::Address(addr as u16)
            }
            AddressingMode::Indirect => {
                if let Opeland::Value(addr_low) = self.get_opeland(&AddressingMode::Absolute) {
                    let addr_high = u16::from(addr_low).rotate_left(8);
                    let addr_low = u16::from(self.memory.read(addr_low as usize)).rotate_left(8);
                    let addr_high = u16::from(self.memory.read(addr_high as usize));
                    Opeland::Address((addr_high + addr_low) as u16)
                } else {
                    unreachable!()
                }
            }
            AddressingMode::IndirectX => {
                let pre_addr = u16::from(self.fetch());
                let x = u16::from(self.register.X.get());
                let addr_low = pre_addr + x;
                let addr_high = u16::from(self.fetch()).rotate_left(8);
                Opeland::Address(addr_high + addr_low)
            }
            AddressingMode::IndirectY => {
                let pre_addr = u16::from(self.fetch());
                let addr_high = u16::from(self.memory.read(pre_addr as usize)).rotate_left(8);
                let addr_low = u16::from(self.memory.read(pre_addr as usize + 1usize));
                let y = u16::from(self.register.Y.get());
                Opeland::Address(addr_high + addr_low + y)
            }
            AddressingMode::Immediate => Opeland::Value(self.fetch()),
            _ => unreachable!(),
        }
    }
}