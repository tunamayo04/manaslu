use crate::bus::MemoryIndexer;
use crate::cpu::instruction_set::{AddressingMode, Instruction, InstructionSet, Operand};
use crate::cpu::registers::{Flag, Registers};
use crate::utils::math::is_negative;

impl<T: MemoryIndexer> InstructionSet<T> {
    pub(crate) fn iny(instruction: &Instruction<T>, registers: &mut Registers, bus: &mut T) -> Result<u8, String> {
        let result = registers.y.wrapping_add(1);

        registers.set_flag(Flag::Zero, result == 0);
        registers.set_flag(Flag::Negative, is_negative(result));

        registers.y = result;

        Ok(2)
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::instruction_set::InstructionType::{INC, INX, INY};
    use crate::utils::testing::TestBus;
    use super::*;

    #[test]
    fn iny_instruction_positive() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction{
            instruction_type: INY,
            opcode: 0xC8,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: InstructionSet::iny,
        };

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(2));
        assert_eq!(registers.y, 0x01);
        assert_eq!(registers.get_flag(Flag::Zero), false);
        assert_eq!(registers.get_flag(Flag::Negative), false);
    }

    #[test]
    fn iny_instruction_negative() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction{
            instruction_type: INY,
            opcode: 0xC8,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: InstructionSet::iny,
        };

        registers.y = 0x7F;

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(2));
        assert_eq!(registers.y, 0x80);
        assert_eq!(registers.get_flag(Flag::Zero), false);
        assert_eq!(registers.get_flag(Flag::Negative), true);
    }

    #[test]
    fn iny_instruction_zero() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction{
            instruction_type: INY,
            opcode: 0xC8,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: InstructionSet::iny,
        };

        registers.y = 0xFF;

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(2));
        assert_eq!(registers.x, 0);
        assert_eq!(registers.get_flag(Flag::Zero), true);
        assert_eq!(registers.get_flag(Flag::Negative), false);
    }
}