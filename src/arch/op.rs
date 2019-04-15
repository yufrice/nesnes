use log::info;

use crate::arch::{cpu::CPU, Opeland, register::State, WriteAddr};

#[derive(Debug, PartialEq)]
pub enum OPCode {
    // Arithmetic
    /// Add
    ADC,
    /// Sub
    SBC,
    // Logic
    /// AND
    AND,
    /// OR
    ORA,
    /// XOR
    EOR,
    // Shift
    ASL,
    LSR,
    ROL,
    ROR,
    // branch
    BCC,
    BCS,
    BEQ,
    BNE,
    BVC,
    BVS,
    BPL,
    BMI,
    // bit
    BIT,
    // jump
    JMP,
    JSR,
    RTS,
    // interrupt
    BRK,
    RTI,
    // compare
    CMP,
    CPX,
    CPY,
    INC,
    DEC,
    INX,
    DEX,
    INY,
    DEY,
    // flag
    CLC,
    SEC,
    CLI,
    /// IRQ 禁止
    SEI,
    CLD,
    SED,
    CLV,
    LDA,
    LDX,
    LDY,
    STA,
    STX,
    STY,
    TAX,
    TXA,
    TAY,
    TYA,
    TSX,
    TXS,
    PHA,
    PLA,
    PHP,
    PLP,
    NOP,
}

#[derive(Debug)]
pub(crate) enum AddressingMode {
    /// 副作用を期待
    Implied,
    /// Aレジスタ
    Accumulator,
    /// 次の番地をアドレスとして参照
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    /// 次の番地とPCを加算
    Relative,
    /// 次16bit先のアドレス参照
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    /// 次の番地を参照
    Immediate,
}

#[derive(Debug)]
pub struct Operation {
    pub(crate) op: OPCode,
    pub(crate) mode: AddressingMode,
    pub(crate) cycle: u32,
}

use AddressingMode::*;
use OPCode::*;


impl Operation {
    /// 8bit -> OP/Addressing
    /// あとでhashmapから引くようにする
    pub fn new(op: u8) -> Operation {
        match op {
            // ADC
            0x29 => create(AND, Immediate, 2),
            0x25 => create(AND, ZeroPage, 3),
            0x35 => create(AND, ZeroPageX, 4),
            0x2D => create(AND, Absolute, 4),
            0x3D => create(AND, AbsoluteX, 4),
            0x39 => create(AND, AbsoluteY, 4),
            0x21 => create(AND, Indirect, 6),
            0x31 => create(AND, IndirectY, 5),
            // ASL
            0x06 => create(ASL, ZeroPage, 5),
            0x0A => create(ASL, Accumulator, 2),
            0x0E => create(ASL, Absolute, 6),
            0x16 => create(ASL, ZeroPageX, 6),
            0x1E => create(ASL, AbsoluteX, 7),
            // BIT
            // CMP
            // CPX
            // CPY
            // DEC
            // DEX
            0xCA => create(DEX, Implied, 2),
            // DEY
            0x88 => create(DEY, Implied, 2),
            // EOR
            // INC
            0xE6 => create(INC, ZeroPage, 5),
            0xF6 => create(INC, ZeroPageX, 6),
            0xEE => create(INC, Absolute, 6),
            0xFE => create(INC, AbsoluteX, 7),
            // INX
            0xE8 => create(INX, Implied, 2),
            // INY
            // LSR
            0x09 => create(ORA, Immediate, 2),
            0x05 => create(ORA, ZeroPage, 3),
            0x15 => create(ORA, ZeroPageX, 4),
            0x0D => create(ORA, Absolute, 4),
            0x1D => create(ORA, AbsoluteX, 4),
            0x19 => create(ORA, AbsoluteY, 4),
            0x01 => create(ORA, IndirectX, 6),
            0x11 => create(ORA, IndirectY, 5),
            // ROL
            // ROR
            // SBC

            // Stack
            // PHA
            // PHP
            // PLA
            // PLP

            // Branch
            0x90 => create(BCC, Relative, 2),
            0xB0 => create(BCS, Relative, 2),
            0xF0 => create(BEQ, Relative, 2),
            0x30 => create(BMI, Relative, 2),
            0xD0 => create(BNE, Relative, 2),
            0x10 => create(BPL, Relative, 2),
            0x50 => create(BVC, Relative, 2),
            0x70 => create(BVS, Relative, 2),

            // Jump
            // JMP
            0x4C => create(JMP, Absolute, 3),
            0x6C => create(JMP, Indirect, 5),
            0x20 => create(JSR, Absolute, 6),
            // RTS
            // RTI

            // Load
            // LDA
            0xA1 => create(LDA, IndirectX, 6),
            0xA5 => create(LDA, ZeroPage, 2),
            0xA9 => create(LDA, Immediate, 2),
            0xAD => create(LDA, Absolute, 4),
            0xB1 => create(LDA, IndirectY, 5),
            0xB5 => create(LDA, ZeroPageX, 4),
            0xB9 => create(LDA, AbsoluteY, 4),
            0xBD => create(LDA, AbsoluteX, 4),
            // LDX
            0xA2 => create(LDX, Immediate, 2),
            0xA6 => create(LDX, ZeroPage, 3),
            0xAE => create(LDX, Absolute, 4),
            0xB6 => create(LDX, ZeroPageY, 4),
            0xBE => create(LDX, AbsoluteY, 4),
            // LDY
            0xA0 => create(LDY, Immediate, 2),
            0xA4 => create(LDY, ZeroPage, 3),
            0xB4 => create(LDY, ZeroPageX, 4),
            0xAC => create(LDY, Absolute, 4),
            0xBC => create(LDY, AbsoluteX, 4),
            // STA
            0x85 => create(STA, ZeroPage, 3),
            0x95 => create(STA, ZeroPageX, 4),
            0x8D => create(STA, Absolute, 4),
            0x9D => create(STA, AbsoluteX, 5),
            0x99 => create(STA, AbsoluteY, 5),
            0x81 => create(STA, IndirectX, 6),
            0x91 => create(STA, IndirectY, 6),
            // STX
            // STY
            // TAX
            // TAY
            // TSX
            // TXA
            // TXS
            0x9A => create(TXS, Implied, 2),
            // TYA
            0x98 => create(TYA, Implied, 2),

            // Implied
            0x00 => create(BRK, Implied, 7),
            0x01 => create(NOP, Implied, 2),
            0x18 => create(CLC, Implied, 2),
            0xD8 => create(CLD, Implied, 2),
            0x58 => create(CLI, Implied, 2),
            0xB8 => create(CLV, Implied, 2),
            0x38 => create(SEC, Implied, 2),
            0xF8 => create(SED, Implied, 2),
            0x78 => create(SEI, Implied, 2),
            _ => unimplemented!("0x{:X}", op),
        }
    }
}

