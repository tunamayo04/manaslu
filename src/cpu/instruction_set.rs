use crate::bus::MemoryIndexer;
use crate::cpu::instruction_set::InstructionType::{ADC, AND, LDA};
use crate::cpu::registers::{Flag, Registers};
use crate::utils::math::is_negative;

type Operation<T: MemoryIndexer> = fn(&Instruction<T>, &mut Registers, &mut T) -> Result<u8, String>;

#[derive(Copy, Clone)]
pub enum AddressingMode {
    ZeroPageIndexedX,
    ZeroPageIndexedY,
    AbsoluteIndexedX,
    AbsoluteIndexedY,
    IndirectIndexedX,
    IndirectIndexedY,
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage,
    Absolute,
    Relative,
    Indirect,
}
#[derive(Copy, Clone)]
pub enum InstructionType {
    // Access
    LDA,
    STA,
    LDX,
    STX,
    LDY,
    STY,
    // Transfer
    TAX,
    TXA,
    TAY,
    TYA,
    // Arithmetic
    ADC,
    SBC,
    INC,
    DEC,
    INX,
    DEX,
    INY,
    DEY,
    // Shift
    ASL,
    LSR,
    ROL,
    ROR,
    // Bitwise
    AND,
    ORA,
    EOR,
    BIT,
    // Compare
    CMP,
    CPX,
    CPY,
    // Branch
    BCC,
    BCS,
    BEQ,
    BNE,
    BPL,
    BMI,
    BVC,
    BVS,
    // Jump
    JMP,
    JSR,
    RTS,
    BRK,
    RTI,
    // Stack
    PHA,
    PLA,
    PHP,
    PLP,
    TXS,
    TSX,
    // Flags
    CLC,
    SEC,
    CLI,
    SEI,
    CLD,
    SED,
    CLV,
    // Other
    NOP
}

pub struct Instruction<T> {
    pub instruction_type: InstructionType,
    pub opcode: u8,
    pub operand: Option<u16>,
    pub addressing_mode: AddressingMode,
    pub operation: Operation<T>,
}
impl<T> Clone for Instruction<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for Instruction<T> {}

pub struct InstructionSet<T> {
    instructions: [Option<Instruction<T>>; 256],
}
impl<T: MemoryIndexer> InstructionSet<T> {
    pub fn new() -> InstructionSet<T> {
        let mut instructions = std::array::from_fn(|_| None);

        instructions[0x21] = Some(Instruction{
            instruction_type: AND,
            opcode: 0x21,
            operand: None,
            addressing_mode: AddressingMode::IndirectIndexedX,
            operation: Self::and,
        });
        instructions[0x25] = Some(Instruction{
            instruction_type: AND,
            opcode: 0x25,
            operand: None,
            addressing_mode: AddressingMode::ZeroPage,
            operation: Self::and,
        });
        instructions[0x29] = Some(Instruction{
            instruction_type: AND,
            opcode: 0x29,
            operand: None,
            addressing_mode: AddressingMode::Immediate,
            operation: Self::and,
        });
        instructions[0x2D] = Some(Instruction{
            instruction_type: AND,
            opcode: 0x2D,
            operand: None,
            addressing_mode: AddressingMode::Absolute,
            operation: Self::and,
        });

        instructions[0x31] = Some(Instruction{
            instruction_type: AND,
            opcode: 0x31,
            operand: None,
            addressing_mode: AddressingMode::IndirectIndexedY,
            operation: Self::and,
        });
        instructions[0x35] = Some(Instruction{
            instruction_type: AND,
            opcode: 0x35,
            operand: None,
            addressing_mode: AddressingMode::ZeroPageIndexedX,
            operation: Self::and,
        });
        instructions[0x39] = Some(Instruction{
            instruction_type: AND,
            opcode: 0x39,
            operand: None,
            addressing_mode: AddressingMode::AbsoluteIndexedY,
            operation: Self::and,
        });
        instructions[0x3D] = Some(Instruction{
            instruction_type: AND,
            opcode: 0x3D,
            operand: None,
            addressing_mode: AddressingMode::AbsoluteIndexedX,
            operation: Self::and,
        });

        instructions[0x61] = Some(Instruction{
            instruction_type: ADC,
            opcode: 0x61,
            operand: None,
            addressing_mode: AddressingMode::IndirectIndexedX,
            operation: Self::adc,
        });
        instructions[0x65] = Some(Instruction{
            instruction_type: ADC,
            opcode: 0x65,
            operand: None,
            addressing_mode: AddressingMode::ZeroPage,
            operation: Self::adc,
        });
        instructions[0x69] = Some(Instruction{
            instruction_type: ADC,
            opcode: 0x69,
            operand: None,
            addressing_mode: AddressingMode::Immediate,
            operation: Self::adc,
        });
        instructions[0x6D] = Some(Instruction{
            instruction_type: ADC,
            opcode: 0x6D,
            operand: None,
            addressing_mode: AddressingMode::Absolute,
            operation: Self::adc,
        });

        instructions[0x71] = Some(Instruction{
            instruction_type: ADC,
            opcode: 0x71,
            operand: None,
            addressing_mode: AddressingMode::IndirectIndexedY,
            operation: Self::adc,
        });
        instructions[0x75] = Some(Instruction{
            instruction_type: ADC,
            opcode: 0x75,
            operand: None,
            addressing_mode: AddressingMode::ZeroPageIndexedX,
            operation: Self::adc,
        });
        instructions[0x79] = Some(Instruction{
            instruction_type: ADC,
            opcode: 0x79,
            operand: None,
            addressing_mode: AddressingMode::AbsoluteIndexedY,
            operation: Self::adc,
        });
        instructions[0x7D] = Some(Instruction{
            instruction_type: ADC,
            opcode: 0x7D,
            operand: None,
            addressing_mode: AddressingMode::AbsoluteIndexedX,
            operation: Self::adc,
        });

        instructions[0xA9] = Some(Instruction{
            instruction_type: LDA,
            opcode: 0xA9,
            operand: None,
            addressing_mode: AddressingMode::Immediate,
            operation: Self::lda,
        });

        InstructionSet {
            instructions
        }
    }

