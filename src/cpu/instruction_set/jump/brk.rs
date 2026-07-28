use crate::bus::MemoryIndexer;
use crate::cpu::instruction_set::{AddressingMode, Instruction, InstructionSet, Operand};
use crate::cpu::IRQ_VECTOR;
use crate::cpu::registers::{Flag, Registers};
use crate::utils::math::is_negative;

impl<T: MemoryIndexer> InstructionSet<T> {
    pub(crate) fn brk(
        instruction: &Instruction<T>,
        registers: &mut Registers,
        bus: &mut T,
    ) -> Result<u8, String> {
        let [program_counter_low, program_counter_high] = registers.program_counter.to_le_bytes();
        bus.write_byte(registers.stack_pointer as u16 + 0x0100, program_counter_high);
        registers.decrement_stack_pointer(1);
        bus.write_byte(registers.stack_pointer as u16 + 0x0100, program_counter_low);
        registers.decrement_stack_pointer(1);

        bus.write_byte(registers.stack_pointer as u16 + 0x0100, registers.flags | 0b0011_0000);
        registers.decrement_stack_pointer(1);

        registers.program_counter = IRQ_VECTOR;

        match instruction.addressing_mode {
            AddressingMode::Implicit => Ok(7),
            AddressingMode::Immediate => Ok(7),
            _ => Err(String::from("Invalid addressing mode")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::instruction_set::InstructionType::{AND, BIT, BRK, CLC, CMP, JMP, JSR, ORA};
    use crate::utils::testing::TestBus;

    #[test]
    fn brk_instruction() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: BRK,
            opcode: 0x00,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: InstructionSet::brk,
        };

        registers.program_counter = 0xDEAD;
        registers.stack_pointer = 0xFF;

        registers.flags = 0xFF;

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert 
        assert_eq!(result, Ok(7));
        assert_eq!(registers.program_counter, IRQ_VECTOR);
        assert_eq!(registers.stack_pointer, 0xFC);
        assert_eq!(test_bus.read_byte(0xFF + 0x0100), 0xDE);
        assert_eq!(test_bus.read_byte(0xFE + 0x0100), 0xAD);
        assert_eq!(test_bus.read_byte(0xFD + 0x0100), 0xFF);
    }
}
