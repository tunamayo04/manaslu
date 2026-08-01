use log::info;
use crate::cartridge::Cartridge;

pub trait MemoryIndexer {
    fn read_byte(&self, address: u16) -> u8;
    fn write_byte(&mut self, address: u16, value: u8);

    fn read_word(&self, addr: u16) -> u16 {
        let low = self.read_byte(addr) as u16;
        // Only wrap within zero page if addr is in zero page range
        let high_addr = if addr < 0x0100 {
            (addr & 0xFF00) | ((addr + 1) & 0xFF)
        } else {
            addr + 1
        };
        let high = self.read_byte(high_addr) as u16;
        (high << 8) | low
    }

    fn write_word(&mut self, address: u16, value: u16) {
        let [lo, hi] = value.to_le_bytes();
        self.write_byte(address, lo);

        let high_addr = if address < 0x0100 {
            (address & 0xFF00) | ((address + 1) & 0xFF)
        } else {
            address.wrapping_add(1)
        };
        self.write_byte(high_addr, hi);
    }
}

pub struct CpuBus {
    ram: [u8; 0x800],
    cartridge: Cartridge,
}

impl CpuBus {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            ram: [0; 0x800],
            cartridge,
        }
    }
}

impl MemoryIndexer for CpuBus {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            // 2 KB internal RAM mirrored every 0x800 bytes
            0x0000..=0x1FFF => {
                self.ram[(address & 0x07FF) as usize]
            }

            // PPU registers
            // 8-byte region mirrored every 8 bytes
            0x2000..=0x3FFF => {
                let register = 0x2000 | (address & 0x0007);

                match register {
                    0x2000 => todo!("PPUCTRL"),
                    0x2001 => todo!("PPUMASK"),
                    0x2002 => todo!("PPUSTATUS"),
                    0x2003 => todo!("OAMADDR"),
                    0x2004 => todo!("OAMDATA"),
                    0x2005 => todo!("PPUSCROLL"),
                    0x2006 => todo!("PPUADDR"),
                    0x2007 => todo!("PPUDATA"),
                    _ => unreachable!(),
                }
            }

            // APU + I/O registers
            0x4000..=0x4017 => {
                match address {
                    0x4014 => todo!("OAM DMA"),
                    0x4016 => todo!("Controller 1"),
                    0x4017 => todo!("Controller 2 / APU"),
                    _ => todo!("APU register"),
                }
            }

            // Normally disabled/test mode
            0x4018..=0x401F => {
                0
            }

            // Cartridge space
            0x4020..=0xFFFF => {
                self.cartridge.read_byte(address)
            }
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            // Internal RAM
            0x0000..=0x1FFF => {
                self.ram[(address & 0x07FF) as usize] = value;
            }

            // PPU registers
            0x2000..=0x3FFF => {
                let register = 0x2000 | (address & 0x0007);

                match register {
                    0x2000 => todo!("PPUCTRL"),
                    0x2001 => todo!("PPUMASK"),
                    0x2002 => todo!("PPUSTATUS"),
                    0x2003 => todo!("OAMADDR"),
                    0x2004 => todo!("OAMDATA"),
                    0x2005 => todo!("PPUSCROLL"),
                    0x2006 => todo!("PPUADDR"),
                    0x2007 => todo!("PPUDATA"),
                    _ => unreachable!(),
                }
            }

            // APU + I/O
            0x4000..=0x4017 => {
                match address {
                    0x4014 => todo!("OAM DMA"),
                    0x4016 => todo!("Controller strobe"),
                    0x4017 => todo!("APU frame counter"),
                    _ => todo!("APU register"),
                }
            }

            0x4018..=0x401F => {}

            // Cartridge (PRG RAM, mapper registers, etc.)
            0x4020..=0xFFFF => {
                self.cartridge.write_byte(address, value);
            }
        }
    }
}