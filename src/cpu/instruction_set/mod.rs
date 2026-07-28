mod access;
mod arithmetic;
mod bitwise;
mod branch;
pub mod compare;
mod flags;
pub mod jump;
mod nop;
mod shift;
mod stack;
pub mod transfer;

use crate::bus::MemoryIndexer;
use crate::cpu::instruction_set::InstructionType::*;
use crate::cpu::registers::Registers;

type Operation<T> = fn(&Instruction<T>, &mut Registers, &mut T) -> Result<u8, String>;

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
    NOP,
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
impl<T: MemoryIndexer> Default for InstructionSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: MemoryIndexer> InstructionSet<T> {
    pub fn new() -> InstructionSet<T> {
        let mut instructions = std::array::from_fn(|_| None);

        instructions[0x00] = Some(Instruction {
            instruction_type: BRK,
            opcode: 0x00,
            operand: None,
            addressing_mode: AddressingMode::Immediate,
            operation: Self::brk,
        });
        instructions[0x01] = Some(Instruction {
            instruction_type: AND,
            opcode: 0x01,
            operand: None,
            addressing_mode: AddressingMode::IndirectIndexedX,
            operation: Self::and,
        });
        instructions[0x05] = Some(Instruction {
            instruction_type: AND,
            opcode: 0x05,
            operand: None,
            addressing_mode: AddressingMode::ZeroPage,
            operation: Self::and,
        });
        instructions[0x06] = Some(Instruction {
            instruction_type: ASL,
            opcode: 0x06,
            operand: None,
            addressing_mode: AddressingMode::ZeroPage,
            operation: Self::asl,
        });
        instructions[0x08] = Some(Instruction {
            instruction_type: PHP,
            opcode: 0x08,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: Self::php,
        });
        instructions[0x09] = Some(Instruction {
            instruction_type: AND,
            opcode: 0x09,
            operand: None,
            addressing_mode: AddressingMode::Immediate,
            operation: Self::and,
        });
        instructions[0x0A] = Some(Instruction {
            instruction_type: ASL,
            opcode: 0x0A,
            operand: None,
            addressing_mode: AddressingMode::Accumulator,
            operation: Self::asl,
        });
        instructions[0x0D] = Some(Instruction {
            instruction_type: AND,
            opcode: 0x0D,
            operand: None,
            addressing_mode: AddressingMode::Absolute,
            operation: Self::and,
        });
        instructions[0x0E] = Some(Instruction {
            instruction_type: ASL,
            opcode: 0x0E,
            operand: None,
            addressing_mode: AddressingMode::Absolute,
            operation: Self::asl,
        });

        instructions[0x10] = Some(Instruction {
            instruction_type: AND,
            opcode: 0x11,
            operand: None,
            addressing_mode: AddressingMode::Relative,
            operation: Self::bpl,
        });
        instructions[0x11] = Some(Instruction {
            instruction_type: AND,
            opcode: 0x11,
            operand: None,
            addressing_mode: AddressingMode::IndirectIndexedY,
            operation: Self::and,
        });
        instructions[0x15] = Some(Instruction {
            instruction_type: AND,
            opcode: 0x15,
            operand: None,
            addressing_mode: AddressingMode::ZeroPageIndexedX,
            operation: Self::and,
        });
        instructions[0x16] = Some(Instruction {
            instruction_type: ASL,
            opcode: 0x16,
            operand: None,
            addressing_mode: AddressingMode::ZeroPageIndexedX,
            operation: Self::asl,
        });
        instructions[0x18] = Some(Instruction {
            instruction_type: CLC,
            opcode: 0x18,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: Self::clc,
        });
        instructions[0x19] = Some(Instruction {
            instruction_type: AND,
            opcode: 0x19,
            operand: None,
            addressing_mode: AddressingMode::AbsoluteIndexedY,
            operation: Self::and,
        });
        instructions[0x1D] = Some(Instruction {
            instruction_type: AND,
            opcode: 0x1D,
            operand: None,
            addressing_mode: AddressingMode::AbsoluteIndexedX,
            operation: Self::and,
        });
        instructions[0x1E] = Some(Instruction {
            instruction_type: ASL,
            opcode: 0x1E,
            operand: None,
            addressing_mode: AddressingMode::AbsoluteIndexedX,
            operation: Self::asl,
        });

        instructions[0x20] = Some(Instruction {
            instruction_type: JSR,
            opcode: 0x20,
            operand: None,
            addressing_mode: AddressingMode::Absolute,
            operation: Self::jsr,
        });
        instructions[0x21] = Some(Instruction {
            instruction_type: AND,
            opcode: 0x21,
            operand: None,
            addressing_mode: AddressingMode::IndirectIndexedX,
            operation: Self::and,
        });
        instructions[0x24] = Some(Instruction {
            instruction_type: BIT,
            opcode: 0x24,
            operand: None,
            addressing_mode: AddressingMode::ZeroPage,
            operation: Self::bit,
        });
        instructions[0x25] = Some(Instruction {
            instruction_type: AND,
            opcode: 0x25,
            operand: None,
            addressing_mode: AddressingMode::ZeroPage,
            operation: Self::and,
        });
        instructions[0x26] = Some(Instruction {
            instruction_type: ROL,
            opcode: 0x26,
            operand: None,
            addressing_mode: AddressingMode::ZeroPage,
            operation: Self::rol,
        });
        instructions[0x28] = Some(Instruction {
            instruction_type: PLP,
            opcode: 0x28,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: Self::plp,
        });
        instructions[0x29] = Some(Instruction {
            instruction_type: AND,
            opcode: 0x29,
            operand: None,
            addressing_mode: AddressingMode::Immediate,
            operation: Self::and,
        });
        instructions[0x2A] = Some(Instruction {
            instruction_type: ROL,
            opcode: 0x2A,
            operand: None,
            addressing_mode: AddressingMode::Accumulator,
            operation: Self::rol,
        });
        instructions[0x2C] = Some(Instruction {
            instruction_type: BIT,
            opcode: 0x2C,
            operand: None,
            addressing_mode: AddressingMode::Absolute,
            operation: Self::bit,
        });
        instructions[0x2D] = Some(Instruction {
            instruction_type: AND,
            opcode: 0x2D,
            operand: None,
            addressing_mode: AddressingMode::Absolute,
            operation: Self::and,
        });
        instructions[0x2E] = Some(Instruction {
            instruction_type: ROL,
            opcode: 0x2E,
            operand: None,
            addressing_mode: AddressingMode::Absolute,
            operation: Self::rol,
        });

        instructions[0x30] = Some(Instruction {
            instruction_type: BMI,
            opcode: 0x30,
            operand: None,
            addressing_mode: AddressingMode::Relative,
            operation: Self::bmi,
        });
        instructions[0x31] = Some(Instruction {
            instruction_type: AND,
            opcode: 0x31,
            operand: None,
            addressing_mode: AddressingMode::IndirectIndexedY,
            operation: Self::and,
        });
        instructions[0x35] = Some(Instruction {
            instruction_type: AND,
            opcode: 0x35,
            operand: None,
            addressing_mode: AddressingMode::ZeroPageIndexedX,
            operation: Self::and,
        });
        instructions[0x36] = Some(Instruction {
            instruction_type: ROL,
            opcode: 0x36,
            operand: None,
            addressing_mode: AddressingMode::ZeroPageIndexedX,
            operation: Self::rol,
        });
        instructions[0x38] = Some(Instruction {
            instruction_type: SEC,
            opcode: 0x38,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: Self::sec,
        });
        instructions[0x39] = Some(Instruction {
            instruction_type: AND,
            opcode: 0x39,
            operand: None,
            addressing_mode: AddressingMode::AbsoluteIndexedY,
            operation: Self::and,
        });
        instructions[0x3D] = Some(Instruction {
            instruction_type: AND,
            opcode: 0x3D,
            operand: None,
            addressing_mode: AddressingMode::AbsoluteIndexedX,
            operation: Self::and,
        });
        instructions[0x3E] = Some(Instruction {
            instruction_type: ROL,
            opcode: 0x3E,
            operand: None,
            addressing_mode: AddressingMode::AbsoluteIndexedX,
            operation: Self::rol,
        });

        instructions[0x40] = Some(Instruction {
            instruction_type: RTI,
            opcode: 0x40,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: Self::rti,
        });
        instructions[0x41] = Some(Instruction {
            instruction_type: EOR,
            opcode: 0x41,
            operand: None,
            addressing_mode: AddressingMode::IndirectIndexedX,
            operation: Self::eor,
        });
        instructions[0x45] = Some(Instruction {
            instruction_type: EOR,
            opcode: 0x45,
            operand: None,
            addressing_mode: AddressingMode::ZeroPage,
            operation: Self::eor,
        });
        instructions[0x46] = Some(Instruction {
            instruction_type: LSR,
            opcode: 0x46,
            operand: None,
            addressing_mode: AddressingMode::ZeroPage,
            operation: Self::lsr,
        });
        instructions[0x48] = Some(Instruction {
            instruction_type: PHA,
            opcode: 0x48,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: Self::pha,
        });
        instructions[0x49] = Some(Instruction {
            instruction_type: EOR,
            opcode: 0x49,
            operand: None,
            addressing_mode: AddressingMode::Immediate,
            operation: Self::eor,
        });
        instructions[0x4A] = Some(Instruction {
            instruction_type: LSR,
            opcode: 0x4A,
            operand: None,
            addressing_mode: AddressingMode::Accumulator,
            operation: Self::lsr,
        });
        instructions[0x4C] = Some(Instruction {
            instruction_type: EOR,
            opcode: 0x4D,
            operand: None,
            addressing_mode: AddressingMode::Absolute,
            operation: Self::jmp,
        });
        instructions[0x4D] = Some(Instruction {
            instruction_type: EOR,
            opcode: 0x4D,
            operand: None,
            addressing_mode: AddressingMode::Absolute,
            operation: Self::eor,
        });
        instructions[0x4E] = Some(Instruction {
            instruction_type: LSR,
            opcode: 0x4E,
            operand: None,
            addressing_mode: AddressingMode::Absolute,
            operation: Self::lsr,
        });

        instructions[0x50] = Some(Instruction {
            instruction_type: BVC,
            opcode: 0x51,
            operand: None,
            addressing_mode: AddressingMode::Relative,
            operation: Self::bvc,
        });
        instructions[0x51] = Some(Instruction {
            instruction_type: EOR,
            opcode: 0x51,
            operand: None,
            addressing_mode: AddressingMode::IndirectIndexedY,
            operation: Self::eor,
        });
        instructions[0x55] = Some(Instruction {
            instruction_type: EOR,
            opcode: 0x55,
            operand: None,
            addressing_mode: AddressingMode::ZeroPageIndexedX,
            operation: Self::eor,
        });
        instructions[0x56] = Some(Instruction {
            instruction_type: LSR,
            opcode: 0x56,
            operand: None,
            addressing_mode: AddressingMode::ZeroPageIndexedX,
            operation: Self::lsr,
        });
        instructions[0x58] = Some(Instruction {
            instruction_type: CLI,
            opcode: 0x58,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: Self::cli,
        });
        instructions[0x59] = Some(Instruction {
            instruction_type: EOR,
            opcode: 0x59,
            operand: None,
            addressing_mode: AddressingMode::AbsoluteIndexedY,
            operation: Self::eor,
        });
        instructions[0x5D] = Some(Instruction {
            instruction_type: EOR,
            opcode: 0x5D,
            operand: None,
            addressing_mode: AddressingMode::AbsoluteIndexedX,
            operation: Self::eor,
        });
        instructions[0x5E] = Some(Instruction {
            instruction_type: LSR,
            opcode: 0x5E,
            operand: None,
            addressing_mode: AddressingMode::AbsoluteIndexedX,
            operation: Self::lsr,
        });

        instructions[0x60] = Some(Instruction {
            instruction_type: RTS,
            opcode: 0x60,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: Self::rts,
        });
        instructions[0x61] = Some(Instruction {
            instruction_type: ADC,
            opcode: 0x61,
            operand: None,
            addressing_mode: AddressingMode::IndirectIndexedX,
            operation: Self::adc,
        });
        instructions[0x65] = Some(Instruction {
            instruction_type: ADC,
            opcode: 0x65,
            operand: None,
            addressing_mode: AddressingMode::ZeroPage,
            operation: Self::adc,
        });
        instructions[0x66] = Some(Instruction {
            instruction_type: ROR,
            opcode: 0x66,
            operand: None,
            addressing_mode: AddressingMode::ZeroPage,
            operation: Self::ror,
        });
        instructions[0x68] = Some(Instruction {
            instruction_type: PLA,
            opcode: 0x68,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: Self::pla,
        });
        instructions[0x69] = Some(Instruction {
            instruction_type: ADC,
            opcode: 0x69,
            operand: None,
            addressing_mode: AddressingMode::Immediate,
            operation: Self::adc,
        });
        instructions[0x6A] = Some(Instruction {
            instruction_type: ROR,
            opcode: 0x6A,
            operand: None,
            addressing_mode: AddressingMode::Accumulator,
            operation: Self::ror,
        });
        instructions[0x6C] = Some(Instruction {
            instruction_type: EOR,
            opcode: 0x6D,
            operand: None,
            addressing_mode: AddressingMode::Indirect,
            operation: Self::jmp,
        });
        instructions[0x6D] = Some(Instruction {
            instruction_type: ADC,
            opcode: 0x6D,
            operand: None,
            addressing_mode: AddressingMode::Absolute,
            operation: Self::adc,
        });
        instructions[0x6E] = Some(Instruction {
            instruction_type: ROR,
            opcode: 0x6E,
            operand: None,
            addressing_mode: AddressingMode::Absolute,
            operation: Self::ror,
        });

        instructions[0x70] = Some(Instruction {
            instruction_type: BVS,
            opcode: 0x70,
            operand: None,
            addressing_mode: AddressingMode::Relative,
            operation: Self::bvs,
        });
        instructions[0x71] = Some(Instruction {
            instruction_type: ADC,
            opcode: 0x71,
            operand: None,
            addressing_mode: AddressingMode::IndirectIndexedY,
            operation: Self::adc,
        });
        instructions[0x75] = Some(Instruction {
            instruction_type: ADC,
            opcode: 0x75,
            operand: None,
            addressing_mode: AddressingMode::ZeroPageIndexedX,
            operation: Self::adc,
        });

        instructions[0x76] = Some(Instruction {
            instruction_type: ROR,
            opcode: 0x76,
            operand: None,
            addressing_mode: AddressingMode::ZeroPageIndexedX,
            operation: Self::ror,
        });
        instructions[0x78] = Some(Instruction {
            instruction_type: SEI,
            opcode: 0x78,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: Self::sei,
        });
        instructions[0x79] = Some(Instruction {
            instruction_type: ADC,
            opcode: 0x79,
            operand: None,
            addressing_mode: AddressingMode::AbsoluteIndexedY,
            operation: Self::adc,
        });
        instructions[0x7D] = Some(Instruction {
            instruction_type: ADC,
            opcode: 0x7D,
            operand: None,
            addressing_mode: AddressingMode::AbsoluteIndexedX,
            operation: Self::adc,
        });
        instructions[0x7E] = Some(Instruction {
            instruction_type: ROR,
            opcode: 0x7E,
            operand: None,
            addressing_mode: AddressingMode::AbsoluteIndexedX,
            operation: Self::ror,
        });

        instructions[0x81] = Some(Instruction {
            instruction_type: STA,
            opcode: 0x81,
            operand: None,
            addressing_mode: AddressingMode::IndirectIndexedX,
            operation: Self::sta,
        });
        instructions[0x85] = Some(Instruction {
            instruction_type: STA,
            opcode: 0x85,
            operand: None,
            addressing_mode: AddressingMode::ZeroPage,
            operation: Self::sta,
        });
        instructions[0x84] = Some(Instruction {
            instruction_type: STY,
            opcode: 0x84,
            operand: None,
            addressing_mode: AddressingMode::ZeroPage,
            operation: Self::sty,
        });
        instructions[0x86] = Some(Instruction {
            instruction_type: STX,
            opcode: 0x86,
            operand: None,
            addressing_mode: AddressingMode::ZeroPage,
            operation: Self::stx,
        });
        instructions[0x88] = Some(Instruction {
            instruction_type: DEY,
            opcode: 0x88,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: Self::dey,
        });
        instructions[0x8A] = Some(Instruction {
            instruction_type: TXA,
            opcode: 0x8A,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: Self::txa,
        });
        instructions[0x8C] = Some(Instruction {
            instruction_type: STY,
            opcode: 0x8C,
            operand: None,
            addressing_mode: AddressingMode::Absolute,
            operation: Self::sty,
        });
        instructions[0x8D] = Some(Instruction {
            instruction_type: STA,
            opcode: 0x8D,
            operand: None,
            addressing_mode: AddressingMode::Absolute,
            operation: Self::sta,
        });
        instructions[0x8E] = Some(Instruction {
            instruction_type: STX,
            opcode: 0x8E,
            operand: None,
            addressing_mode: AddressingMode::Absolute,
            operation: Self::stx,
        });

        instructions[0x90] = Some(Instruction {
            instruction_type: BCC,
            opcode: 0x90,
            operand: None,
            addressing_mode: AddressingMode::Relative,
            operation: Self::bcc,
        });
        instructions[0x91] = Some(Instruction {
            instruction_type: STA,
            opcode: 0x91,
            operand: None,
            addressing_mode: AddressingMode::IndirectIndexedY,
            operation: Self::sta,
        });
        instructions[0x95] = Some(Instruction {
            instruction_type: STA,
            opcode: 0x95,
            operand: None,
            addressing_mode: AddressingMode::ZeroPageIndexedX,
            operation: Self::sta,
        });
        instructions[0x94] = Some(Instruction {
            instruction_type: STY,
            opcode: 0x94,
            operand: None,
            addressing_mode: AddressingMode::ZeroPageIndexedX,
            operation: Self::sty,
        });
        instructions[0x96] = Some(Instruction {
            instruction_type: STX,
            opcode: 0x96,
            operand: None,
            addressing_mode: AddressingMode::ZeroPageIndexedY,
            operation: Self::stx,
        });
        instructions[0x98] = Some(Instruction {
            instruction_type: TYA,
            opcode: 0x98,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: Self::tya,
        });
        instructions[0x99] = Some(Instruction {
            instruction_type: STA,
            opcode: 0x99,
            operand: None,
            addressing_mode: AddressingMode::AbsoluteIndexedY,
            operation: Self::sta,
        });
        instructions[0x9A] = Some(Instruction {
            instruction_type: TXS,
            opcode: 0x9A,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: Self::txs,
        });
        instructions[0x9D] = Some(Instruction {
            instruction_type: STA,
            opcode: 0x9D,
            operand: None,
            addressing_mode: AddressingMode::AbsoluteIndexedY,
            operation: Self::sta,
        });

        instructions[0xA0] = Some(Instruction {
            instruction_type: LDY,
            opcode: 0xA0,
            operand: None,
            addressing_mode: AddressingMode::Immediate,
            operation: Self::ldy,
        });
        instructions[0xA1] = Some(Instruction {
            instruction_type: LDA,
            opcode: 0xA1,
            operand: None,
            addressing_mode: AddressingMode::IndirectIndexedX,
            operation: Self::lda,
        });
        instructions[0xA2] = Some(Instruction {
            instruction_type: LDX,
            opcode: 0xA2,
            operand: None,
            addressing_mode: AddressingMode::Immediate,
            operation: Self::ldx,
        });
        instructions[0xA4] = Some(Instruction {
            instruction_type: LDY,
            opcode: 0xA4,
            operand: None,
            addressing_mode: AddressingMode::ZeroPage,
            operation: Self::ldy,
        });
        instructions[0xA5] = Some(Instruction {
            instruction_type: LDA,
            opcode: 0xA5,
            operand: None,
            addressing_mode: AddressingMode::ZeroPage,
            operation: Self::lda,
        });
        instructions[0xA6] = Some(Instruction {
            instruction_type: LDX,
            opcode: 0xA6,
            operand: None,
            addressing_mode: AddressingMode::ZeroPage,
            operation: Self::ldx,
        });
        instructions[0xA8] = Some(Instruction {
            instruction_type: TAY,
            opcode: 0xA8,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: Self::tay,
        });
        instructions[0xA9] = Some(Instruction {
            instruction_type: LDA,
            opcode: 0xA9,
            operand: None,
            addressing_mode: AddressingMode::Immediate,
            operation: Self::lda,
        });
        instructions[0xAA] = Some(Instruction {
            instruction_type: TAX,
            opcode: 0xAA,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: Self::tax,
        });
        instructions[0xAC] = Some(Instruction {
            instruction_type: LDY,
            opcode: 0xAC,
            operand: None,
            addressing_mode: AddressingMode::Absolute,
            operation: Self::ldy,
        });
        instructions[0xAD] = Some(Instruction {
            instruction_type: LDA,
            opcode: 0xAD,
            operand: None,
            addressing_mode: AddressingMode::Absolute,
            operation: Self::lda,
        });
        instructions[0xAE] = Some(Instruction {
            instruction_type: LDX,
            opcode: 0xAE,
            operand: None,
            addressing_mode: AddressingMode::Absolute,
            operation: Self::ldx,
        });

        instructions[0xB0] = Some(Instruction {
            instruction_type: BCS,
            opcode: 0xB0,
            operand: None,
            addressing_mode: AddressingMode::Relative,
            operation: Self::bcs,
        });
        instructions[0xB1] = Some(Instruction {
            instruction_type: LDA,
            opcode: 0xB1,
            operand: None,
            addressing_mode: AddressingMode::IndirectIndexedY,
            operation: Self::lda,
        });
        instructions[0xB4] = Some(Instruction {
            instruction_type: LDY,
            opcode: 0xB4,
            operand: None,
            addressing_mode: AddressingMode::ZeroPageIndexedX,
            operation: Self::ldy,
        });
        instructions[0xB5] = Some(Instruction {
            instruction_type: LDA,
            opcode: 0xB5,
            operand: None,
            addressing_mode: AddressingMode::ZeroPageIndexedX,
            operation: Self::lda,
        });
        instructions[0xB6] = Some(Instruction {
            instruction_type: LDX,
            opcode: 0xB6,
            operand: None,
            addressing_mode: AddressingMode::ZeroPageIndexedY,
            operation: Self::ldx,
        });
        instructions[0xB8] = Some(Instruction {
            instruction_type: CLV,
            opcode: 0xB8,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: Self::clv,
        });
        instructions[0xB9] = Some(Instruction {
            instruction_type: LDA,
            opcode: 0xB9,
            operand: None,
            addressing_mode: AddressingMode::AbsoluteIndexedY,
            operation: Self::lda,
        });
        instructions[0xBA] = Some(Instruction {
            instruction_type: TSX,
            opcode: 0xBA,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: Self::tsx,
        });
        instructions[0xBC] = Some(Instruction {
            instruction_type: LDY,
            opcode: 0xBC,
            operand: None,
            addressing_mode: AddressingMode::AbsoluteIndexedX,
            operation: Self::ldy,
        });
        instructions[0xBD] = Some(Instruction {
            instruction_type: LDA,
            opcode: 0xBD,
            operand: None,
            addressing_mode: AddressingMode::AbsoluteIndexedX,
            operation: Self::lda,
        });
        instructions[0xBE] = Some(Instruction {
            instruction_type: LDX,
            opcode: 0xBE,
            operand: None,
            addressing_mode: AddressingMode::AbsoluteIndexedY,
            operation: Self::ldx,
        });

        instructions[0xC0] = Some(Instruction {
            instruction_type: CPY,
            opcode: 0xC0,
            operand: None,
            addressing_mode: AddressingMode::Immediate,
            operation: Self::cpy,
        });
        instructions[0xC1] = Some(Instruction {
            instruction_type: CMP,
            opcode: 0xC1,
            operand: None,
            addressing_mode: AddressingMode::IndirectIndexedX,
            operation: Self::cmp,
        });
        instructions[0xC4] = Some(Instruction {
            instruction_type: CPY,
            opcode: 0xC4,
            operand: None,
            addressing_mode: AddressingMode::ZeroPage,
            operation: Self::cpy,
        });
        instructions[0xC5] = Some(Instruction {
            instruction_type: CMP,
            opcode: 0xC5,
            operand: None,
            addressing_mode: AddressingMode::ZeroPage,
            operation: Self::cmp,
        });
        instructions[0xC6] = Some(Instruction {
            instruction_type: DEC,
            opcode: 0xAD,
            operand: None,
            addressing_mode: AddressingMode::ZeroPage,
            operation: Self::dec,
        });
        instructions[0xC9] = Some(Instruction {
            instruction_type: CMP,
            opcode: 0xC9,
            operand: None,
            addressing_mode: AddressingMode::Immediate,
            operation: Self::cmp,
        });
        instructions[0xCA] = Some(Instruction {
            instruction_type: DEX,
            opcode: 0xCA,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: Self::dex,
        });
        instructions[0xCC] = Some(Instruction {
            instruction_type: CPY,
            opcode: 0xCC,
            operand: None,
            addressing_mode: AddressingMode::Absolute,
            operation: Self::cpy,
        });
        instructions[0xCD] = Some(Instruction {
            instruction_type: CMP,
            opcode: 0xCD,
            operand: None,
            addressing_mode: AddressingMode::Absolute,
            operation: Self::cmp,
        });
        instructions[0xCE] = Some(Instruction {
            instruction_type: DEC,
            opcode: 0xCE,
            operand: None,
            addressing_mode: AddressingMode::Absolute,
            operation: Self::dec,
        });

        instructions[0xD0] = Some(Instruction {
            instruction_type: BNE,
            opcode: 0xD0,
            operand: None,
            addressing_mode: AddressingMode::Relative,
            operation: Self::bne,
        });
        instructions[0xD1] = Some(Instruction {
            instruction_type: CMP,
            opcode: 0xD1,
            operand: None,
            addressing_mode: AddressingMode::IndirectIndexedY,
            operation: Self::cmp,
        });
        instructions[0xD5] = Some(Instruction {
            instruction_type: CMP,
            opcode: 0xD5,
            operand: None,
            addressing_mode: AddressingMode::ZeroPageIndexedX,
            operation: Self::cmp,
        });
        instructions[0xD6] = Some(Instruction {
            instruction_type: DEC,
            opcode: 0xD6,
            operand: None,
            addressing_mode: AddressingMode::ZeroPageIndexedX,
            operation: Self::dec,
        });
        instructions[0xD8] = Some(Instruction {
            instruction_type: CLD,
            opcode: 0xD8,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: Self::cld,
        });
        instructions[0xD9] = Some(Instruction {
            instruction_type: CMP,
            opcode: 0xD9,
            operand: None,
            addressing_mode: AddressingMode::AbsoluteIndexedY,
            operation: Self::cmp,
        });
        instructions[0xDD] = Some(Instruction {
            instruction_type: CMP,
            opcode: 0xDD,
            operand: None,
            addressing_mode: AddressingMode::AbsoluteIndexedX,
            operation: Self::cmp,
        });
        instructions[0xDE] = Some(Instruction {
            instruction_type: DEC,
            opcode: 0xDE,
            operand: None,
            addressing_mode: AddressingMode::AbsoluteIndexedX,
            operation: Self::dec,
        });

        instructions[0xE0] = Some(Instruction {
            instruction_type: CPX,
            opcode: 0xE0,
            operand: None,
            addressing_mode: AddressingMode::Immediate,
            operation: Self::cpx,
        });
        instructions[0xE1] = Some(Instruction {
            instruction_type: SBC,
            opcode: 0xE1,
            operand: None,
            addressing_mode: AddressingMode::IndirectIndexedX,
            operation: Self::sbc,
        });
        instructions[0xE4] = Some(Instruction {
            instruction_type: CPX,
            opcode: 0xE4,
            operand: None,
            addressing_mode: AddressingMode::ZeroPage,
            operation: Self::cpx,
        });
        instructions[0xE5] = Some(Instruction {
            instruction_type: SBC,
            opcode: 0xE5,
            operand: None,
            addressing_mode: AddressingMode::ZeroPage,
            operation: Self::sbc,
        });
        instructions[0xE6] = Some(Instruction {
            instruction_type: INC,
            opcode: 0xE6,
            operand: None,
            addressing_mode: AddressingMode::ZeroPage,
            operation: Self::inc,
        });
        instructions[0xE8] = Some(Instruction {
            instruction_type: INX,
            opcode: 0xE8,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: Self::inx,
        });
        instructions[0xE9] = Some(Instruction {
            instruction_type: SBC,
            opcode: 0xE9,
            operand: None,
            addressing_mode: AddressingMode::Immediate,
            operation: Self::sbc,
        });
        instructions[0xEA] = Some(Instruction {
            instruction_type: NOP,
            opcode: 0xEA,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: Self::nop,
        });
        instructions[0xEC] = Some(Instruction {
            instruction_type: CPX,
            opcode: 0xEC,
            operand: None,
            addressing_mode: AddressingMode::Absolute,
            operation: Self::cpx,
        });
        instructions[0xED] = Some(Instruction {
            instruction_type: SBC,
            opcode: 0xED,
            operand: None,
            addressing_mode: AddressingMode::Absolute,
            operation: Self::sbc,
        });
        instructions[0xEE] = Some(Instruction {
            instruction_type: INC,
            opcode: 0xEE,
            operand: None,
            addressing_mode: AddressingMode::Absolute,
            operation: Self::inc,
        });

        instructions[0xF6] = Some(Instruction {
            instruction_type: BEQ,
            opcode: 0xF6,
            operand: None,
            addressing_mode: AddressingMode::Relative,
            operation: Self::beq,
        });
        instructions[0xF1] = Some(Instruction {
            instruction_type: SBC,
            opcode: 0xF1,
            operand: None,
            addressing_mode: AddressingMode::IndirectIndexedY,
            operation: Self::sbc,
        });
        instructions[0xF5] = Some(Instruction {
            instruction_type: BEQ,
            opcode: 0xF5,
            operand: None,
            addressing_mode: AddressingMode::ZeroPageIndexedX,
            operation: Self::sbc,
        });
        instructions[0xF6] = Some(Instruction {
            instruction_type: INC,
            opcode: 0xF6,
            operand: None,
            addressing_mode: AddressingMode::ZeroPageIndexedX,
            operation: Self::inc,
        });
        instructions[0xF8] = Some(Instruction {
            instruction_type: SED,
            opcode: 0xF8,
            operand: None,
            addressing_mode: AddressingMode::Implicit,
            operation: Self::sed,
        });
        instructions[0xF9] = Some(Instruction {
            instruction_type: SBC,
            opcode: 0xF9,
            operand: None,
            addressing_mode: AddressingMode::AbsoluteIndexedY,
            operation: Self::sbc,
        });
        instructions[0xFD] = Some(Instruction {
            instruction_type: SBC,
            opcode: 0xFD,
            operand: None,
            addressing_mode: AddressingMode::AbsoluteIndexedX,
            operation: Self::sbc,
        });
        instructions[0xFE] = Some(Instruction {
            instruction_type: INC,
            opcode: 0xFE,
            operand: None,
            addressing_mode: AddressingMode::AbsoluteIndexedX,
            operation: Self::inc,
        });

        InstructionSet { instructions }
    }

    pub fn get_instruction(&self, opcode: u8) -> Option<Instruction<T>> {
        self.instructions[opcode as usize]
    }
}
