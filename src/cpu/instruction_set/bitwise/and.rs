use crate::bus::MemoryIndexer;
use crate::cpu::instruction_set::{AddressingMode, Instruction, InstructionSet, Operand};
use crate::cpu::registers::{Flag, Registers};
use crate::utils::math::is_negative;

impl<T: MemoryIndexer> InstructionSet<T> {
    pub(crate) fn and(
        instruction: &Instruction<T>,
        registers: &mut Registers,
        bus: &mut T,
    ) -> Result<u8, String> {
        let operand = match instruction.operand.ok_or("test")? {
            Operand::Address(address) => bus.read_byte(address),
            Operand::Value(value) => value,
        };

        registers.accumulator = registers.accumulator & operand;

        registers.set_flag(Flag::Zero, false);
        registers.set_flag(Flag::Negative, is_negative(registers.accumulator));

        match instruction.addressing_mode {
            AddressingMode::Immediate => Ok(2),
            AddressingMode::ZeroPage => Ok(3),
            AddressingMode::ZeroPageIndexedX => Ok(4),
            AddressingMode::Absolute => Ok(4),
            AddressingMode::AbsoluteIndexedX => Ok(4),
            AddressingMode::AbsoluteIndexedY => Ok(4),
            AddressingMode::IndirectIndexedX => Ok(6),
            AddressingMode::IndirectIndexedY => Ok(5),
            _ => Err(String::from("Invalid addressing mode")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::instruction_set::InstructionType::AND;
    use crate::utils::testing::TestBus;

    #[test]
    fn and_instruction_positive() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: AND,
            opcode: 0x29,
            operand: Some(Operand::Value(0b10101011)),
            addressing_mode: AddressingMode::Immediate,
            operation: InstructionSet::and,
        };

        registers.accumulator = 0b00001001;

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(2));
        assert_eq!(registers.accumulator, 0b00001001);
        assert_eq!(registers.get_flag(Flag::Zero), false);
        assert_eq!(registers.get_flag(Flag::Negative), false);
    }

    #[test]
    fn and_instruction_negative() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: AND,
            opcode: 0x29,
            operand: Some(Operand::Value(0b10101011)),
            addressing_mode: AddressingMode::Immediate,
            operation: InstructionSet::and,
        };

        registers.accumulator = 0b10001001;

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(2));
        assert_eq!(registers.accumulator, 0b10001001);
        assert_eq!(registers.get_flag(Flag::Zero), false);
        assert_eq!(registers.get_flag(Flag::Negative), true);
    }
}
