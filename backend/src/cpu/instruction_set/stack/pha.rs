use crate::bus::SerialInterface;
use crate::cpu::instruction_set::{AddressingMode, Instruction, InstructionSet};
use crate::cpu::registers::Registers;

impl<T: SerialInterface> InstructionSet<T> {
    pub(crate) fn pha(
        instruction: &Instruction<T>,
        registers: &mut Registers,
        bus: &mut T,
    ) -> Result<u8, String> {
        bus.write_byte(
            0x0100 + registers.stack_pointer as u16,
            registers.accumulator,
        );
        registers.decrement_stack_pointer(1);

        match instruction.addressing_mode {
            AddressingMode::Implicit => Ok(3),
            _ => Err(String::from("Invalid addressing mode")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::instruction_set::InstructionType::{AND, BIT, CLC, CLD, CLV, ORA, PHA};
    use crate::utils::testing::TestBus;

    #[test]
    fn pha_instruction() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: PHA,
            opcode: 0x48,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: InstructionSet::pha,
        };

        registers.stack_pointer = 0xFF;
        registers.accumulator = 0x55;

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(3));
        assert_eq!(registers.stack_pointer, 0xFE);
        assert_eq!(test_bus.read_byte(0x0100 + 0xFFu16), 0x55);
    }
}
