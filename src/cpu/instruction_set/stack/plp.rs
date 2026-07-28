use crate::bus::MemoryIndexer;
use crate::cpu::instruction_set::{AddressingMode, Instruction, InstructionSet, Operand};
use crate::cpu::registers::{Flag, Registers};

impl<T: MemoryIndexer> InstructionSet<T> {
    pub(crate) fn plp(
        instruction: &Instruction<T>,
        registers: &mut Registers,
        bus: &mut T,
    ) -> Result<u8, String> {
        registers.increment_stack_pointer(1);
        let value = bus.read_byte(0x0100 + registers.stack_pointer as u16);

        registers.flags = value & 0b1100_1111;

        match instruction.addressing_mode {
            AddressingMode::Implicit => Ok(4),
            _ => Err(String::from("Invalid addressing mode")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::instruction_set::InstructionType::{AND, BIT, CLC, CLD, CLV, ORA, PHA, PHP, PLP};
    use crate::utils::testing::TestBus;

    #[test]
    fn plp_instruction_no_flag() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: PLP,
            opcode: 0x08,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: InstructionSet::plp,
        };

        registers.flags = 0xFF;

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(4));
        assert_eq!(registers.stack_pointer, 0x01);
        assert_eq!(registers.flags, 0x00);
    }

    #[test]
    fn php_instruction_all_flags() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: PLP,
            opcode: 0x28,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: InstructionSet::plp,
        };

        test_bus.write_byte(0x0100 + 0x01, 0xFF);

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(4));
        assert_eq!(registers.stack_pointer, 0x01);
        assert_eq!(registers.flags, 0b1100_1111);
    }
}
