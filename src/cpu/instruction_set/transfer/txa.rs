use crate::bus::MemoryIndexer;
use crate::cpu::instruction_set::{AddressingMode, Instruction, InstructionSet};
use crate::cpu::registers::{Flag, Registers};
use crate::utils::math::is_negative;

impl<T: MemoryIndexer> InstructionSet<T> {
    pub(crate) fn txa(
        instruction: &Instruction<T>,
        registers: &mut Registers,
        _bus: &mut T,
    ) -> Result<u8, String> {
        registers.accumulator = registers.x;

        registers.set_flag(Flag::Zero, registers.accumulator == 0);
        registers.set_flag(Flag::Negative, is_negative(registers.accumulator));

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
        AND, BIT, CLC, CLD, CLV, ORA, PHA, PHP, TAX, TXA,
    };
    use crate::utils::testing::TestBus;

    #[test]
    fn tax_instruction_positive() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: TXA,
            opcode: 0x8A,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: InstructionSet::txa,
        };

        registers.x = 0x55;

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(2));
        assert_eq!(registers.x, 0x55);
        assert_eq!(registers.accumulator, 0x55);
        assert_eq!(registers.get_flag(Flag::Zero), false);
        assert_eq!(registers.get_flag(Flag::Negative), false);
    }

    #[test]
    fn tax_instruction_negative() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: TXA,
            opcode: 0x8A,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: InstructionSet::txa,
        };

        registers.x = 0xFF;

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(2));
        assert_eq!(registers.x, 0xFF);
        assert_eq!(registers.accumulator, 0xFF);
        assert_eq!(registers.get_flag(Flag::Zero), false);
        assert_eq!(registers.get_flag(Flag::Negative), true);
    }

    #[test]
    fn tax_instruction_zero() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: TXA,
            opcode: 0x8A,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: InstructionSet::txa,
        };

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(2));
        assert_eq!(registers.x, 0);
        assert_eq!(registers.accumulator, 0);
        assert_eq!(registers.get_flag(Flag::Zero), true);
        assert_eq!(registers.get_flag(Flag::Negative), false);
    }
}
