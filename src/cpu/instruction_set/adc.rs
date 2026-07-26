use crate::bus::MemoryIndexer;
use crate::cpu::instruction_set::{AddressingMode, Instruction, InstructionSet, Operand};
use crate::cpu::registers::{Flag, Registers};
use crate::utils::math::is_negative;

impl<T: MemoryIndexer> InstructionSet<T> {
    pub(crate) fn adc(instruction: &Instruction<T>, registers: &mut Registers, bus: &mut T) -> Result<u8, String> {
        let operand = match instruction.operand.ok_or("test")? {
            Operand::Address(address) => bus.read_byte(address),
            Operand::Value(value) => value
        };

        let a = registers.accumulator as u16;
        let result = a + operand as u16 + registers.get_flag(Flag::Carry) as u16;
        let carry = result > 0xFF;
        let result = result as u8;

        registers.set_flag(Flag::Carry, carry);
        registers.set_flag(Flag::Zero, result == 0);
        registers.set_flag(Flag::Negative, is_negative(result));
        registers.set_flag(Flag::Overflow, (result ^ a as u8) & (result ^ operand) & 0x80 != 0);
        registers.accumulator = result;

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
    use crate::cpu::instruction_set::InstructionType::ADC;
    use crate::utils::testing::TestBus;
    use super::*;

    #[test]
    fn adc_sets_no_flags() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction{
            instruction_type: ADC,
            opcode: 0x69,
            operand: Some(Operand::Value(0x01)),
            addressing_mode: AddressingMode::Immediate,
            operation: InstructionSet::adc,
        };

        registers.accumulator = 0x01;
        registers.set_flag(Flag::Carry, true);

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(2));
        assert_eq!(registers.accumulator, 0x03);
        assert_eq!(registers.get_flag(Flag::Carry), false);
        assert_eq!(registers.get_flag(Flag::Zero), false);
        assert_eq!(registers.get_flag(Flag::Negative), false);
        assert_eq!(registers.get_flag(Flag::Overflow), false);
    }

    #[test]
    fn adc_sets_carry_flag_zero_result() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction{
            instruction_type: ADC,
            opcode: 0x69,
            operand: Some(Operand::Value(0x01)),
            addressing_mode: AddressingMode::Immediate,
            operation: InstructionSet::adc,
        };

        registers.accumulator = 0xFF;
        registers.set_flag(Flag::Carry, false);

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(2));
        assert_eq!(registers.accumulator, 0x00);
        assert_eq!(registers.get_flag(Flag::Carry), true);
        assert_eq!(registers.get_flag(Flag::Zero), true);
        assert_eq!(registers.get_flag(Flag::Negative), false);
        assert_eq!(registers.get_flag(Flag::Overflow), false);
    }

    #[test]
    fn adc_sets_overflow_flag_negative_result() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction{
            instruction_type: ADC,
            opcode: 0x69,
            operand: Some(Operand::Value(0x01)),
            addressing_mode: AddressingMode::Immediate,
            operation: InstructionSet::adc,
        };

        registers.accumulator = 0x7F;

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(2));
        assert_eq!(registers.accumulator, 0x80);
        assert_eq!(registers.get_flag(Flag::Overflow), true);
        assert_eq!(registers.get_flag(Flag::Carry), false);
        assert_eq!(registers.get_flag(Flag::Negative), true);
        assert_eq!(registers.get_flag(Flag::Zero), false);
    }

    #[test]
    fn adc_sets_overflow_flag_positive_result() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction{
            instruction_type: ADC,
            opcode: 0x69,
            operand: Some(Operand::Value(0xFF)),
            addressing_mode: AddressingMode::Immediate,
            operation: InstructionSet::adc,
        };

        registers.accumulator = 0x80;

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(2));
        assert_eq!(registers.accumulator, 0x7F);
        assert_eq!(registers.get_flag(Flag::Overflow), true);
        assert_eq!(registers.get_flag(Flag::Carry), true);
        assert_eq!(registers.get_flag(Flag::Negative), false);
        assert_eq!(registers.get_flag(Flag::Zero), false);
    }
}