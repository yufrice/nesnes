use crate::arch::{cpu::CPU, register::State, Accumulate, Opeland, WriteAddr};

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
            0x69 => create(ADC, Immediate, 2),
            0x65 => create(ADC, ZeroPage, 3),
            0x75 => create(ADC, ZeroPageX, 4),
            0x6D => create(ADC, Absolute, 4),
            0x7D => create(ADC, AbsoluteX, 4),
            0x79 => create(ADC, AbsoluteY, 4),
            0x61 => create(ADC, IndirectX, 6),
            0x71 => create(ADC, IndirectY, 5),

            0x29 => create(AND, Immediate, 2),
            0x25 => create(AND, ZeroPage, 3),
            0x35 => create(AND, ZeroPageX, 4),
            0x2D => create(AND, Absolute, 4),
            0x3D => create(AND, AbsoluteX, 4),
            0x39 => create(AND, AbsoluteY, 4),
            0x21 => create(AND, Indirect, 6),
            0x31 => create(AND, IndirectY, 5),
            0x06 => create(ASL, ZeroPage, 5),
            0x0A => create(ASL, Accumulator, 2),
            0x0E => create(ASL, Absolute, 6),
            0x16 => create(ASL, ZeroPageX, 6),
            0x1E => create(ASL, AbsoluteX, 7),
            // BIT
            0xC9 => create(CMP, Immediate, 2),
            0xC5 => create(CMP, ZeroPage, 3),
            0xD5 => create(CMP, ZeroPageX, 4),
            0xCD => create(CMP, Absolute, 4),
            0xDD => create(CMP, AbsoluteX, 4),
            0xD9 => create(CMP, AbsoluteY, 4),
            0xC1 => create(CMP, IndirectX, 6),
            0xD1 => create(CMP, IndirectY, 5),
            0xE0 => create(CPX, Immediate, 2),
            0xE4 => create(CPX, ZeroPage, 3),
            0xEC => create(CPX, Absolute, 4),
            0xC0 => create(CPY, Immediate, 2),
            0xC4 => create(CPY, ZeroPage, 3),
            0xCC => create(CPY, Absolute, 4),
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
            0xE9 => create(SBC, Immediate, 2),
            0xE5 => create(SBC, ZeroPage, 3),
            0xF5 => create(SBC, ZeroPageX, 4),
            0xED => create(SBC, Absolute, 4),
            0xFD => create(SBC, AbsoluteX, 4),
            0xF9 => create(SBC, AbsoluteY, 4),
            0xE1 => create(SBC, IndirectX, 6),
            0xF1 => create(SBC, IndirectY, 5),

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
            0x60 => create(RTS, Implied, 6),
            // RTI

            // Load
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
            // STA周り
            0x85 => create(STA, ZeroPage, 3),
            0x95 => create(STA, ZeroPageX, 4),
            0x8D => create(STA, Absolute, 4),
            0x9D => create(STA, AbsoluteX, 5),
            0x99 => create(STA, AbsoluteY, 5),
            0x81 => create(STA, IndirectX, 6),
            0x91 => create(STA, IndirectY, 6),
            0x86 => create(STX, ZeroPage, 3),
            0x96 => create(STX, ZeroPageY, 4),
            0x8E => create(STX, Absolute, 4),
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
            0xEA => create(NOP, Implied, 2),
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
    fn nz_withSet(&self, value: u8, addr: WriteAddr) {
        let zero = value == 0;
        // 補数で負
        let neg = (value & 0x80).rotate_right(0x80) != 0;
        self.register.p.set(State {
            n: neg,
            v: false,
            b: false,
            i: false,
            z: zero,
            c: false,
        });
        match addr {
            WriteAddr::a => self.register.a.set(value),
            WriteAddr::x => self.register.x.set(value),
            WriteAddr::y => self.register.y.set(value),
            WriteAddr::sp => self.register.sp.set(value),
            WriteAddr::Memory(addr) => self.memory.write(value, addr),
            WriteAddr::None => (),
            _ => unreachable!(),
        }
    }

    pub(crate) fn nzc_withSet(&self, result: u16, addr: WriteAddr) {
        // 符号なしオーバーフロー
        let carry = result > 0xFF;
        // フローカット
        let result = result & 0xFF;

        // 残りの該当フラグを処理してレジスタに格納
        self.nz_withSet(result as u8, addr);
        let state = &self.register.p;
        state.set(State {
            c: carry,
            ..state.get()
        });
    }

    pub(crate) fn nvzc_withSet(&self, pre: u16, rhs: u16, result: u16, addr: WriteAddr) {
        // 符号ありオーバーフロー
        let overflow = 0 != ((pre ^ result) & (rhs ^ result) & 0x80);
        // フローカット
        let result = result & 0xFF;

        // 残りの該当フラグを処理してレジスタに格納
        self.nzc_withSet(result, addr);
        let state = &self.register.p;
        state.set(State {
            v: overflow,
            ..state.get()
        });
    }

    pub(crate) fn flag_op(&self, op: &OPCode) {
        let state = &self.register.p;
        match op {
            CLC => state.set(State {
                c: false,
                ..state.get()
            }),
            SEC => state.set(State {
                c: true,
                ..state.get()
            }),
            CLI => state.set(State {
                i: false,
                ..state.get()
            }),
            SEI => state.set(State {
                i: true,
                ..state.get()
            }),
            CLV => state.set(State {
                v: false,
                ..state.get()
            }),
            // Disable
            CLD | SED => (),
            _ => unreachable!(),
        }
    }

    pub(crate) fn logic_op(&self, op: &OPCode, opeland: u8) {
        match op {
            AND => self.nz_withSet(self.register.a.get() & opeland, WriteAddr::a),
            ORA => self.nz_withSet(self.register.a.get() | opeland, WriteAddr::a),
            EOR => self.nz_withSet(self.register.a.get() ^ opeland, WriteAddr::a),
            _ => unreachable!(),
        }
    }

    pub(crate) fn compare_op(&self, op: &OPCode, opeland: u8) {
        let lhs = match op {
            CMP => self.register.a.get(),
            CPX => self.register.x.get(),
            CPY => self.register.y.get(),
            _ => unreachable!(),
        };

        let result = (i16::from(lhs) - i16::from(opeland)) as u16;

        self.nzc_withSet(result, WriteAddr::None);
    }

    pub(crate) fn acc_op(&self, op: &OPCode, opeland: u8) {
        let pre_a = u16::from(self.register.a.get());
        let opeland = u16::from(opeland);
        let result = match op {
            ADC => pre_a + opeland + (self.register.p.get().c as u16),
            SBC => pre_a - opeland + (self.register.p.get().c as u16),
            ASL => unimplemented!(),
            LSR => unimplemented!(),
            ROL => unimplemented!(),
            ROR => unimplemented!(),
            _ => unreachable!(),
        };

        self.nvzc_withSet(pre_a, opeland, result, WriteAddr::a);
    }

    pub(crate) fn register_acc_op(&self, op: &OPCode, opeland: Opeland) {
        let value = match opeland {
            Opeland::Value(value) => value,
            Opeland::Address(adr) => self.memory.read(adr as usize),
            Opeland::None if op == &OPCode::INX => self.register.x.get(),
            Opeland::None if op == &OPCode::DEX => self.register.x.get(),
            Opeland::None if op == &OPCode::INY => self.register.y.get(),
            Opeland::None if op == &OPCode::DEY => self.register.y.get(),
            _ => unreachable!(),
        };

        match (op, opeland) {
            (INC, Opeland::Address(addr)) => {
                self.nz_withSet(value + 1, WriteAddr::Memory(addr as usize))
            }
            (DEC, Opeland::Address(addr)) => {
                self.nz_withSet(value - 1, WriteAddr::Memory(addr as usize))
            }
            (INX, Opeland::None) => self.nz_withSet(value.calc_add(1), WriteAddr::x),
            (DEX, Opeland::None) => self.nz_withSet(value.calc_sub(1), WriteAddr::x),
            (INY, Opeland::None) => self.nz_withSet(value.calc_add(1), WriteAddr::y),
            (DEY, Opeland::None) => self.nz_withSet(value.calc_sub(1), WriteAddr::y),
            _ => unreachable!(),
        }
    }

    pub(crate) fn jump_op(&self, op: &OPCode, opeland: u16) {
        self.register.soft_reset();
        let addr = opeland - 0x8000;
        match op {
            JMP => self.register.pc.set(addr),
            JSR => {
                let pc = self.register.pc.get() - 1;
                let pc_high = (pc >> 8) as u8 & 0xFF;
                let pc_low = (pc & 0xFF) as u8;
                self.stack_push(pc_high);
                self.stack_push(pc_low);
                self.register.pc.set(addr);
            }
            _ => unreachable!(),
        }
    }

    pub(crate) fn rt_op(&self, op: &OPCode) {
        let pc_low = u16::from(self.stack_pop());
        let pc_high = u16::from(self.stack_pop()) << 8;
        self.register.pc.set(pc_low + pc_high)
    }

    pub(crate) fn load_op(&self, op: &OPCode, opeland: Opeland) {
        let value = match opeland {
            Opeland::Value(value) => value,
            Opeland::Address(adr) => self.memory.read(adr as usize),
            _ => unreachable!(),
        };

        match op {
            LDA => self.nz_withSet(value, WriteAddr::a),
            LDX => self.nz_withSet(value, WriteAddr::x),
            LDY => self.nz_withSet(value, WriteAddr::y),
            _ => unreachable!(),
        }
    }

    pub(crate) fn copy_op(&self, op: &OPCode) {
        match op {
            TAX => self.nz_withSet(self.register.a.get(), WriteAddr::x),
            TAY => self.nz_withSet(self.register.a.get(), WriteAddr::y),
            TSX => self.nz_withSet(self.register.sp.get(), WriteAddr::x),
            TXA => self.nz_withSet(self.register.x.get(), WriteAddr::a),
            TXS => self.nz_withSet(self.register.x.get(), WriteAddr::sp),
            TYA => self.nz_withSet(self.register.y.get(), WriteAddr::a),
            _ => unreachable!(),
        }
    }

    pub(crate) fn store_op(&self, op: &OPCode, opeland: usize) {
        match op {
            STA => self.memory.write(self.register.a.get(), opeland),
            STX => self.memory.write(self.register.x.get(), opeland),
            STY => self.memory.write(self.register.y.get(), opeland),
            _ => unreachable!(),
        };
        self.register.soft_reset();
    }

    pub(crate) fn branch_op(&self, op: &OPCode, opeland: u16) {
        match op {
            BCC if !self.register.p.get().c => self.register.pc.set(opeland),
            BCS if self.register.p.get().c => self.register.pc.set(opeland),
            BEQ if self.register.p.get().z => self.register.pc.set(opeland),
            BNE if !self.register.p.get().z => self.register.pc.set(opeland),
            BVC if !self.register.p.get().v => self.register.pc.set(opeland),
            BVS if !self.register.p.get().v => self.register.pc.set(opeland),
            BPL if !self.register.p.get().n => self.register.pc.set(opeland),
            BMI if self.register.p.get().n => self.register.pc.set(opeland),
            _ => (),
        };
        self.register.soft_reset();
    }
}

fn create(op: OPCode, mode: AddressingMode, cycle: u32) -> Operation {
    Operation { op, mode, cycle }
}