impl CPU {
    fn nz_withSet(&self, value: u8, reg: WriteAddr) {
        let zero = value == 0;
        // 補数で負
        let neg = !((value & 0x80).rotate_right(0x80) == 0);
        self.register.P.set(State {
            N: neg,
            V: false,
            B: false,
            I: false,
            Z: zero,
            C: false,
        });
        match reg {
            WriteAddr::A => self.register.A.set(value),
            WriteAddr::X => self.register.X.set(value),
            WriteAddr::Y => self.register.Y.set(value),
            WriteAddr::SP => self.register.SP.set(value),
            WriteAddr::Memory(addr) => self.memory.write(value, addr),
            _ => unreachable!(),
        }
    }

    pub(crate) fn flag_op(&self, op: &OPCode) {
        let ref state = self.register.P;
        match op {
            CLC => state.set(State {
                C: false,
                ..state.get()
            }),
            SEC => state.set(State {
                C: true,
                ..state.get()
            }),
            CLI => state.set(State {
                I: false,
                ..state.get()
            }),
            SEI => state.set(State {
                I: true,
                ..state.get()
            }),
            CLV => state.set(State {
                V: false,
                ..state.get()
            }),
            // Disable
            CLD | SED => (),
            _ => unreachable!(),
        }
    }

    pub(crate) fn logic_op(&self, op: &OPCode, opeland: u8) {
        match op {
            AND => self.nz_withSet(self.register.A.get() & opeland, WriteAddr::A),
            ORA => self.nz_withSet(self.register.A.get() | opeland, WriteAddr::A),
            EOR => self.nz_withSet(self.register.A.get() ^ opeland, WriteAddr::A),
            _ => unreachable!(),
        }
    }

