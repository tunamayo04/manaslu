use crate::bus::MemoryIndexer;
use crate::cpu::instruction_set::{AddressingMode, Instruction, InstructionSet, Operand};
use crate::cpu::registers::Registers;

impl<T: MemoryIndexer> InstructionSet<T> {
    pub(crate) fn jsr(
        instruction: &Instruction<T>,
        registers: &mut Registers,
        bus: &mut T,
    ) -> Result<u8, String> {
        let operand = instruction
            .operand
            .ok_or(String::from("No operand found"))?;
        let operand_value = match operand {
            Operand::Address(address) => address,
            _ => Err("Invalid operand")?,
        };

        let [program_counter_low, program_counter_high] = registers.program_counter.to_le_bytes();
        bus.write_byte(
            registers.stack_pointer as u16 + 0x0100,
            program_counter_high,
        );
        registers.decrement_stack_pointer(1);
        bus.write_byte(registers.stack_pointer as u16 + 0x0100, program_counter_low);
        registers.decrement_stack_pointer(1);

        registers.program_counter = operand_value;

        match instruction.addressing_mode {
            AddressingMode::Absolute => Ok(6),
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
        registers.stack_pointer = 0xFF;

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(6));
        assert_eq!(registers.program_counter, 0xBEEF);
        assert_eq!(registers.stack_pointer, 0xFD);
        assert_eq!(test_bus.read_byte(0xFF + 0x0100), 0xDE);
        assert_eq!(test_bus.read_byte(0xFE + 0x0100), 0xAD);
    }
}