    pub fn get_instruction(&self, opcode: u8) -> Option<Instruction<T>> {
        match &self.instructions[opcode as usize] {
            Some(instruction) => Some(*instruction),
            None => None,
        }
    }

    fn adc(instruction: &Instruction<T>, registers: &mut Registers, _: &mut T) -> Result<u8, String> {
        let operand = instruction.operand.ok_or("Invalid operand")? as u8;

        let a = registers.accumulator as u16;
        let result = a + operand as u16 + registers.get_flag(Flag::Carry) as u16;
        let carry = result > 0xFF;
        let result = result as u8;

        registers.set_flag(Flag::Carry, carry);
        registers.set_flag(Flag::Zero, result == 0);
        registers.set_flag(Flag::Negative, is_negative(result));
        registers.set_flag(Flag::Overflow, (result ^ a as u8) & (result ^ operand) & 0x80 != 0);
        registers.accumulator = result;

        match instruction.addressing_mode {
            AddressingMode::Immediate => Ok(2),
            AddressingMode::ZeroPage => Ok(3),
            AddressingMode::ZeroPageIndexedX => Ok(4),
            AddressingMode::Absolute => Ok(4),
            AddressingMode::AbsoluteIndexedX => Ok(4),
            AddressingMode::AbsoluteIndexedY => Ok(4),
            AddressingMode::IndirectIndexedX => Ok(6),
            AddressingMode::IndirectIndexedY => Ok(5),
            _ => Err(String::from("Invalid addressing mode")),
        }
    }

    fn and(instruction: &Instruction<T>, registers: &mut Registers, _: &mut T) -> Result<u8, String> {
        let operand = instruction.operand.ok_or("Invalid operand")? as u8;
        registers.accumulator = registers.accumulator & operand;

        registers.set_flag(Flag::Zero, false);
        registers.set_flag(Flag::Negative, is_negative(registers.accumulator));

        match instruction.addressing_mode {
            AddressingMode::Immediate => Ok(2),
            AddressingMode::ZeroPage => Ok(3),
            AddressingMode::ZeroPageIndexedX => Ok(4),
            AddressingMode::Absolute => Ok(4),
            AddressingMode::AbsoluteIndexedX => Ok(4),
            AddressingMode::AbsoluteIndexedY => Ok(4),
            AddressingMode::IndirectIndexedX => Ok(6),
            AddressingMode::IndirectIndexedY => Ok(5),
            _ => Err(String::from("Invalid addressing mode")),
        }
    }

    fn asl(instruction: &Instruction<T>, registers: &mut Registers, _: &mut T) -> Result<u8, String> {
        let operand = instruction.operand.ok_or("Invalid operand")? as u8;
        registers.accumulator = registers.accumulator << operand;

        match instruction.addressing_mode {
            AddressingMode::Accumulator => Ok(2),
            AddressingMode::ZeroPage => Ok(5),
            AddressingMode::ZeroPageIndexedX => Ok(6),
            AddressingMode::Absolute => Ok(6),
            AddressingMode::AbsoluteIndexedX => Ok(7),
            _ => Err(String::from("Invalid addressing mode")),
        }
    }

