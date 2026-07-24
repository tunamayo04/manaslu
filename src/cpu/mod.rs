use crate::bus::MemoryIndexer;
use crate::cpu::instruction_set::{AddressingMode, Instruction, InstructionSet};
use crate::cpu::registers::Registers;

pub mod registers;
pub mod instruction_set;

pub const NMI_VECTOR: u16 = 0xFFFA;
pub const RST_VECTOR: u16 = 0xFFFC;
pub const IRQ_VECTOR: u16 = 0xFFFE;

pub struct CPU<T: MemoryIndexer> {
    registers: Registers,
    instruction_set: InstructionSet<T>,
}
impl<T: MemoryIndexer> CPU<T> {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            instruction_set: InstructionSet::new(),
        }
    }

    pub fn reset(&mut self, bus: &T) {
        self.registers.reset();

        let reset_address = bus.read_word(RST_VECTOR);
        self.registers.program_counter = reset_address;
    }

    fn step(&mut self, bus: &mut T) -> Result<(),String> {
        let next_instruction = self.fetch_next_instruction(bus)?;

        (next_instruction.operation)(&self.registers, bus);

        Ok(())
    }

    fn fetch_next_instruction(&mut self, bus: &mut T) -> Result<Instruction<T>, String> {
        let opcode = bus.read_byte(self.registers.program_counter);
        let Some(mut instruction) = self.instruction_set.get_instruction(opcode) else {
            return Err(String::from("Unknown opcode"));
        };

        self.registers.increment_program_counter(1);

        instruction.operand = self.fetch_operand(instruction.addressing_mode, bus);

        Ok(instruction)
    }

    fn fetch_operand(&mut self, addressing_mode: AddressingMode, bus: &mut T) -> Option<u16> {
        match addressing_mode {
            AddressingMode::ZeroPageX => None,
            AddressingMode::ZeroPageY => None,
            AddressingMode::AbsoluteX => None,
            AddressingMode::AbsoluteY => None,
            AddressingMode::IndirectX => None,
            AddressingMode::IndirectY => None,
            AddressingMode::Implicit => None,
            AddressingMode::Accumulator => Some(self.registers.accumulator as u16),
            AddressingMode::Immediate => {
                let value = bus.read_byte(self.registers.program_counter);
                self.registers.increment_program_counter(1);

                Some(value as u16)
            }
            AddressingMode::ZeroPage => None,
            AddressingMode::Absolute => {
                let address = bus.read_word(self.registers.program_counter);
                self.registers.increment_program_counter(2);

                Some(bus.read_byte(address) as u16)
            }
            AddressingMode::Relative => {
                let offset = bus.read_byte(self.registers.program_counter) as i8;
                self.registers.increment_program_counter(1);

                Some(self.registers.program_counter + offset)
            }
            AddressingMode::Indirect => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestBus {
        memory: Vec<u8>,
    }
    impl TestBus {
        fn new() -> Self {
            TestBus {
                memory: vec![0; 0xFFFF],
            }
        }
    }
    impl MemoryIndexer for TestBus {
        fn read_byte(&self, address: u16) -> u8 {
            self.memory[address as usize]
        }
        fn write_byte(&mut self, address: u16, value: u8) {
            self.memory[address as usize] = value;
        }

        fn read_word(&self, address: u16) -> u16 {
            let lower_byte = self.memory[address as usize];
            let upper_byte = self.memory[address.wrapping_add(1) as usize];
            u16::from_le_bytes([lower_byte, upper_byte])
        }

        fn write_word(&mut self, address: u16, value: u16) {
            let [lower_byte, upper_byte] = value.to_le_bytes();

            self.memory[address as usize] = lower_byte;
            self.memory[address.wrapping_add(1) as usize] = upper_byte;
        }
    }

    #[test]
    fn reset_resets_registers() {
        let mut cpu = CPU::new();
        let mut test_bus = TestBus::new();
        test_bus.write_word(RST_VECTOR, 0xBEEF);

        cpu.registers.program_counter = 123;
        cpu.registers.x = 43;
        cpu.registers.y = 12;
        cpu.registers.accumulator = 92;
        cpu.registers.stack_pointer = 67;
        cpu.registers.flags = 122;

        cpu.reset(&mut test_bus);
        assert_eq!(cpu.registers.x, 0);
        assert_eq!(cpu.registers.y, 0);
        assert_eq!(cpu.registers.accumulator, 0);
        assert_eq!(cpu.registers.stack_pointer, 0);
        assert_eq!(cpu.registers.flags, 0b0010_0000);
        assert_eq!(cpu.registers.program_counter, 0xBEEF);
    }

    #[test]
    fn fetch_next_instruction_reads_at_current_pc() {
        let mut cpu = CPU::new();
        let mut test_bus = TestBus::new();

        let program_counter = 0xBEEF;
        cpu.registers.program_counter = program_counter;
        test_bus.write_byte(program_counter, 0xA9);

        let instruction = cpu.fetch_next_instruction(&mut test_bus);
        assert!(instruction.is_ok());
        assert_eq!(instruction.unwrap().opcode, 0xA9);
    }
}