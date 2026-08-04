use crate::cpu::instruction_set::{Instruction, InstructionSet, Operand};
use crate::cpu::registers::{Flag, Registers};

impl<T> InstructionSet<T> {
    pub(crate) fn beq(
        instruction: &Instruction<T>,
        registers: &mut Registers,
        _bus: &mut T,
    ) -> Result<u8, String> {
        let operand = instruction.operand.ok_or("test")?;
        let branch_address = match operand {
            Operand::Address(address) => address,
            Operand::Value(_) => return Err(String::from("Invalid operand")),
        };

        let zero_flag = registers.get_flag(Flag::Zero);
        if zero_flag {
            registers.program_counter = branch_address;
        }

        Ok(2 + if zero_flag { 1 } else { 0 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::instruction_set::AddressingMode;
    use crate::cpu::instruction_set::InstructionType::{BCC, BCS, BEQ};
    use crate::utils::testing::TestBus;

    #[test]
    fn bcc_carry_set() {
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: BEQ,
            opcode: 0xF0,
            operand: Some(Operand::Address(0xBEEF)),
            addressing_mode: AddressingMode::Relative,
            operation: InstructionSet::beq,
        };

        registers.program_counter = 0xDEAD;
        registers.set_flag(Flag::Zero, true);

        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        assert_eq!(result, Ok(3));
        assert_eq!(registers.program_counter, 0xBEEF);
    }

    #[test]
    fn bcc_carry_not_set() {
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction {
            instruction_type: BEQ,
            opcode: 0xF0,
            operand: Some(Operand::Address(0xBEEF)),
            addressing_mode: AddressingMode::Relative,
            operation: InstructionSet::beq,
        };

        registers.program_counter = 0xDEAD;
        registers.set_flag(Flag::Zero, false);

        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        assert_eq!(result, Ok(2));
        assert_eq!(registers.program_counter, 0xDEAD);
    }
}
