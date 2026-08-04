use crate::bus::SerialInterface;
use crate::cpu::instruction_set::{AddressingMode, Instruction, InstructionSet, Operand};
use crate::cpu::registers::{Flag, Registers};
use crate::utils::math::is_negative;

impl<T: SerialInterface> InstructionSet<T> {
    pub(crate) fn bit(
        instruction: &Instruction<T>,
        registers: &mut Registers,
        bus: &mut T,
    ) -> Result<u8, String> {
        let operand = match instruction.operand.ok_or("Missing operand")? {
            Operand::Address(address) => bus.read_byte(address),
            _ => return Err(String::from("Invalid operand")),
        };

        let result = registers.accumulator & operand;

        registers.set_flag(Flag::Zero, result == 0);
        registers.set_flag(Flag::Negative, is_negative(operand));
        registers.set_flag(Flag::Overflow, operand & 0b0100_0000 != 0);

        match instruction.addressing_mode {
            AddressingMode::ZeroPage => Ok(3),
            AddressingMode::Absolute => Ok(4),
            _ => Err(String::from("Invalid addressing mode")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::instruction_set::InstructionType::{AND, BIT, ORA};
    use crate::utils::testing::TestBus;

    #[test]
    fn bit_instruction_zero() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: BIT,
            opcode: 0x2C,
            operand: Some(Operand::Address(0xBEEF)),
            addressing_mode: AddressingMode::Absolute,
            operation: InstructionSet::bit,
        };

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(4));
        assert_eq!(registers.get_flag(Flag::Zero), true);
        assert_eq!(registers.get_flag(Flag::Negative), false);
        assert_eq!(registers.get_flag(Flag::Overflow), false);
    }

    #[test]
    fn bit_instruction_negative() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: BIT,
            opcode: 0x2C,
            operand: Some(Operand::Address(0xBEEF)),
            addressing_mode: AddressingMode::Absolute,
            operation: InstructionSet::bit,
        };

        registers.accumulator = 0b10000000;
        test_bus.write_byte(0xBEEF, 0b10000000);

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(4));
        assert_eq!(registers.get_flag(Flag::Zero), false);
        assert_eq!(registers.get_flag(Flag::Negative), true);
        assert_eq!(registers.get_flag(Flag::Overflow), false);
    }

    #[test]
    fn ora_instruction_overflow() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: BIT,
            opcode: 0x2C,
            operand: Some(Operand::Address(0xBEEF)),
            addressing_mode: AddressingMode::Absolute,
            operation: InstructionSet::bit,
        };

        registers.accumulator = 0b01000000;
        test_bus.write_byte(0xBEEF, 0b01000000);

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(4));
        assert_eq!(registers.get_flag(Flag::Zero), false);
        assert_eq!(registers.get_flag(Flag::Negative), false);
        assert_eq!(registers.get_flag(Flag::Overflow), true);
    }
}
