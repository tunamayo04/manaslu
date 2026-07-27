use crate::bus::MemoryIndexer;
use crate::cpu::instruction_set::{AddressingMode, Instruction, InstructionSet, Operand};
use crate::cpu::registers::{Registers};

impl<T: MemoryIndexer> InstructionSet<T> {
    pub fn stx(
        instruction: &Instruction<T>,
        registers: &mut Registers,
        bus: &mut T,
    ) -> Result<u8, String> {
        let operand = instruction.operand.ok_or("Missing operand")?;
        let address = match operand {
            Operand::Address(address) => address,
            _ => return Err(String::from("Invalid operand type")),
        };

        bus.write_byte(address, registers.x);

        match instruction.addressing_mode {
            AddressingMode::ZeroPage => Ok(3),
            AddressingMode::ZeroPageIndexedX => Ok(4),
            AddressingMode::Absolute => Ok(4),
            _ => Err(String::from("Invalid addressing mode")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::instruction_set::InstructionType::{STX};
    use crate::utils::testing::TestBus;

    #[test]
    fn stx_stores_value() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: STX,
            opcode: 0x8E,
            operand: Some(Operand::Address(0xBEEF)),
            addressing_mode: AddressingMode::Absolute,
            operation: InstructionSet::stx,
        };

        registers.x = 0x12;

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(4));
        assert_eq!(test_bus.read_byte(0xBEEF), 0x12);
    }
}
