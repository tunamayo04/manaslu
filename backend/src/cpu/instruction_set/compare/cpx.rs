use crate::bus::SerialInterface;
use crate::cpu::instruction_set::{AddressingMode, Instruction, InstructionSet, Operand};
use crate::cpu::registers::{Flag, Registers};
use crate::utils::math::is_negative;

impl<T: SerialInterface> InstructionSet<T> {
    pub(crate) fn cpx(
        instruction: &Instruction<T>,
        registers: &mut Registers,
        bus: &mut T,
    ) -> Result<u8, String> {
        let operand = instruction
            .operand
            .ok_or(String::from("No operand found"))?;
        let operand_value = match operand {
            Operand::Value(value) => value,
            Operand::Address(address) => bus.read_byte(address),
        };

        let result = registers.x.wrapping_sub(operand_value);

        registers.set_flag(Flag::Carry, registers.x >= operand_value);
        registers.set_flag(Flag::Zero, result == 0);
        registers.set_flag(Flag::Negative, is_negative(result));

        match instruction.addressing_mode {
            AddressingMode::Immediate => Ok(2),
            AddressingMode::ZeroPage => Ok(3),
            AddressingMode::ZeroPageIndexedX => Ok(4),
            AddressingMode::Absolute => Ok(4),
            AddressingMode::AbsoluteIndexedX => Ok(4),
            AddressingMode::AbsoluteIndexedY => Ok(4),
            AddressingMode::IndirectIndexedX => Ok(6),
            AddressingMode::IndirectIndexedY => Ok(5),
            AddressingMode::Implicit => Ok(2),
            _ => Err(String::from("Invalid addressing mode")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::instruction_set::InstructionType::{AND, BIT, CLC, CPX, ORA};
    use crate::utils::testing::TestBus;

    #[test]
    fn cpx_instruction_carry() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: CPX,
            opcode: 0xE0,
            operand: Some(Operand::Value(1)),
            addressing_mode: AddressingMode::Immediate,
            operation: InstructionSet::cpx,
        };

        registers.x = 10;

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(2));
        assert_eq!(registers.get_flag(Flag::Carry), true);
        assert_eq!(registers.get_flag(Flag::Zero), false);
        assert_eq!(registers.get_flag(Flag::Negative), false);
    }

    #[test]
    fn cpx_instruction_zero() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: CPX,
            opcode: 0xE0,
            operand: Some(Operand::Value(1)),
            addressing_mode: AddressingMode::Immediate,
            operation: InstructionSet::cpx,
        };

        registers.x = 1;

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(2));
        assert_eq!(registers.get_flag(Flag::Carry), true);
        assert_eq!(registers.get_flag(Flag::Zero), true);
        assert_eq!(registers.get_flag(Flag::Negative), false);
    }

    #[test]
    fn cpx_instruction_negative() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: CPX,
            opcode: 0xE0,
            operand: Some(Operand::Address(0xBEEF)),
            addressing_mode: AddressingMode::Absolute,
            operation: InstructionSet::cpx,
        };

        test_bus.write_byte(0xBEEF, 10);
        registers.x = 1;

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(4));
        assert_eq!(registers.get_flag(Flag::Carry), false);
        assert_eq!(registers.get_flag(Flag::Zero), false);
        assert_eq!(registers.get_flag(Flag::Negative), true);
    }
}
