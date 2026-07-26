mod adc;
mod and;
mod asl;
mod bcc;

use crate::bus::MemoryIndexer;
use crate::cpu::instruction_set::InstructionType::*;
use crate::cpu::registers::{Flag, Registers};

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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Operand {
    Value(u8),
    Address(u16),
}

pub struct Instruction<T> {
    pub instruction_type: InstructionType,
    pub opcode: u8,
    pub operand: Option<Operand>,
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

        instructions[0x06] = Some(Instruction{
            instruction_type: ASL,
            opcode: 0x06,
            operand: None,
            addressing_mode: AddressingMode::ZeroPage,
            operation: Self::asl,
        });
        instructions[0x0A] = Some(Instruction{
            instruction_type: ASL,
            opcode: 0x0A,
            operand: None,
            addressing_mode: AddressingMode::Accumulator,
            operation: Self::asl,
        });
        instructions[0x0E] = Some(Instruction{
            instruction_type: ASL,
            opcode: 0x0E,
            operand: None,
            addressing_mode: AddressingMode::Absolute,
            operation: Self::asl,
        });

        instructions[0x16] = Some(Instruction{
            instruction_type: ASL,
            opcode: 0x16,
            operand: None,
            addressing_mode: AddressingMode::ZeroPageIndexedX,
            operation: Self::asl,
        });
        instructions[0x1E] = Some(Instruction{
            instruction_type: ASL,
            opcode: 0x1E,
            operand: None,
            addressing_mode: AddressingMode::AbsoluteIndexedX,
            operation: Self::asl,
        });

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

        instructions[0x90] = Some(Instruction{
            instruction_type: BCC,
            opcode: 0x90,
            operand: None,
            addressing_mode: AddressingMode::Relative,
            operation: Self::bcc,
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

    fn lda(instruction: &Instruction<T>, registers: &mut Registers, bus: &mut T) -> Result<u8, String>{
        todo!()
    }
}