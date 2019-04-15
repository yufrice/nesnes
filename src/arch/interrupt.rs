use crate::arch::cpu::*;

impl CPU {
  fn nmi(&self) -> Result<(), &'static str> {
    self.register.P.borrow_mut().nmi();
    Err("")
  }
}