    pub(crate) fn acc_op(&self, op: &OPCode, opeland: u8) {
        let pre_a = self.register.A.get() as u16;
        let opeland = opeland as u16;
        let result = match op {
            ADC => pre_a + opeland + (self.register.P.get().C as u16),
            SBC => pre_a - opeland + (self.register.P.get().C as u16),
            ASL => unimplemented!(),
            LSR => unimplemented!(),
            ROL => unimplemented!(),
            ROR => unimplemented!(),
            _ => unreachable!(),
        };

        // 符号なしオーバーフロー
        let carry = pre_a > 0xFF;
        // 符号ありオーバーフロー
        let overflow = 0 != ((pre_a ^ result) & (opeland ^ result) & 0x80);
        // フローカット
        let result = (result & 0xFF) as u8;

        // 残りの該当フラグを処理してレジスタに格納
        self.nz_withSet(result, WriteAddr::A);
        let ref state = self.register.P;
        state.set(State {
            C: carry,
            V: overflow,
            ..state.get()
        });
    }

    pub(crate) fn register_acc_op(&self, op: &OPCode, opeland: Opeland) {
        let value = match opeland {
            Opeland::Value(value) => value,
            Opeland::Address(adr) => self.memory.read(adr as usize),
            Opeland::None if op == &OPCode::INX  => self.register.X.get(),
            Opeland::None if op == &OPCode::INY  => self.register.Y.get(),
            Opeland::None if op == &OPCode::DEY  => self.register.Y.get(),
            _ => unreachable!(),
        };

        match (op, opeland) {
            (INC, Opeland::Address(addr)) => self.nz_withSet(value+1, WriteAddr::Memory(addr as usize)),
            (DEC, Opeland::Address(addr)) => self.nz_withSet(value-1, WriteAddr::Memory(addr as usize)),
            (INX, Opeland::None)  => self.nz_withSet(value+1, WriteAddr::X),
            (INY, Opeland::None)  => self.nz_withSet(value+1, WriteAddr::Y),
            (DEY, Opeland::None)  => self.nz_withSet(value-1, WriteAddr::Y),
            _ => unreachable!(),
        }
    }

    pub(crate) fn jump_op(&self, op: &OPCode, opeland: usize) {
        match op {
            JMP => self.register.PC.set(opeland as u16),
            JSR => {
                self.stack_push();
            }
            RTS => unimplemented!(),
            RTI => unimplemented!(),
            _ => unreachable!(),
        }
    }

    pub(crate) fn load_op(&self, op: &OPCode, opeland: Opeland) {
        let value = match opeland {
            Opeland::Value(value) => value,
            Opeland::Address(adr) => self.memory.read(adr as usize),
            _ => unreachable!(),
        };

        match op {
            LDA => self.nz_withSet(value, WriteAddr::A),
            LDX => self.nz_withSet(value, WriteAddr::X),
            LDY => self.nz_withSet(value, WriteAddr::Y),
            _ => unreachable!(),
        }
    }

    pub(crate) fn copy_op(&self, op: &OPCode) {
        match op {
            TAX => self.nz_withSet(self.register.A.get(), WriteAddr::X),
            TAY => self.nz_withSet(self.register.A.get(), WriteAddr::Y),
            TSX => self.nz_withSet(self.register.SP.get(), WriteAddr::X),
            TXA => self.nz_withSet(self.register.X.get(), WriteAddr::A),
            TXS => self.nz_withSet(self.register.X.get(), WriteAddr::SP),
            TYA => self.nz_withSet(self.register.Y.get(), WriteAddr::A),
            _ => unreachable!(),
        }
    }

    pub(crate) fn store_op(&self, op: &OPCode, opeland: usize) {
        match op {
            STA => self.memory.write(self.register.A.get(), opeland),
            STX => self.memory.write(self.register.X.get(), opeland),
            STY => self.memory.write(self.register.Y.get(), opeland),
            _ => unreachable!(),
        };
    }

    pub(crate) fn branch_op(&self, op: &OPCode, opeland: u16) {
        match op {
            BCC if self.register.P.get().C == false => self.register.PC.set(opeland),
            BCS if self.register.P.get().C == true => self.register.PC.set(opeland),
            BEQ if self.register.P.get().Z == true => self.register.PC.set(opeland),
            BNE if self.register.P.get().Z == false => self.register.PC.set(opeland),
            BVC if self.register.P.get().V == false => self.register.PC.set(opeland),
            BVS if self.register.P.get().V == true => self.register.PC.set(opeland),
            BPL if self.register.P.get().N == false => self.register.PC.set(opeland),
            BMI if self.register.P.get().N == true => self.register.PC.set(opeland),
            _ => (),
        }
    }
}

fn create(op: OPCode, mode: AddressingMode, cycle: u32) -> Operation {
    Operation {
        op: op,
        mode: mode,
        cycle: cycle,
    }
}