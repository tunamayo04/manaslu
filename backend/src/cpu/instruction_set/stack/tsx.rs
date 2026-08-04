use crate::bus::SerialInterface;
use crate::cpu::instruction_set::{AddressingMode, Instruction, InstructionSet};
use crate::cpu::registers::{Flag, Registers};
use crate::utils::math::is_negative;

impl<T: SerialInterface> InstructionSet<T> {
    pub(crate) fn tsx(
        instruction: &Instruction<T>,
        registers: &mut Registers,
        _bus: &mut T,
    ) -> Result<u8, String> {
        registers.x = registers.stack_pointer;

        registers.set_flag(Flag::Negative, is_negative(registers.x));
        registers.set_flag(Flag::Zero, registers.stack_pointer == 0);

        match instruction.addressing_mode {
            AddressingMode::Implicit => Ok(2),
            _ => Err(String::from("Invalid addressing mode")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::instruction_set::InstructionType::{
        AND, BIT, CLC, CLD, CLV, ORA, PHA, PHP, TSX,
    };
    use crate::utils::testing::TestBus;

    #[test]
    fn tsx_instruction_no_flag() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: TSX,
            opcode: 0xBA,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: InstructionSet::tsx,
        };

        registers.stack_pointer = 0xB6;

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(2));
        assert_eq!(registers.x, 0xB6);
        assert_eq!(registers.get_flag(Flag::Negative), true);
        assert_eq!(registers.get_flag(Flag::Zero), false);
    }
}
