use std::cell::RefCell;

pub(crate) struct APU {
    pulse1: RefCell<Pulse>,
    pulse2: RefCell<Pulse>,
    dmc:    RefCell<DMC>,
}

impl Default for APU {
    fn default() -> Self {
        Self {
            pulse1: RefCell::new(Pulse::default()),
            pulse2: RefCell::new(Pulse::default()),
            dmc: RefCell::new(DMC::default()),
        }
    }
}

impl APU {
    pub(crate) fn write(&self, addr: usize, value: u8) {
        match addr {
            0x4000 => self.pulse1.borrow_mut().update_ctrl0(value),
            0x4001 => self.pulse1.borrow_mut().update_ctrl1(value),
            0x4002 => self.pulse1.borrow_mut().timer_low = value,
            0x4003 => unimplemented!(),
            0x4004 => self.pulse2.borrow_mut().update_ctrl0(value),
            0x4005 => self.pulse2.borrow_mut().update_ctrl1(value),
            0x4006 => self.pulse2.borrow_mut().timer_low = value,
            0x4007 ... 0x4008 => unimplemented!(),
            0x4009 => (),
            0x4010 => {
                self.dmc.borrow_mut().is_irq = value & 0b1000_0000 != 0;
                self.dmc.borrow_mut().is_loop = value & 0b0100_0000 != 0;
                self.dmc.borrow_mut().frequency = value & 0b0000_1111;
            },
            0x4011 => self.dmc.borrow_mut().load_counter = value & 0b0111_1111,
            0x4012 => self.dmc.borrow_mut().sample_address = value,
            0x4013 => self.dmc.borrow_mut().sample_length = value,
            0x4014 => unimplemented!(),
            0x4015 => unimplemented!(),
            0x4017 => unimplemented!(),
            _ => unreachable!(),
        }
    }
}

trait Channels {
    fn play();
}

#[derive(Copy, Clone)]
pub(crate) struct Pulse {
    // 0x4000, 0x4004
    duty: u8,
    envelope_loop: bool,
    is_disable_envelope: bool,
    volume: u8,
    // 0x4001, 0x4005
    is_enable_sweep: bool,
    period: u8,
    negate: bool,
    shift: u8,
    // 0x4002, 0x4006
    timer_low: u8,
    // 0x4003, 0x4007
    length_counter: u8,
    timer_high: u8,
}

impl Default for Pulse {
    fn default() -> Self {
        Self {
            // 0x4000, 0x4004
            duty: 0u8,
            envelope_loop: false,
            is_disable_envelope: false,
            volume: 0,
            // 0x4001, 0x4005
            is_enable_sweep: false,
            period: 0,
            negate: false,
            shift: 0,
            // 0x4002, 0x4006
            timer_low: 0,
            // 0x4003, 0x4007
            length_counter: 0,
            timer_high: 0,
        }
    }
}

impl Pulse {
    fn update_ctrl0(&mut self, value: u8) {
        self.duty = value & 0b1100_0000;
        self.envelope_loop = (value & 0b0010_0000) != 0;
        self.is_disable_envelope = (value & 0b0001_0000) != 0;
        self.volume = value & 0b0000_1111;
    }

    fn update_ctrl1(&mut self, value: u8) {
        self.is_enable_sweep = value & 0b1000_0000 != 0;
        self.period = value & 0b0111_0000;
        self.negate = value & 0b0000_1000 != 0;
        self.shift = value  & 0b0000_0111;
    }
}

impl Channels for Pulse {
    fn play() {
    }
}

pub(crate) struct DMC {
    // 0x4010
    is_irq: bool,
    is_loop: bool,
    frequency: u8,
    // 0x4011
    load_counter: u8,
    // 0x4012
    sample_address: u8,
    // 0x4013
    sample_length: u8,
}


impl Default for DMC {
    fn default() -> Self {
        Self {
            is_irq: false,
            is_loop: false,
            frequency: 0,
            load_counter: 0,
            sample_address: 0,
            sample_length: 0,
        }
    }
}
