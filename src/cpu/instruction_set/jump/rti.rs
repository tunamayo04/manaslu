use crate::bus::MemoryIndexer;
use crate::cpu::instruction_set::{AddressingMode, Instruction, InstructionSet, Operand};
use crate::cpu::registers::{Flag, Registers};
use crate::utils::math::is_negative;

impl<T: MemoryIndexer> InstructionSet<T> {
    pub(crate) fn rti(
        instruction: &Instruction<T>,
        registers: &mut Registers,
        bus: &mut T,
    ) -> Result<u8, String> {
        registers.increment_stack_pointer(1);
        let flags = bus.read_byte(registers.stack_pointer as u16 + 0x0100);
        registers.increment_stack_pointer(1);
        let program_counter_low = bus.read_byte(registers.stack_pointer as u16 + 0x0100);
        registers.increment_stack_pointer(1);
        let program_counter_high = bus.read_byte(registers.stack_pointer as u16 + 0x0100);

        registers.program_counter = u16::from_le_bytes([program_counter_low, program_counter_high]);
        registers.flags = flags;

        match instruction.addressing_mode {
            AddressingMode::Implicit => Ok(6),
            _ => Err(String::from("Invalid addressing mode")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::instruction_set::InstructionType::{AND, BIT, CLC, CMP, JMP, JSR, ORA, RTI, RTS};
    use crate::cpu::IRQ_VECTOR;
    use crate::utils::testing::TestBus;

    #[test]
    fn rti_instruction() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: RTI,
            opcode: 0x40,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: InstructionSet::rti,
        };

        registers.program_counter = IRQ_VECTOR;
        registers.stack_pointer = 0xFC;

        test_bus.write_byte(registers.stack_pointer as u16 + 0x101, 0xFF);
        test_bus.write_byte(registers.stack_pointer as u16 + 0x102, 0xEF);
        test_bus.write_byte(registers.stack_pointer as u16 + 0x103, 0xBE);


        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert 
        assert_eq!(result, Ok(6));
        assert_eq!(registers.program_counter, 0xBEEF);
        assert_eq!(registers.stack_pointer, 0xFF);
        assert_eq!(registers.flags, 0xFF);

    }
}
