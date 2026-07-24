use crate::bus::MemoryIndexer;
use crate::cpu::instruction_set::InstructionType::LDA;
use crate::cpu::registers::Registers;

type Operation<T: MemoryIndexer> = fn(&Registers, &mut T);

#[derive(Copy, Clone)]
pub enum AddressingMode {
    ZeroPageX,
    ZeroPageY,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
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

    fn lda(registers: &Registers, bus: &mut T) {

    }
}