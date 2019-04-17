use std::ops::Not;
use std::cell::{Cell, RefCell};

use crate::arch::RcRefCell;

/// WRAM: 2KByte
/// IOP: PPU I/O
/// IOA: APU I/O
/// APU, PAD
/// ROM: 32KByte
///
/// ROMも同様
pub struct CPUMemory {
    /// 2KB WRAM
    pub(crate) WRAM: RefCell<[u8; 0x0800]>,
    /// PPUレジスタ
    pub(crate) IOP: RcRefCell<PPURegister>,
    /// APU, PAD
    pub(crate) IOA: [u8; 0x0020],
    /// ROMプログラム部
    pub(crate) PRG_ROM: Vec<u8>,
}

impl CPUMemory {
    pub(crate) fn new(rom: Vec<u8>, prg: RcRefCell<PPURegister>) -> CPUMemory {
        CPUMemory {
            WRAM: RefCell::new([0x00; 0x0800]),
            IOP: prg,
            IOA: [0x00; 0x0020],
            PRG_ROM: rom,
        }
    }

    pub(crate) fn read(&self, addr: usize) -> u8 {
        // WRAM
        if addr < 0x0800usize {
            self.WRAM.borrow()[addr]
        // WRAM mirror
        // addr - 0x0800
        } else if addr < 0x2000usize {
            let addr = addr - 0x2000;
            self.WRAM.borrow()[addr]
        // PPU
        } else if addr < 0x2008usize {
            let ppu_reg = &mut self.IOP.borrow_mut();
            match addr {
                0x2000 => unreachable!(),
                0x2001 => unreachable!(),
                0x2002 => {
                    ppu_reg.PPUSCROLL.set(0x00);
                    ppu_reg.PPUSTATUS.get()
                }
                0x2003 => unreachable!(),
                0x2004 => ppu_reg.OAMDATA.get(),
                0x2005 => unreachable!(),
                0x2006 => unreachable!(),
                0x2007 => {
                    let counter = if 0 != (ppu_reg.PPUCTRL.get() & 0x02) {
                        1
                    } else {
                        32
                    };
                    let addr = ppu_reg.PPUADDR.get();
                    ppu_reg.PPUADDR.set(addr + counter);
                    ppu_reg.PPUDATA.read(usize::from(addr))
                }
                _ => unreachable!(),
            }
        } else if addr < 0x4000usize {
            unreachable!()
        // APU, PAD
        } else if addr < 0x4020usize {
            unimplemented!("APU, PAD")
        // Expand ROM
        } else if addr < 0x6000usize {
            unimplemented!("Ex ROM")
        // Expand RAM
        } else if addr < 0x8000usize {
            unimplemented!("Ex RAM")
        // ROM
        } else if addr < 0x10000usize {
            let addr = addr - 0x8000usize;
            self.PRG_ROM[addr]
        } else {
            unreachable!("Out of Memory")
        }
    }

    pub(crate) fn write(&self, value: u8, addr: usize) {
        // WRAM
        if addr < 0x0800usize {
            let ram = &mut self.WRAM.borrow_mut();
            ram[addr] = value;
        // WRAM mirror
        // addr - 0x0800
        } else if addr < 0x2000usize {
            let addr = addr - 0x2000usize;
            let ram = &mut self.WRAM.borrow_mut();
            ram[addr] = value;
        // PPU
        } else if addr < 0x2008usize {
            let ppu_reg = &mut self.IOP.borrow_mut();
            match addr {
                0x2000 => ppu_reg.PPUCTRL.set(value),
                0x2001 => ppu_reg.PPUMASK.set(value),
                0x2002 => unreachable!(),
                0x2003 => {
                ppu_reg.oamaddr_bit_flag.set(!ppu_reg.oamaddr_bit_flag.get());
                ppu_reg.OAMADDR.set(value);
                }
                0x2004 => {
                    let pre_adr = ppu_reg.oamaddr_bit_flag.get();
                    // ToDo u16空間用意してhighはシフト
                    let value = match ppu_reg.oamaddr_bit_flag.get() {
                        BitFlag::Low => value,
                        BitFlag::High => value,
                    };
                    ppu_reg.OAMDATA.set(value);
                    ppu_reg.OAMADDR.set(ppu_reg.OAMADDR.get() + 1);
                }
                0x2005 => ppu_reg.PPUSCROLL.set(value),
                0x2006 => {
                    let pre_adr = ppu_reg.PPUADDR.get();
                    let value = match ppu_reg.ppuaddr_bit_flag.get() {
                        BitFlag::Low => pre_adr + u16::from(value),
                        BitFlag::High => u16::from(value).rotate_left(8),
                    };
                    ppu_reg.PPUADDR.set(value);
                    ppu_reg.ppuaddr_bit_flag.set(!ppu_reg.ppuaddr_bit_flag.get());
                },
                0x2007 => {
                    let addr = ppu_reg.PPUADDR.get();
                    ppu_reg.PPUDATA.write(usize::from(addr), value);

                    let counter = if 0 != (ppu_reg.PPUCTRL.get() & 0x02) {
                        1
                    } else {
                        32
                    };

                    let addr = ppu_reg.PPUADDR.get();
                    ppu_reg.PPUADDR.set(addr + counter);
                }
                _ => unreachable!(),
            };
        } else if addr < 0x4000usize {
            unreachable!()
        // APU, PAD
        } else if addr < 0x4020usize {
            unimplemented!("APU, PAD")
        // Expand ROM
        } else {
            unimplemented!()
        }
    }
}

