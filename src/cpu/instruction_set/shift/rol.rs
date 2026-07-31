use crate::bus::MemoryIndexer;
use crate::cpu::instruction_set::{AddressingMode, Instruction, InstructionSet, Operand};
use crate::cpu::registers::{Flag, Registers};
use crate::utils::math::is_negative;

impl<T: MemoryIndexer> InstructionSet<T> {
    pub(crate) fn rol(
        instruction: &Instruction<T>,
        registers: &mut Registers,
        bus: &mut T,
    ) -> Result<u8, String> {
        let operand = instruction.operand.ok_or("test")?;
        let operand_value = match operand {
            Operand::Address(address) => bus.read_byte(address),
            Operand::Value(value) => value,
        };

        let carry = registers.get_flag(Flag::Carry);
        let result = (operand_value << 1) | (carry as u8);

        registers.set_flag(Flag::Carry, operand_value & 0x80 != 0);
        registers.set_flag(Flag::Zero, result == 0);
        registers.set_flag(Flag::Negative, is_negative(result));

        match operand {
            Operand::Value(_) => registers.accumulator = result,
            Operand::Address(address) => bus.write_byte(address, result),
        }

        match instruction.addressing_mode {
            AddressingMode::Accumulator => Ok(2),
            AddressingMode::ZeroPage => Ok(5),
            AddressingMode::ZeroPageIndexedX => Ok(6),
            AddressingMode::Absolute => Ok(6),
            AddressingMode::AbsoluteIndexedX => Ok(7),
            _ => Err(String::from("Invalid addressing mode")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::instruction_set::InstructionType::{ASL, ROL};
    use crate::utils::testing::TestBus;

    #[test]
    fn asl_instruction_no_carry_positive_to_accumulator() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: ROL,
            opcode: 0x2A,
            operand: Some(Operand::Value(0b00000001)),
            addressing_mode: AddressingMode::Accumulator,
            operation: InstructionSet::rol,
        };

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(2));
        assert_eq!(registers.accumulator, 0b00000010);
        assert_eq!(registers.get_flag(Flag::Carry), false);
        assert_eq!(registers.get_flag(Flag::Zero), false);
        assert_eq!(registers.get_flag(Flag::Negative), false);
    }

    #[test]
    fn asl_instruction_with_carry_zero_to_address() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: ROL,
            opcode: 0x2E,
            operand: Some(Operand::Address(0xBEEF)),
            addressing_mode: AddressingMode::Absolute,
            operation: InstructionSet::rol,
        };

        test_bus.write_byte(0xBEEF, 0b1000_0000);

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(6));
        assert_eq!(test_bus.read_byte(0xBEEF), 0b0000_0001);
        assert_eq!(registers.get_flag(Flag::Carry), true);
        assert_eq!(registers.get_flag(Flag::Zero), false);
        assert_eq!(registers.get_flag(Flag::Negative), false);
    }

    #[test]
    fn asl_instruction_no_carry_negative_to_address() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: ROL,
            opcode: 0x2E,
            operand: Some(Operand::Address(0xBEEF)),
            addressing_mode: AddressingMode::Absolute,
            operation: InstructionSet::rol,
        };

        test_bus.write_byte(0xBEEF, 0b0100_0000);

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(6));
        assert_eq!(test_bus.read_byte(0xBEEF), 0b1000_0000);
        assert_eq!(registers.get_flag(Flag::Carry), false);
        assert_eq!(registers.get_flag(Flag::Zero), false);
        assert_eq!(registers.get_flag(Flag::Negative), true);
    }
}
