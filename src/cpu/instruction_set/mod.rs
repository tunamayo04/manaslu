pub mod arithmetic;
pub mod bitwise;
pub mod branch;
pub mod shift;
pub mod nop;

use crate::bus::MemoryIndexer;
use crate::cpu::instruction_set::InstructionType::*;
use crate::cpu::registers::Registers;

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

        instructions[0x88] = Some(Instruction{
            instruction_type: DEY,
            opcode: 0x88,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: Self::dey,
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

        instructions[0xCA] = Some(Instruction{
            instruction_type: DEX,
            opcode: 0xCA,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: Self::dex,
        });
        instructions[0xC6] = Some(Instruction{
            instruction_type: DEC,
            opcode: 0xAD,
            operand: None,
            addressing_mode: AddressingMode::ZeroPage,
            operation: Self::dec,
        });
        instructions[0xCE] = Some(Instruction{
            instruction_type: DEC,
            opcode: 0xCE,
            operand: None,
            addressing_mode: AddressingMode::Absolute,
            operation: Self::dec,
        });

        instructions[0xD6] = Some(Instruction{
            instruction_type: DEC,
            opcode: 0xD6,
            operand: None,
            addressing_mode: AddressingMode::ZeroPageIndexedX,
            operation: Self::dec,
        });
        instructions[0xDE] = Some(Instruction{
            instruction_type: DEC,
            opcode: 0xDE,
            operand: None,
            addressing_mode: AddressingMode::AbsoluteIndexedX,
            operation: Self::dec,
        });

        instructions[0xE1] = Some(Instruction{
            instruction_type: SBC,
            opcode: 0xE1,
            operand: None,
            addressing_mode: AddressingMode::IndirectIndexedX,
            operation: Self::sbc,
        });
        instructions[0xE5] = Some(Instruction{
            instruction_type: SBC,
            opcode: 0xE5,
            operand: None,
            addressing_mode: AddressingMode::ZeroPage,
            operation: Self::sbc,
        });
        instructions[0xE6] = Some(Instruction{
            instruction_type: INC,
            opcode: 0xE6,
            operand: None,
            addressing_mode: AddressingMode::ZeroPage,
            operation: Self::inc,
        });
        instructions[0xE8] = Some(Instruction{
            instruction_type: INX,
            opcode: 0xE8,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: Self::inx,
        });
        instructions[0xE9] = Some(Instruction{
            instruction_type: SBC,
            opcode: 0xE9,
            operand: None,
            addressing_mode: AddressingMode::Immediate,
            operation: Self::sbc,
        });
        instructions[0xEA] = Some(Instruction{
            instruction_type: NOP,
            opcode: 0xEA,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: Self::nop,
        });
        instructions[0xED] = Some(Instruction{
            instruction_type: SBC,
            opcode: 0xED,
            operand: None,
            addressing_mode: AddressingMode::Absolute,
            operation: Self::sbc,
        });
        instructions[0xEE] = Some(Instruction{
            instruction_type: INC,
            opcode: 0xEE,
            operand: None,
            addressing_mode: AddressingMode::Absolute,
            operation: Self::inc,
        });

        instructions[0xF1] = Some(Instruction{
            instruction_type: SBC,
            opcode: 0xF1,
            operand: None,
            addressing_mode: AddressingMode::IndirectIndexedY,
            operation: Self::sbc,
        });
        instructions[0xF5] = Some(Instruction{
            instruction_type: BEQ,
            opcode: 0xF5,
            operand: None,
            addressing_mode: AddressingMode::ZeroPageIndexedX,
            operation: Self::sbc,
        });
        instructions[0xF6] = Some(Instruction{
            instruction_type: INC,
            opcode: 0xF6,
            operand: None,
            addressing_mode: AddressingMode::ZeroPageIndexedX,
            operation: Self::inc,
        });
        instructions[0xF9] = Some(Instruction{
            instruction_type: SBC,
            opcode: 0xF9,
            operand: None,
            addressing_mode: AddressingMode::AbsoluteIndexedY,
            operation: Self::sbc,
        });
        instructions[0xFD] = Some(Instruction{
            instruction_type: SBC,
            opcode: 0xFD,
            operand: None,
            addressing_mode: AddressingMode::AbsoluteIndexedX,
            operation: Self::sbc,
        });
        instructions[0xFE] = Some(Instruction{
            instruction_type: INC,
            opcode: 0xFE,
            operand: None,
            addressing_mode: AddressingMode::AbsoluteIndexedX,
            operation: Self::inc,
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