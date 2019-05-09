use std::cell::Cell;

/// CPU内レジスタ
#[derive(Debug)]
pub struct Register {
    /// Accumulator
    pub(crate) a: Cell<u8>,
    /// Indexes
    pub(crate) x: Cell<u8>,
    pub(crate) y: Cell<u8>,
    /// Program Counter
    pub(crate) pc: Cell<u16>,
    /// Stack Pointer
    pub(crate) sp: Cell<u8>,
    /// Statuc register
    pub(crate) p: Cell<State>,
}

impl Default for Register {
    fn default() -> Self {
        let state = State::default();
        Self {
            a: Cell::new(0x00),
            x: Cell::new(0x00),
            y: Cell::new(0x00),
            pc: Cell::new(0x0000),
            sp: Cell::new(0x00),
            p: Cell::new(state),
        }
    }
}

impl Register {
    pub(crate) fn pc_increment(&self) {
        self.pc.set(1 + self.pc.get());
    }

    pub(crate) fn sp_increment(&self) {
        self.sp.set(1 + self.sp.get());
    }

    pub(crate) fn sp_decrement(&self) {
        self.sp.set(self.sp.get() - 1);
    }

    pub(crate) fn soft_reset(&self) {
        self.p.set(State::default());
    }

    // interrupt signal
    pub(crate) fn nmi(&self) {
        let state = &self.p;
        state.set(State {
            i: true,
            b: false,
            ..state.get()
        })
    }

    pub(crate) fn hard_reset(&self) {
        let state = &self.p;
        state.set(State {
            i: true,
            ..state.get()
        })
    }

    pub(crate) fn irq(&self) {
        let state = &self.p;
        let brk = !state.get().b;
        state.set(State {
            i: true,
            b: brk,
            ..state.get()
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct State {
    pub(crate) n: bool,
    pub(crate) v: bool,
    // R: (),
    pub(crate) b: bool,
    // D,
    pub(crate) i: bool,
    pub(crate) z: bool,
    pub(crate) c: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            n: false,
            v: false,
            b: true,
            i: true,
            z: false,
            c: false,
        }
    }
}
