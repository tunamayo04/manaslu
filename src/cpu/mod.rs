use crate::cpu::registers::Registers;

pub mod registers;

pub struct CPU {
    _registers: Registers,
}
impl CPU {
    pub fn new() -> Self {
        Self {
            _registers: Registers::new(),
        }
    }

    fn _fetch_next_instruction(&self) -> u8 {
        todo!()
    }
}