use crate::cartridge::Cartridge;

pub trait MemoryIndexer {
    fn read_byte(&self, address: u16) -> u8;
    fn write_byte(&mut self, address: u16, value: u8);
    fn read_word(&self, address: u16) -> u16;
    fn write_word(&mut self, address: u16, value: u16);
}

pub struct CpuBus {
    ram: [u8; 0x800],
    cartridge: Cartridge,
}
impl CpuBus {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            ram: [0; 0x800],
            cartridge
        }
    }
}
impl MemoryIndexer for CpuBus {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0..=0x07FF => {
                self.ram[address as usize]
            }
            0x0800..=0x1FFF => {
                self.read_byte(address - 0x0800)
            }
            0x2000..=0x2007 => {
                // PPU Registers
                todo!()
            }
            0x2008..=0x3FFF => {
                todo!()
            }
            0x4000..=0x4017 => {
                todo!()
            }
            0x4018..=0x401F => {
                todo!()
            }
            0x4020..=0xFFFF => {
                todo!()
            }
            _ => { todo!() }
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            _ => todo!()
        }
    }

    fn read_word(&self, address: u16) -> u16 {
        match address {
            0..=0x07FF => {
                let low_nibble = self.ram[address as usize];
                let high_nibble = self.ram[(address + 1) as usize];
                u16::from_le_bytes([low_nibble, high_nibble])
            }
            0x0800..=0x1FFF => {
                self.read_word(address - 0x8000)
            }
            0x2000..=0x2007 => {
                // PPU Registers
                todo!()
            }
            0x2008..=0x3FFF => {
                todo!()
            }
            0x4000..=0x4017 => {
                todo!()
            }
            0x4018..=0x401F => {
                todo!()
            }
            0x4020..=0xFFFF => {
                todo!()
            }
            _ => todo!()
        }

    }

    fn write_word(&mut self, address: u16, value: u16) {
        todo!()
    }
}