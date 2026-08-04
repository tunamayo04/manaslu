use crate::bus::MemoryIndexer;
use crate::cpu::instruction_set::{AddressingMode, Instruction, InstructionSet, Operand};
use crate::cpu::registers::{Flag, Registers};
use crate::utils::math::is_negative;

impl<T: MemoryIndexer> InstructionSet<T> {
    pub(crate) fn eor(
        instruction: &Instruction<T>,
        registers: &mut Registers,
        bus: &mut T,
    ) -> Result<u8, String> {
        let operand = match instruction.operand.ok_or("Missing operand")? {
            Operand::Address(address) => bus.read_byte(address),
            Operand::Value(value) => value,
        };

        registers.accumulator ^= operand;

        registers.set_flag(Flag::Zero, registers.accumulator == 0);
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
    use crate::cpu::instruction_set::InstructionType::{AND, EOR, ORA};
    use crate::utils::testing::TestBus;

    #[test]
    fn eor_instruction_positive() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: EOR,
            opcode: 0x49,
            operand: Some(Operand::Value(0b00001111)),
            addressing_mode: AddressingMode::Immediate,
            operation: InstructionSet::eor,
        };

        registers.accumulator = 0b0000011;

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(2));
        assert_eq!(registers.accumulator, 0b00001100);
        assert_eq!(registers.get_flag(Flag::Zero), false);
        assert_eq!(registers.get_flag(Flag::Negative), false);
    }

    #[test]
    fn eor_instruction_negative() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: EOR,
            opcode: 0x49,
            operand: Some(Operand::Value(0b1110_0000)),
            addressing_mode: AddressingMode::Immediate,
            operation: InstructionSet::eor,
        };

        registers.accumulator = 0b0100_0000;

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(2));
        assert_eq!(registers.accumulator, 0b1010_0000);
        assert_eq!(registers.get_flag(Flag::Zero), false);
        assert_eq!(registers.get_flag(Flag::Negative), true);
    }

    #[test]
    fn eor_instruction_zero() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: EOR,
            opcode: 0x49,
            operand: Some(Operand::Value(0b0000_0000)),
            addressing_mode: AddressingMode::Immediate,
            operation: InstructionSet::eor,
        };

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(2));
        assert_eq!(registers.accumulator, 0b0000_0000);
        assert_eq!(registers.get_flag(Flag::Zero), true);
        assert_eq!(registers.get_flag(Flag::Negative), false);
    }
}