// VRAM E0117
pub(crate) struct PPUMemory(pub(crate) RefCell<[u8; 0x4000]>);

impl std::fmt::Debug for PPUMemory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "VRAM")
    }
}

impl Default for PPUMemory {
    fn default() -> Self {
        Self(RefCell::new([0x00; 0x4000]))
    }
}

impl PPUMemory {
    pub(crate) fn read(&self, addr: usize) -> u8 {
        let PPUMemory(vram) = self;

        let addr = match addr {
            0x0000 ... 0x27C0 => addr,
            0x2800 ... 0x2FBF => addr - 0x0800,
            0x2FC0 ... 0x2FFF => addr - 0x08C0,
            0x3000 ... 0x3EFF => unreachable!(),
            0x3F00 ... 0x3F1F => addr,
            _ => unreachable!(),
        };

        vram.borrow()[addr]
    }

    pub(crate) fn write(&self, addr: usize, value: u8) {
        let PPUMemory(vram) = self;

        let addr = match addr {
            0x0000 ... 0x27C0 => addr,
            0x2800 ... 0x2FBF => addr - 0x0800,
            0x2FC0 ... 0x2FFF => addr - 0x08C0,
            0x3000 ... 0x3EFF => unreachable!(),
            0x3F00 ... 0x3F1F => addr,
            0x3F20 ... 0x3FFF => addr - 0x20,
            0x4000 ... 0xFFFF => addr - 0xC00,
            _ => unreachable!("PPU Write Address: {:X}", addr),
        };

        vram.borrow_mut()[addr] = value;
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum BitFlag {
    Low,
    High,
}

impl Not for BitFlag {
    type Output = BitFlag;

    fn not(self) -> BitFlag {
        match self {
            BitFlag::Low => BitFlag::High,
            BitFlag::High => BitFlag::Low,
        }
    }
}

/// PPU Register
#[derive(Debug)]
pub(crate) struct PPURegister {
    /// コントロールレジスタ  
    /// $2000 Write
    /// - 7 NMI enable
    /// - 6 PPU master/slave
    /// - 5 sprite height
    /// - 4 background tile select
    /// - 3 sprite tile select
    /// - 2 incremenet mode
    /// - 1-0 nametable select
    pub PPUCTRL: Cell<u8>,
    /// マスクレジスタ  
    /// $2001 Write
    /// - 7-5 background color (BGR)
    /// - 4 sprite enable
    /// - 3 background enable
    /// - 2 sprite left column enable
    /// - 1 background left column enable
    /// - 0 greyscale/color
    pub PPUMASK: Cell<u8>,
    /// ステータスレジスタ  
    /// $2002 Read
    /// - 7 vblank
    /// - 6 sprite hit
    /// - 5 sprite overflow
    /// - 4-0 disable
    pub PPUSTATUS: Cell<u8>,
    /// スプライトメモリアドレス  
    /// $2003 Write  
    /// スプライトの書き込み先
    pub OAMADDR: Cell<u8>,
    pub oamaddr_bit_flag: Cell<BitFlag>,
    /// スプライトデータ
    /// $2004 Read Write  
    /// $2003に送るスプライトを指定  
    pub OAMDATA: Cell<u8>,
    /// しらん
    /// Write
    pub PPUSCROLL: Cell<u8>,
    /// Write VRAM Address
    /// High/Low Address
    pub PPUADDR: Cell<u16>,
    pub ppuaddr_bit_flag: Cell<BitFlag>,
    /// Read Write VRAM
    pub PPUDATA: PPUMemory,
}

impl Default for PPURegister {
    fn default() -> Self {
        Self {
            PPUCTRL: Cell::new(0b0100_0000),
            PPUMASK: Cell::new(0x00),
            PPUSTATUS: Cell::new(0x00),
            OAMADDR: Cell::new(0x00),
            oamaddr_bit_flag: Cell::new(BitFlag::High),
            OAMDATA: Cell::new(0x00),
            PPUSCROLL: Cell::new(0x00),
            PPUADDR: Cell::new(0x00),
            ppuaddr_bit_flag: Cell::new(BitFlag::High),
            PPUDATA: PPUMemory::default(),
        }
    }
}
