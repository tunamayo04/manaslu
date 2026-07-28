use crate::bus::MemoryIndexer;
use crate::cpu::instruction_set::{AddressingMode, Instruction, InstructionSet, Operand};
use crate::cpu::registers::{Flag, Registers};
use crate::utils::math::is_negative;

impl<T: MemoryIndexer> InstructionSet<T> {
    pub(crate) fn jmp(
        instruction: &Instruction<T>,
        registers: &mut Registers,
        bus: &mut T,
    ) -> Result<u8, String> {
        let operand = instruction.operand.ok_or(String::from("No operand found"))?;
        let operand_value = match operand {
            Operand::Address(address) => address,
            _ => Err("Invalid operand")?,
        };

        registers.program_counter = operand_value;

        match instruction.addressing_mode {
            AddressingMode::Absolute => Ok(3),
            AddressingMode::Indirect => Ok(5),
            _ => Err(String::from("Invalid addressing mode")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::instruction_set::InstructionType::{AND, BIT, CLC, CMP, JMP, JSR, ORA};
    use crate::utils::testing::TestBus;

    #[test]
    fn jsr_instruction() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: JSR,
            opcode: 0x20,
            operand: Some(Operand::Address(0xBEEF)),
            addressing_mode: AddressingMode::Absolute,
            operation: InstructionSet::jsr,
        };

        registers.program_counter = 0xDEAD;

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert 
        assert_eq!(result, Ok(6));
        assert_eq!(registers.program_counter, 0xBEEF);
    }
}
