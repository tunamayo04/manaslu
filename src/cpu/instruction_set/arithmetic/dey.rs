use crate::bus::MemoryIndexer;
use crate::cpu::instruction_set::{AddressingMode, Instruction, InstructionSet, Operand};
use crate::cpu::registers::{Flag, Registers};
use crate::utils::math::is_negative;

impl<T: MemoryIndexer> InstructionSet<T> {
    pub(crate) fn dey(instruction: &Instruction<T>, registers: &mut Registers, bus: &mut T) -> Result<u8, String> {
        let result = registers.y.wrapping_sub(1);

        registers.set_flag(Flag::Zero, result == 0);
        registers.set_flag(Flag::Negative, is_negative(result));

        registers.y = result;

        Ok(2)
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::instruction_set::InstructionType::{DEY};
    use crate::utils::testing::TestBus;
    use super::*;

    #[test]
    fn dey_instruction_positive() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction{
            instruction_type: DEY,
            opcode: 0x88,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: InstructionSet::dey,
        };

        registers.y = 0x02;

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(2));
        assert_eq!(registers.y, 0x01);
        assert_eq!(registers.get_flag(Flag::Zero), false);
        assert_eq!(registers.get_flag(Flag::Negative), false);
    }

    #[test]
    fn dey_instruction_negative() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction{
            instruction_type: DEY,
            opcode: 0x88,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: InstructionSet::dey,
        };

        registers.y = 0x00;

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(2));
        assert_eq!(registers.y, 0xFF);
        assert_eq!(registers.get_flag(Flag::Zero), false);
        assert_eq!(registers.get_flag(Flag::Negative), true);
    }

    #[test]
    fn dey_instruction_zero() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction{
            instruction_type: DEY,
            opcode: 0x88,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: InstructionSet::dey,
        };

        registers.y = 0x01;

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(2));
        assert_eq!(registers.y, 0);
        assert_eq!(registers.get_flag(Flag::Zero), true);
        assert_eq!(registers.get_flag(Flag::Negative), false);
    }
}