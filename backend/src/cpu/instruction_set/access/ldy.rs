use crate::bus::SerialInterface;
use crate::cpu::instruction_set::{AddressingMode, Instruction, InstructionSet, Operand};
use crate::cpu::registers::{Flag, Registers};
use crate::utils::math::is_negative;

impl<T: SerialInterface> InstructionSet<T> {
    pub fn ldy(
        instruction: &Instruction<T>,
        registers: &mut Registers,
        bus: &mut T,
    ) -> Result<u8, String> {
        let operand = instruction.operand.ok_or("Missing operand")?;
        let operand_value = match operand {
            Operand::Value(value) => value,
            Operand::Address(address) => bus.read_byte(address),
        };

        registers.set_flag(Flag::Zero, operand_value == 0);
        registers.set_flag(Flag::Negative, is_negative(operand_value));

        registers.y = operand_value;

        match instruction.addressing_mode {
            AddressingMode::Immediate => Ok(2),
            AddressingMode::ZeroPage => Ok(3),
            AddressingMode::ZeroPageIndexedX => Ok(4),
            AddressingMode::Absolute => Ok(4),
            AddressingMode::AbsoluteIndexedX => Ok(4),
            _ => Err(String::from("Invalid addressing mode")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::instruction_set::InstructionType::LDY;
    use crate::utils::testing::TestBus;

    #[test]
    fn ldy_loads_non_zero_value_immediate() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: LDY,
            opcode: 0xA9,
            operand: Some(Operand::Value(0x25)),
            addressing_mode: AddressingMode::Immediate,
            operation: InstructionSet::ldy,
        };

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(2));
        assert_eq!(registers.y, 0x25);
        assert_eq!(registers.get_flag(Flag::Zero), false);
        assert_eq!(registers.get_flag(Flag::Negative), false);
    }

    #[test]
    fn ldy_loads_zero_value_immediate() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: LDY,
            opcode: 0xA9,
            operand: Some(Operand::Value(0x00)),
            addressing_mode: AddressingMode::Immediate,
            operation: InstructionSet::ldy,
        };

        registers.y = 0x25;

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(2));
        assert_eq!(registers.y, 0x00);
        assert_eq!(registers.get_flag(Flag::Zero), true);
        assert_eq!(registers.get_flag(Flag::Negative), false);
    }

    #[test]
    fn ldy_loads_negative_value_immediate() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: LDY,
            opcode: 0xA9,
            operand: Some(Operand::Value(0xFF)),
            addressing_mode: AddressingMode::Immediate,
            operation: InstructionSet::ldy,
        };

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(2));
        assert_eq!(registers.y, 0xFF);
        assert_eq!(registers.get_flag(Flag::Zero), false);
        assert_eq!(registers.get_flag(Flag::Negative), true);
    }

    #[test]
    fn ldy_loads_value_from_address() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: LDY,
            opcode: 0xA9,
            operand: Some(Operand::Address(0xBEEF)),
            addressing_mode: AddressingMode::Absolute,
            operation: InstructionSet::ldy,
        };

        test_bus.write_byte(0xBEEF, 0x25);

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(4));
        assert_eq!(registers.y, 0x25);
        assert_eq!(registers.get_flag(Flag::Zero), false);
        assert_eq!(registers.get_flag(Flag::Negative), false);
    }
}
