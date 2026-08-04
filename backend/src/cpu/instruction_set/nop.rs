use crate::bus::SerialInterface;
use crate::cpu::instruction_set::{Instruction, InstructionSet};
use crate::cpu::registers::Registers;

impl<T: SerialInterface> InstructionSet<T> {
    pub fn nop(
        _instruction: &Instruction<T>,
        _registers: &mut Registers,
        _bus: &mut T,
    ) -> Result<u8, String> {
        Ok(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::instruction_set::{AddressingMode, InstructionType};
    use crate::utils::testing::TestBus;

    #[test]
    fn nop_returns_2_cycles() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: InstructionType::NOP,
            opcode: 0x00,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: InstructionSet::nop,
        };

        // Act
        let cycles = (instruction.operation)(&instruction, &mut registers, &mut test_bus).unwrap();

        // Assert
        assert_eq!(cycles, 2);
    }
}
