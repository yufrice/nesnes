use std::cell::Cell;


/// CPU内レジスタ
#[derive(Debug)]
pub struct Register {
  /// Accumulator
  pub(crate) A: Cell<u8>,
  /// Indexes
  pub(crate) X: Cell<u8>,
  pub(crate) Y: Cell<u8>,
  /// Program Counter
  pub(crate) PC: Cell<u16>,
  /// Stack Pointer
  pub(crate) SP: Cell<u8>,
  /// Statuc Register
  pub(crate) P: Cell<State>,
}

impl Default for Register {
  fn default() -> Self {
    let state = State::default();
    Self {
      A: Cell::new(0x00),
      X: Cell::new(0x00),
      Y: Cell::new(0x00),
      PC: Cell::new(0x0000),
      SP: Cell::new(0x00),
      P: Cell::new(state),
    }
  }
}

impl Register {
  pub(crate) fn pc_increment(&self) {
    self.PC.set(1 + self.PC.get());
  }

  pub(crate) fn soft_reset(&self) {
    self.P.set(
      State::default()
    );
  }

  // interrupt signal
  pub(crate) fn nmi(&self) {
    let state = &self.P;
    state.set(State {
      I: true,
      B: false,
      ..state.get()
    })
  }

  pub(crate) fn hard_reset(&self) {
    let state = &self.P;
    state.set(State {
      I: true,
      ..state.get()
    })
  }

  pub(crate) fn irq(&self) {
    let state = &self.P;
    let brk = !state.get().B;
    state.set(State {
      I: true,
      B: brk,
      ..state.get()
    })
  }
}

#[derive(Clone, Copy, Debug)]
pub struct State {
  pub(crate) N: bool,
  pub(crate) V: bool,
  // R: (),
  pub(crate) B: bool,
  // D,
  pub(crate) I: bool,
  pub(crate) Z: bool,
  pub(crate) C: bool,
}

impl Default for State {
  fn default() -> Self {
    Self {
      N: false,
      V: false,
      B: true,
      I: true,
      Z: false,
      C: false,
    }
  }
}