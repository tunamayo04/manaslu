use crate::bus::SerialInterface;
use crate::cpu::instruction_set::{AddressingMode, Instruction, InstructionSet};
use crate::cpu::registers::Registers;

impl<T: SerialInterface> InstructionSet<T> {
    pub(crate) fn txs(
        instruction: &Instruction<T>,
        registers: &mut Registers,
        _bus: &mut T,
    ) -> Result<u8, String> {
        registers.stack_pointer = registers.x;

        match instruction.addressing_mode {
            AddressingMode::Implicit => Ok(2),
            _ => Err(String::from("Invalid addressing mode")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::instruction_set::InstructionType::{
        AND, BIT, CLC, CLD, CLV, ORA, PHA, PHP, TSX, TXS,
    };
    use crate::utils::testing::TestBus;

    #[test]
    fn txs_instruction_no_flag() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: TXS,
            opcode: 0x9A,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: InstructionSet::txs,
        };

        registers.x = 0xB6;

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(2));
        assert_eq!(registers.stack_pointer, 0xB6);
    }
}
