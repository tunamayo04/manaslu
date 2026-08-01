use log::info;
use crate::bus::MemoryIndexer;
use crate::cpu::instruction_set::{AddressingMode, Instruction, InstructionSet, Operand};
use crate::cpu::registers::Registers;

pub mod instruction_set;
pub mod registers;

pub const NMI_VECTOR: u16 = 0xFFFA;
pub const RST_VECTOR: u16 = 0xFFFC;
pub const IRQ_VECTOR: u16 = 0xFFFE;

pub struct CPU<T: MemoryIndexer> {
    registers: Registers,
    instruction_set: InstructionSet<T>,
}
impl<T: MemoryIndexer> Default for CPU<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: MemoryIndexer> CPU<T> {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            instruction_set: InstructionSet::new(),
        }
    }

    pub fn reset(&mut self, bus: &T) {
        let reset_address = bus.read_word(RST_VECTOR);
        self.registers.reset(reset_address);
    }

    pub fn reset_at_address(&mut self, address: u16) {
        self.registers.reset(address);
    }

    pub fn step(&mut self, bus: &mut T) -> Result<(), String> {
        let starting_program_counter = self.registers.program_counter;
        let next_instruction = self.fetch_next_instruction(bus)?;

        let (opcode_bytes, operand_display) = self.format_instruction_display(
            &next_instruction,
            starting_program_counter,
            bus,
        );

        info!("{:04X}  {:<9} {:<32} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
            starting_program_counter,
            opcode_bytes,
            operand_display,
            self.registers.accumulator,
            self.registers.x,
            self.registers.y,
            self.registers.flags,
            self.registers.stack_pointer,
        );

        (next_instruction.operation)(&next_instruction, &mut self.registers, bus)?;

        Ok(())
    }

    fn format_instruction_display(
        &self,
        instruction: &Instruction<T>,
        starting_pc: u16,
        bus: &T,
    ) -> (String, String) {
        use std::fmt::Debug;

        let mnemonic = format!("{:?}", instruction.instruction_type);

        match instruction.addressing_mode {
            AddressingMode::Implicit => (
                format!("{:02X}", instruction.opcode),
                mnemonic,
            ),
            AddressingMode::Accumulator => (
                format!("{:02X}", instruction.opcode),
                format!("{mnemonic} A"),
            ),
            AddressingMode::Immediate => {
                if let Some(Operand::Value(val)) = instruction.operand {
                    (
                        format!("{:02X} {:02X}", instruction.opcode, val),
                        format!("{} #${:02X}", mnemonic, val),
                    )
                } else {
                    let val = bus.read_byte(starting_pc + 1);
                    (
                        format!("{:02X} {:02X}", instruction.opcode, val),
                        format!("{} #${:02X}", mnemonic, val),
                    )
                }
            },
            AddressingMode::ZeroPage => {
                let addr = bus.read_byte(starting_pc + 1) as u16;
                let value = bus.read_byte(addr);
                (
                    format!("{:02X} {:02X}", instruction.opcode, addr as u8),
                    format!("{} ${:02X} = {:02X}", mnemonic, addr as u8, value),
                )
            },
            AddressingMode::ZeroPageIndexedX => {
                let base = bus.read_byte(starting_pc + 1);
                let effective = base.wrapping_add(self.registers.x) as u16;
                let value = bus.read_byte(effective);
                (
                    format!("{:02X} {:02X}", instruction.opcode, base),
                    format!("{} ${:02X},X @ {:02X} = {:02X}", mnemonic, base, effective as u8, value),
                )
            },
            AddressingMode::ZeroPageIndexedY => {
                let base = bus.read_byte(starting_pc + 1);
                let effective = base.wrapping_add(self.registers.y) as u16;
                let value = bus.read_byte(effective);
                (
                    format!("{:02X} {:02X}", instruction.opcode, base),
                    format!("{} ${:02X},Y @ {:02X} = {:02X}", mnemonic, base, effective as u8, value),
                )
            },
            AddressingMode::Absolute => {
                let low = bus.read_byte(starting_pc + 1);
                let high = bus.read_byte(starting_pc + 2);
                let addr = (high as u16) << 8 | (low as u16);
                let bytes = format!("{:02X} {:02X} {:02X}", instruction.opcode, low, high);

                // Control flow instructions (JSR, JMP) don't show the value, others do
                let display = if mnemonic == "JSR" || mnemonic == "JMP" {
                    format!("{} ${:04X}", mnemonic, addr)
                } else {
                    let value = bus.read_byte(addr);
                    format!("{} ${:04X} = {:02X}", mnemonic, addr, value)
                };

                (bytes, display)
            },
            AddressingMode::AbsoluteIndexedX => {
                let low = bus.read_byte(starting_pc + 1);
                let high = bus.read_byte(starting_pc + 2);
                let base_addr = (high as u16) << 8 | (low as u16);
                let effective_addr = base_addr.wrapping_add(self.registers.x as u16);
                let value = bus.read_byte(effective_addr);
                let bytes = format!("{:02X} {:02X} {:02X}", instruction.opcode, low, high);

                (
                    bytes,
                    format!("{} ${:04X},X @ {:04X} = {:02X}", mnemonic, base_addr, effective_addr, value),
                )
            },
            AddressingMode::AbsoluteIndexedY => {
                let low = bus.read_byte(starting_pc + 1);
                let high = bus.read_byte(starting_pc + 2);
                let base_addr = (high as u16) << 8 | (low as u16);
                let effective_addr = base_addr.wrapping_add(self.registers.y as u16);
                let value = bus.read_byte(effective_addr);
                let bytes = format!("{:02X} {:02X} {:02X}", instruction.opcode, low, high);

                (
                    bytes,
                    format!("{} ${:04X},Y @ {:04X} = {:02X}", mnemonic, base_addr, effective_addr, value),
                )
            },
            AddressingMode::Indirect => {
                let low = bus.read_byte(starting_pc + 1);
                let high = bus.read_byte(starting_pc + 2);
                let addr = (high as u16) << 8 | (low as u16);
                let bytes = format!("{:02X} {:02X} {:02X}", instruction.opcode, low, high);

                if let Some(Operand::Address(target)) = instruction.operand {
                    (
                        bytes,
                        format!("{} (${:04X}) = {:04X}", mnemonic, addr, target),
                    )
                } else {
                    (bytes, format!("{} (${:04X})", mnemonic, addr))
                }
            },
            AddressingMode::IndirectIndexedX => {
                let base = bus.read_byte(starting_pc + 1);
                let lookup_addr = base.wrapping_add(self.registers.x);
                let effective_addr = bus.read_word(lookup_addr as u16);
                let value = bus.read_byte(effective_addr);

                (
                    format!("{:02X} {:02X}", instruction.opcode, base),
                    format!("{} (${:02X},X) @ {:02X} = {:04X} = {:02X}", mnemonic, base, lookup_addr, effective_addr, value),
                )
            },
            AddressingMode::IndirectIndexedY => {
                let base = bus.read_byte(starting_pc + 1);
                let base_addr = bus.read_word(base as u16);
                let effective_addr = base_addr.wrapping_add(self.registers.y as u16);
                let value = bus.read_byte(effective_addr);

                (
                    format!("{:02X} {:02X}", instruction.opcode, base),
                    format!("{} (${:02X}),Y = {:04X} @ {:04X} = {:02X}", mnemonic, base, base_addr, effective_addr, value),
                )
            },
            AddressingMode::Relative => {
                let offset = bus.read_byte(starting_pc + 1) as i8;
                let target = if let Some(Operand::Address(target)) = instruction.operand {
                    target
                } else {
                    (starting_pc + 2).wrapping_add_signed(offset as i16)
                };

                (
                    format!("{:02X} {:02X}", instruction.opcode, offset as u8),
                    format!("{} ${:04X}", mnemonic, target),
                )
            },
        }
    }

    fn fetch_next_instruction(&mut self, bus: &mut T) -> Result<Instruction<T>, String> {
        let opcode = bus.read_byte(self.registers.program_counter);
        let Some(mut instruction) = self.instruction_set.get_instruction(opcode) else {
            return Err(format!("Invalid opcode: {:02X} at PC: {:04X}", opcode, self.registers.program_counter));
        };

        self.registers.increment_program_counter(1);

        instruction.operand = self.fetch_operand(instruction.addressing_mode, bus);

        Ok(instruction)
    }

    fn fetch_operand(&mut self, addressing_mode: AddressingMode, bus: &mut T) -> Option<Operand> {
        match addressing_mode {
            AddressingMode::ZeroPageIndexedX => {
                let address = bus.read_byte(self.registers.program_counter);
                self.registers.increment_program_counter(1);

                let effective_address = address.wrapping_add(self.registers.x);

                Some(Operand::Address(effective_address as u16))
            }
            AddressingMode::ZeroPageIndexedY => {
                let address = bus.read_byte(self.registers.program_counter);
                self.registers.increment_program_counter(1);

                let effective_address = address.wrapping_add(self.registers.y);

                Some(Operand::Address(effective_address as u16))
            }
            AddressingMode::AbsoluteIndexedX => {
                let address = bus.read_word(self.registers.program_counter);
                self.registers.increment_program_counter(2);

                let effective_address = address.wrapping_add(self.registers.x as u16);

                Some(Operand::Address(effective_address))
            }
            AddressingMode::AbsoluteIndexedY => {
                let address = bus.read_word(self.registers.program_counter);
                self.registers.increment_program_counter(2);

                let effective_address = address.wrapping_add(self.registers.y as u16);

                Some(Operand::Address(effective_address))
            }
            AddressingMode::IndirectIndexedX => {
                let address = bus.read_byte(self.registers.program_counter);
                self.registers.increment_program_counter(1);

                let lookup_address = address.wrapping_add(self.registers.x);
                let effective_address = bus.read_word(lookup_address as u16);

                Some(Operand::Address(effective_address))
            }
            AddressingMode::IndirectIndexedY => {
                let address = bus.read_byte(self.registers.program_counter);
                self.registers.increment_program_counter(1);

                let base_address = bus.read_word(address as u16);
                let effective_address = base_address.wrapping_add(self.registers.y as u16);

                Some(Operand::Address(effective_address))
            }
            AddressingMode::Implicit => None,
            AddressingMode::Accumulator => Some(Operand::Value(self.registers.accumulator)),
            AddressingMode::Immediate => {
                let value = bus.read_byte(self.registers.program_counter);
                self.registers.increment_program_counter(1);

                Some(Operand::Value(value))
            }
            AddressingMode::ZeroPage => {
                let address = bus.read_byte(self.registers.program_counter);
                self.registers.increment_program_counter(1);

                Some(Operand::Address(address as u16))
            }
            AddressingMode::Absolute => {
                let address = bus.read_word(self.registers.program_counter);
                self.registers.increment_program_counter(2);

                Some(Operand::Address(address))
            }
            AddressingMode::Relative => {
                let offset = bus.read_byte(self.registers.program_counter) as i8;
                self.registers.increment_program_counter(1);

                Some(Operand::Address(
                    self.registers
                        .program_counter
                        .wrapping_add_signed(offset as i16),
                ))
            }
            AddressingMode::Indirect => {
                let effective_address = bus.read_word(self.registers.program_counter);
                self.registers.increment_program_counter(2);

                // 6502 JMP Indirect bug: if effective_address is $xxFF, the high byte
                // is fetched from $xx00 instead of $(xx+1)00
                let lo = bus.read_byte(effective_address) as u16;
                let hi_addr = if (effective_address & 0x00FF) == 0x00FF {
                    effective_address & 0xFF00
                } else {
                    effective_address + 1
                };
                let hi = bus.read_byte(hi_addr) as u16;
                let effective_target = (hi << 8) | lo;

                Some(Operand::Address(effective_target))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::testing::TestBus;
    use std::assert_eq;

    #[test]
    fn jmp_indirect_bug_page_boundary() {
        let mut cpu = CPU::new();
        let mut test_bus = TestBus::new();
        
        test_bus.write_byte(0x0100, 0x6C);
        test_bus.write_byte(0x0101, 0xFF);
        test_bus.write_byte(0x0102, 0x02);
        
        test_bus.write_byte(0x02FF, 0x77);
        test_bus.write_byte(0x0200, 0x88);
        test_bus.write_byte(0x0300, 0x99);
        
        cpu.registers.program_counter = 0x0100;
        cpu.step(&mut test_bus).unwrap();
        
        assert_eq!(cpu.registers.program_counter, 0x8877);
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

    // region Instructions
    #[test]
    fn lda_sets_accumulator() {}
    #[test]
    fn implied_addressing_return_no_operand() {
        let mut cpu = CPU::new();
        let mut test_bus = TestBus::new();

        cpu.registers.program_counter = 0xBEEF;
        test_bus.write_byte(0xBEEF, 0xA9);

        let operand = cpu.fetch_operand(AddressingMode::Implicit, &mut test_bus);
        assert!(operand.is_none());
    }

    #[test]
    fn immediate_addressing_returns_correct_operand() {
        let mut cpu = CPU::new();
        let mut test_bus = TestBus::new();

        cpu.registers.program_counter = 0xBEEF;
        test_bus.write_byte(0xBEEF, 0xA9);

        let operand = cpu.fetch_operand(AddressingMode::Immediate, &mut test_bus);
        assert_eq!(operand, Some(Operand::Value(0xA9)));
    }

    #[test]
    fn absolute_addressing_returns_correct_operand() {
        let mut cpu = CPU::new();
        let mut test_bus = TestBus::new();

        let address = 0x4242;
        cpu.registers.program_counter = 0xBEEF;
        test_bus.write_word(0xBEEF, address);
        test_bus.write_byte(address, 0xA9);

        let operand = cpu.fetch_operand(AddressingMode::Absolute, &mut test_bus);
        assert_eq!(operand, Some(Operand::Address(address)));
    }

    #[test]
    fn zero_page_addressing_returns_correct_operand() {
        let mut cpu = CPU::new();
        let mut test_bus = TestBus::new();

        let address = 0x42;
        cpu.registers.program_counter = 0xBEEF;
        test_bus.write_byte(0xBEEF, address);
        test_bus.write_byte(address as u16, 0xA9);

        let operand = cpu.fetch_operand(AddressingMode::ZeroPage, &mut test_bus);
        assert_eq!(operand, Some(Operand::Address(address as u16)));
    }

    #[test]
    fn absolute_x_indexed_addressing_returns_correct_operand() {
        let mut cpu = CPU::new();
        let mut test_bus = TestBus::new();

        let address = 0x3120;
        cpu.registers.program_counter = 0xBEEF;
        cpu.registers.x = 0x12;
        test_bus.write_word(0xBEEF, address);
        test_bus.write_byte(0x3132, 0x78);

        let operand = cpu.fetch_operand(AddressingMode::AbsoluteIndexedX, &mut test_bus);
        assert_eq!(operand, Some(Operand::Address(0x3132)));
    }

    #[test]
    fn absolute_y_indexed_addressing_returns_correct_operand() {
        let mut cpu = CPU::new();
        let mut test_bus = TestBus::new();

        let address = 0x3120;
        cpu.registers.program_counter = 0xBEEF;
        cpu.registers.y = 0x12;
        test_bus.write_word(0xBEEF, address);
        test_bus.write_byte(0x3132, 0x78);

        let operand = cpu.fetch_operand(AddressingMode::AbsoluteIndexedY, &mut test_bus);
        assert_eq!(operand, Some(Operand::Address(0x3132)));
    }

    #[test]
    fn zero_paged_x_indexed_addressing_returns_correct_operand() {
        let mut cpu = CPU::new();
        let mut test_bus = TestBus::new();

        let address = 0x80;
        cpu.registers.program_counter = 0xBEEF;
        cpu.registers.x = 0x02;
        test_bus.write_byte(0xBEEF, address);
        test_bus.write_byte(0x82, 0x64);

        let operand = cpu.fetch_operand(AddressingMode::ZeroPageIndexedX, &mut test_bus);
        assert_eq!(operand, Some(Operand::Address(0x82)));
    }

    #[test]
    fn zero_paged_y_indexed_addressing_returns_correct_operand() {
        let mut cpu = CPU::new();
        let mut test_bus = TestBus::new();

        let address = 0x80;
        cpu.registers.program_counter = 0xBEEF;
        cpu.registers.y = 0x02;
        test_bus.write_byte(0xBEEF, address);
        test_bus.write_byte(0x82, 0x64);

        let operand = cpu.fetch_operand(AddressingMode::ZeroPageIndexedY, &mut test_bus);
        assert_eq!(operand, Some(Operand::Address(0x82)));
    }

    #[test]
    fn indirect_addressing_returns_correct_operand() {
        let mut cpu = CPU::new();
        let mut test_bus = TestBus::new();

        let first_address = 0x82FF;
        cpu.registers.program_counter = 0xBEEF;
        test_bus.write_word(0xBEEF, first_address);
        test_bus.write_word(first_address, 0x80C4);

        let operand = cpu.fetch_operand(AddressingMode::Indirect, &mut test_bus);
        assert_eq!(operand, Some(Operand::Address(0x80C4)));
    }

    #[test]
    fn preindexed_indirect_x_addressing_returns_correct_operand() {
        let mut cpu = CPU::new();
        let mut test_bus = TestBus::new();

        let first_address = 0x70;
        let second_address = 0x3023;
        cpu.registers.program_counter = 0xBEEF;
        cpu.registers.x = 0x05;
        test_bus.write_word(0xBEEF, first_address);
        test_bus.write_word(0x75, second_address);
        test_bus.write_byte(second_address, 0xA5);

        let operand = cpu.fetch_operand(AddressingMode::IndirectIndexedX, &mut test_bus);
        assert_eq!(operand, Some(Operand::Address(second_address)));
    }

    #[test]
    fn preindexed_indirect_y_addressing_returns_correct_operand() {
        let mut cpu = CPU::new();
        let mut test_bus = TestBus::new();

        let first_address = 0x70;
        let second_address = 0x3543;
        cpu.registers.program_counter = 0xBEEF;
        cpu.registers.y = 0x10;
        test_bus.write_word(0xBEEF, first_address);
        test_bus.write_word(first_address, second_address);
        test_bus.write_byte(second_address + 0x10, 0xA5);

        let operand = cpu.fetch_operand(AddressingMode::IndirectIndexedY, &mut test_bus);
        assert_eq!(operand, Some(Operand::Address(second_address + 0x10)));
    }

    #[test]
    fn relative_addressing_returns_correct_operand() {
        let mut cpu = CPU::new();
        let mut test_bus = TestBus::new();

        cpu.registers.program_counter = 0x1001;
        test_bus.write_byte(0x1001, 0x03);
        let operand = cpu.fetch_operand(AddressingMode::Relative, &mut test_bus);
        assert_eq!(operand, Some(Operand::Address(0x1005)));
    }

    #[test]
    fn relative_addressing_returns_correct_operand_with_negative_offset() {
        let mut cpu = CPU::new();
        let mut test_bus = TestBus::new();

        cpu.registers.program_counter = 0x1001;
        test_bus.write_byte(0x1001, 0xFB); // -5
        let operand = cpu.fetch_operand(AddressingMode::Relative, &mut test_bus);
        assert_eq!(operand, Some(Operand::Address(0x0FFD)));
    }
    // endregion
}
