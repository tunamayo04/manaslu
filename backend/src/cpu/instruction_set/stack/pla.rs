use crate::bus::MemoryIndexer;
use crate::cpu::instruction_set::{AddressingMode, Instruction, InstructionSet};
use crate::cpu::registers::{Flag, Registers};
use crate::utils::math::is_negative;

impl<T: MemoryIndexer> InstructionSet<T> {
    pub(crate) fn pla(
        instruction: &Instruction<T>,
        registers: &mut Registers,
        bus: &mut T,
    ) -> Result<u8, String> {
        registers.increment_stack_pointer(1);
        registers.accumulator = bus.read_byte(0x0100 + registers.stack_pointer as u16);

        registers.set_flag(Flag::Zero, registers.accumulator == 0);
        registers.set_flag(Flag::Negative, is_negative(registers.accumulator));

        match instruction.addressing_mode {
            AddressingMode::Implicit => Ok(4),
            _ => Err(String::from("Invalid addressing mode")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::instruction_set::InstructionType::{AND, BIT, CLC, CLD, CLV, ORA, PHA, PLA};
    use crate::utils::testing::TestBus;

    #[test]
    fn pla_instruction() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: PLA,
            opcode: 0x68,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: InstructionSet::pla,
        };

        test_bus.write_byte(0x0101, 0x55);

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(4));
        assert_eq!(registers.stack_pointer, 0x01);
        assert_eq!(registers.accumulator, 0x55);
        assert_eq!(registers.get_flag(Flag::Zero), false);
        assert_eq!(registers.get_flag(Flag::Negative), false);
    }
}
