use crate::bus::MemoryIndexer;
use crate::cartridge::Cartridge;
use crate::ppu::Ppu;

pub struct CpuBus {
    ram: [u8; 0x800],
    cartridge: Cartridge,
    ppu: Ppu
}

impl CpuBus {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            ram: [0; 0x800],
            cartridge,
            ppu: Ppu::new(),
        }
    }
}

impl CpuBus {
    pub fn ppu(&mut self) -> &mut Ppu {
        &mut self.ppu
    }
}

impl MemoryIndexer for CpuBus {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            // 2 KB internal RAM mirrored every 0x800 bytes
            0x0000..=0x1FFF => self.ram[(address & 0x07FF) as usize],

            // PPU registers mirrored every 8 bytes
            0x2000..=0x3FFF => {
                let register = 0x2000 | (address & 0x0007);

                match register {
                    0x2000 => self.ppu.registers.ppu_ctrl,
                    0x2001 => self.ppu.registers.ppu_mask,
                    0x2002 => self.ppu.registers.ppu_status,
                    0x2003 => self.ppu.registers.oam_addr,
                    0x2004 => self.ppu.registers.oam_data,
                    0x2005 => self.ppu.registers.ppu_scroll,
                    0x2006 => self.ppu.registers.ppu_addr,
                    0x2007 => self.ppu.registers.ppu_data,
                    _ => unreachable!(),
                }
            }

            // APU + I/O registers
            0x4000..=0x4017 => match address {
                0x4014 => self.ppu.registers.oam_dma,
                0x4016 => todo!("Controller 1"),
                0x4017 => todo!("Controller 2 / APU"),
                _ => todo!("APU register"),
            },

            // Normally disabled/test mode
            0x4018..=0x401F => 0,

            // Cartridge space
            0x4020..=0xFFFF => self.cartridge.read_byte(address),
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
                    0x2000 => self.ppu.registers.ppu_ctrl = value,
                    0x2001 => self.ppu.registers.ppu_mask = value,
                    0x2002 => self.ppu.registers.ppu_status = value,
                    0x2003 => self.ppu.registers.oam_addr = value,
                    0x2004 => self.ppu.registers.oam_data = value,
                    0x2005 => self.ppu.registers.ppu_scroll = value,
                    0x2006 => self.ppu.registers.ppu_addr = value,
                    0x2007 => self.ppu.registers.ppu_data = value,
                    _ => unreachable!(),
                }
            }

            // APU + I/O
            0x4000..=0x4017 => match address {
                0x4014 => self.ppu.registers.oam_dma = value,
                0x4016 => todo!("Controller strobe"),
                0x4017 => todo!("APU frame counter"),
                _ => todo!("APU register"),
            },

            0x4018..=0x401F => {}

            // Cartridge (PRG RAM, mapper registers, etc.)
            0x4020..=0xFFFF => {
                self.cartridge.write_byte(address, value);
            }
        }
    }
}
