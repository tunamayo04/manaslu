pub enum Flag {
    Carry = 0,
    Zero = 1,
    InterruptDisable = 2,
    DecimalMode = 3,
    B = 4,
    Overflow = 6,
    Negative = 7,
}

pub struct Registers {
    pub accumulator: u8,
    pub stack_pointer: u8,
    pub program_counter: u16,
    pub flags: u8,
    pub x: u8,
    pub y: u8,
}
impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}

impl Registers {
    pub fn new() -> Self {
        Self {
            accumulator: 0,
            stack_pointer: 0,
            program_counter: 0,
            flags: 0b0010_0000,
            x: 0,
            y: 0,
        }
    }

    pub fn reset(&mut self) {
        self.accumulator = 0;
        self.stack_pointer = 0;
        self.program_counter = 0;
        self.flags = 0b0010_0000;
        self.x = 0;
        self.y = 0;
    }

    pub fn increment_program_counter(&mut self, amount: u16) {
        self.program_counter = self.program_counter.wrapping_add(amount);
    }

    pub fn set_flag(&mut self, flag: Flag, value: bool) {
        if value {
            self.flags |= 1 << flag as u8;
        } else {
            self.flags &= !(1 << flag as u8);
        }
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        (self.flags & (1 << flag as u8)) != 0
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::registers::{Flag, Registers};

    // ---- Carry Flag ----
    #[test]
    fn reads_carry_flag() {
        let mut registers = Registers::new();
        registers.flags = 0b0000_0001;

        assert_eq!(registers.get_flag(Flag::Carry), true);
    }
    #[test]
    fn writes_carry_flag() {
        let mut registers = Registers::new();
        registers.set_flag(Flag::Carry, true);

        assert_eq!(registers.flags, 0b0010_0001);
    }

    // ---- Zero Flag ----
    #[test]
    fn reads_zero_flag() {
        let mut registers = Registers::new();
        registers.flags = 0b0000_0010;

        assert_eq!(registers.get_flag(Flag::Zero), true);
    }
    #[test]
    fn writes_zero_flag() {
        let mut registers = Registers::new();
        registers.set_flag(Flag::Zero, true);

        assert_eq!(registers.flags, 0b0010_0010);
    }

    // ---- Interrupt Disable Flag ----
    #[test]
    fn reads_interrupt_disable_flag() {
        let mut registers = Registers::new();
        registers.flags = 0b0000_0100;

        assert_eq!(registers.get_flag(Flag::InterruptDisable), true);
    }
    #[test]
    fn writes_interrupt_disable_flag() {
        let mut registers = Registers::new();
        registers.set_flag(Flag::InterruptDisable, true);

        assert_eq!(registers.flags, 0b0010_0100);
    }

    // ---- Decimal Mode Flag ----
    #[test]
    fn reads_decimal_mode_flag() {
        let mut registers = Registers::new();
        registers.flags = 0b0000_1000;

        assert_eq!(registers.get_flag(Flag::DecimalMode), true);
    }
    #[test]
    fn writes_decimal_mode_flag() {
        let mut registers = Registers::new();
        registers.set_flag(Flag::DecimalMode, true);

        assert_eq!(registers.flags, 0b0010_1000);
    }

    // ---- B Flag ----
    #[test]
    fn reads_b_flag() {
        let mut registers = Registers::new();
        registers.flags = 0b0001_0000;

        assert_eq!(registers.get_flag(Flag::B), true);
    }
    #[test]
    fn writes_b_flag() {
        let mut registers = Registers::new();
        registers.set_flag(Flag::B, true);

        assert_eq!(registers.flags, 0b0011_0000);
    }

    // ---- Overflow Flag ----
    #[test]
    fn reads_overflow_flag() {
        let mut registers = Registers::new();
        registers.flags = 0b0100_0000;

        assert_eq!(registers.get_flag(Flag::Overflow), true);
    }
    #[test]
    fn writes_overflow_flag() {
        let mut registers = Registers::new();
        registers.set_flag(Flag::Overflow, true);

        assert_eq!(registers.flags, 0b0110_0000);
    }

    // ---- Negative Flag ----
    #[test]
    fn reads_negative_flag() {
        let mut registers = Registers::new();
        registers.flags = 0b1000_0000;

        assert_eq!(registers.get_flag(Flag::Negative), true);
    }
    #[test]
    fn writes_negative_flag() {
        let mut registers = Registers::new();
        registers.set_flag(Flag::Negative, true);

        assert_eq!(registers.flags, 0b1010_0000);
    }
}
