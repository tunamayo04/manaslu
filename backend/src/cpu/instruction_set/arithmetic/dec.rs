use crate::bus::MemoryIndexer;
use crate::cpu::instruction_set::{AddressingMode, Instruction, InstructionSet, Operand};
use crate::cpu::registers::{Flag, Registers};
use crate::utils::math::is_negative;

impl<T: MemoryIndexer> InstructionSet<T> {
    pub(crate) fn dec(
        instruction: &Instruction<T>,
        registers: &mut Registers,
        bus: &mut T,
    ) -> Result<u8, String> {
        let operand = instruction.operand.ok_or("test")?;
        let (address, operand_value) = match operand {
            Operand::Address(address) => (address, bus.read_byte(address)),
            _ => return Err(String::from("Invalid operand")),
        };

        let result = operand_value.wrapping_sub(1);

        registers.set_flag(Flag::Zero, result == 0);
        registers.set_flag(Flag::Negative, is_negative(result));

        bus.write_byte(address, result);

        match instruction.addressing_mode {
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
    use crate::cpu::instruction_set::InstructionType::{DEC, INC};
    use crate::utils::testing::TestBus;

    #[test]
    fn dec_instruction_positive() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: DEC,
            opcode: 0xCE,
            operand: Some(Operand::Address(0xBEEF)),
            addressing_mode: AddressingMode::Absolute,
            operation: InstructionSet::dec,
        };

        test_bus.write_byte(0xBEEF, 0x02);

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(6));
        assert_eq!(test_bus.read_byte(0xBEEF), 0x01);
        assert_eq!(registers.get_flag(Flag::Zero), false);
        assert_eq!(registers.get_flag(Flag::Negative), false);
    }

    #[test]
    fn dec_instruction_negative() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: DEC,
            opcode: 0xCE,
            operand: Some(Operand::Address(0xBEEF)),
            addressing_mode: AddressingMode::Absolute,
            operation: InstructionSet::dec,
        };

        test_bus.write_byte(0xBEEF, 0x00);

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(6));
        assert_eq!(test_bus.read_byte(0xBEEF), 0xFF);
        assert_eq!(registers.get_flag(Flag::Zero), false);
        assert_eq!(registers.get_flag(Flag::Negative), true);
    }

    #[test]
    fn dec_instruction_zero() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: DEC,
            opcode: 0xCE,
            operand: Some(Operand::Address(0xBEEF)),
            addressing_mode: AddressingMode::Absolute,
            operation: InstructionSet::dec,
        };

        test_bus.write_byte(0xBEEF, 0x01);

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(6));
        assert_eq!(test_bus.read_byte(0xBEEF), 0);
        assert_eq!(registers.get_flag(Flag::Zero), true);
        assert_eq!(registers.get_flag(Flag::Negative), false);
    }
}