    fn lda(instruction: &Instruction<T>, registers: &mut Registers, bus: &mut T) -> Result<u8, String>{
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::instruction_set::InstructionType::ADC;
    use crate::utils::testing::TestBus;
    use super::*;

    // region ADC
    #[test]
    fn adc_sets_no_flags() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction{
            instruction_type: ADC,
            opcode: 0x69,
            operand: Some(0x01),
            addressing_mode: AddressingMode::Immediate,
            operation: InstructionSet::adc,
        };

        registers.accumulator = 0x01;
        registers.set_flag(Flag::Carry, true);

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(2));
        assert_eq!(registers.accumulator, 0x03);
        assert_eq!(registers.get_flag(Flag::Carry), false);
        assert_eq!(registers.get_flag(Flag::Zero), false);
        assert_eq!(registers.get_flag(Flag::Negative), false);
        assert_eq!(registers.get_flag(Flag::Overflow), false);
    }

    #[test]
    fn adc_sets_carry_flag_zero_result() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction{
            instruction_type: ADC,
            opcode: 0x69,
            operand: Some(0x01),
            addressing_mode: AddressingMode::Immediate,
            operation: InstructionSet::adc,
        };

        registers.accumulator = 0xFF;
        registers.set_flag(Flag::Carry, false);

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(2));
        assert_eq!(registers.accumulator, 0x00);
        assert_eq!(registers.get_flag(Flag::Carry), true);
        assert_eq!(registers.get_flag(Flag::Zero), true);
        assert_eq!(registers.get_flag(Flag::Negative), false);
        assert_eq!(registers.get_flag(Flag::Overflow), false);
    }

    #[test]
    fn adc_sets_overflow_flag_negative_result() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction{
            instruction_type: ADC,
            opcode: 0x69,
            operand: Some(0x01),
            addressing_mode: AddressingMode::Immediate,
            operation: InstructionSet::adc,
        };

        registers.accumulator = 0x7F;

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(2));
        assert_eq!(registers.accumulator, 0x80);
        assert_eq!(registers.get_flag(Flag::Overflow), true);
        assert_eq!(registers.get_flag(Flag::Carry), false);
        assert_eq!(registers.get_flag(Flag::Negative), true);
        assert_eq!(registers.get_flag(Flag::Zero), false);
    }

    #[test]
    fn adc_sets_overflow_flag_positive_result() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction{
            instruction_type: ADC,
            opcode: 0x69,
            operand: Some(0xFF),
            addressing_mode: AddressingMode::Immediate,
            operation: InstructionSet::adc,
        };

        registers.accumulator = 0x80;

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(2));
        assert_eq!(registers.accumulator, 0x7F);
        assert_eq!(registers.get_flag(Flag::Overflow), true);
        assert_eq!(registers.get_flag(Flag::Carry), true);
        assert_eq!(registers.get_flag(Flag::Negative), false);
        assert_eq!(registers.get_flag(Flag::Zero), false);
    }
    // endregion ADC

    // region AND
    #[test]
    fn and_instruction_positive() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction{
            instruction_type: AND,
            opcode: 0x29,
            operand: Some(0b10101011),
            addressing_mode: AddressingMode::Immediate,
            operation: InstructionSet::and,
        };

        registers.accumulator = 0b00001001;

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(2));
        assert_eq!(registers.accumulator, 0b00001001);
        assert_eq!(registers.get_flag(Flag::Zero), false);
        assert_eq!(registers.get_flag(Flag::Negative), false);
    }

    #[test]
    fn and_instruction_negative() {
        // Arrange
        let mut registers = Registers::new();
        let mut test_bus = TestBus::new();

        let instruction = Instruction{
            instruction_type: AND,
            opcode: 0x29,
            operand: Some(0b10101011),
            addressing_mode: AddressingMode::Immediate,
            operation: InstructionSet::and,
        };

        registers.accumulator = 0b10001001;

        // Act
        let result = (instruction.operation)(&instruction, &mut registers, &mut test_bus);

        // Assert
        assert_eq!(result, Ok(2));
        assert_eq!(registers.accumulator, 0b10001001);
        assert_eq!(registers.get_flag(Flag::Zero), false);
        assert_eq!(registers.get_flag(Flag::Negative), true);
    }
    // endregion AND
}