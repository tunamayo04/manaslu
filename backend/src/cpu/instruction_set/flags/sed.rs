use crate::bus::MemoryIndexer;
use crate::cpu::instruction_set::{AddressingMode, Instruction, InstructionSet};
use crate::cpu::registers::{Flag, Registers};

impl<T: MemoryIndexer> InstructionSet<T> {
    pub(crate) fn sed(
        instruction: &Instruction<T>,
        registers: &mut Registers,
        _bus: &mut T,
    ) -> Result<u8, String> {
        registers.set_flag(Flag::DecimalMode, true);

        match instruction.addressing_mode {
            AddressingMode::Implicit => Ok(2),
            _ => Err(String::from("Invalid addressing mode")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::instruction_set::InstructionType::{AND, BIT, CLC, CLD, ORA, SED};
    use crate::utils::testing::TestBus;

    #[test]
    fn sed_instruction() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: SED,
            opcode: 0xF8,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: InstructionSet::sed,
        };

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(2));
        assert_eq!(registers.get_flag(Flag::DecimalMode), true);
    }
}
