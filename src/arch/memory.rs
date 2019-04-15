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
                    let counter = self.ppu_addr_inc();
                    let addr = ppu_reg.PPUADDR.get();
                    ppu_reg.PPUADDR.set(addr + counter);
                    ppu_reg.PPUDATA.get()
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
                0x2003 => ppu_reg.OAMADDR.set(value),
                0x2004 => {
                    ppu_reg.OAMDATA.set(value);
                    ppu_reg.OAMADDR.set(ppu_reg.OAMADDR.get() + 1);
                }
                0x2005 => ppu_reg.PPUSCROLL.set(value),
                0x2006 => ppu_reg.PPUADDR.set(value),
                0x2007 => {
                    ppu_reg.PPUDATA.set(value);
                    //let counter = self.ppu_addr_inc();
                    let counter = 0;
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

    pub(crate) fn ppu_addr_inc(&self) -> u8 {
        let ppu_reg = self.IOP.borrow();
        if 0 != (ppu_reg.PPUCTRL.get() & 0x02) {
            1
        } else {
            32
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
    /// スプライトデータ
    /// $2004 Read Write  
    /// $2003に送るスプライトを指定  
    pub OAMDATA: Cell<u8>,
    /// しらん
    /// Write
    pub PPUSCROLL: Cell<u8>,
    /// しらん
    /// Write
    pub PPUADDR: Cell<u8>,
    /// しらん
    /// Read Write
    pub PPUDATA: Cell<u8>,
}

impl Default for PPURegister {
    fn default() -> Self {
        Self {
            PPUCTRL: Cell::new(0b01000000),
            PPUMASK: Cell::new(0x00),
            PPUSTATUS: Cell::new(0x00),
            OAMADDR: Cell::new(0x00),
            OAMDATA: Cell::new(0x00),
            PPUSCROLL: Cell::new(0x00),
            PPUADDR: Cell::new(0x00),
            PPUDATA: Cell::new(0x00),
        }
    }
}
