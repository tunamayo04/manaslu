use crate::bus::SerialInterface;
use crate::cartridge::Cartridge;
use crate::ppu::Ppu;
use crate::ppu::registers::PPURegisters;

pub struct CpuBus {
    ram: [u8; 0x800],
    cartridge: Cartridge,
    ppu: Ppu
}

impl CpuBus {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            ram: [0; 0x800],
            cartridge: cartridge.clone(), // TODO: Should not clone here
            ppu: Ppu::new(cartridge.clone()),
        }
    }
}

impl CpuBus {
    pub fn ppu_mut(&mut self) -> &mut Ppu {
        &mut self.ppu
    }

    pub fn ppu(&self) -> &Ppu {
        &self.ppu
    }
}

impl SerialInterface for CpuBus {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            // 2 KB internal RAM mirrored every 0x800 bytes
            0x0000..=0x1FFF => self.ram[(address & 0x07FF) as usize],

            // PPU registers mirrored every 8 bytes
            0x2000..=0x3FFF => {
                let register = 0x2000 | (address & 0x0007);

                match register {
                    0x2000 => self.ppu.registers.get_register(PPURegisters::PpuCtrl, &self.ppu.bus),
                    0x2001 => self.ppu.registers.get_register(PPURegisters::PpuMask, &self.ppu.bus),
                    0x2002 => self.ppu.registers.get_register(PPURegisters::PpuStatus, &self.ppu.bus),
                    0x2003 => self.ppu.registers.get_register(PPURegisters::OamAddr, &self.ppu.bus),
                    0x2004 => self.ppu.registers.get_register(PPURegisters::OamData, &self.ppu.bus),
                    0x2005 => self.ppu.registers.get_register(PPURegisters::PpuScroll, &self.ppu.bus),
                    0x2006 => self.ppu.registers.get_register(PPURegisters::PpuAddr, &self.ppu.bus),
                    0x2007 => self.ppu.registers.get_register(PPURegisters::PpuData, &self.ppu.bus),
                    _ => unreachable!(),
                }
            }

            // APU + I/O registers
            0x4000..=0x4017 => match address {
                0x4014 => self.ppu.registers.get_register(PPURegisters::OamDma, &self.ppu.bus),
                0x4016 => 0, // Controller 1,
                0x4017 => 0, // "Controller 2 / APU,
                _ => 0 // APU register,
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
                    0x2000 => self.ppu.registers.set_register(PPURegisters::PpuCtrl, value, &mut self.ppu.bus),
                    0x2001 => self.ppu.registers.set_register(PPURegisters::PpuMask, value, &mut self.ppu.bus),
                    0x2002 => self.ppu.registers.set_register(PPURegisters::PpuStatus, value, &mut self.ppu.bus),
                    0x2003 => self.ppu.registers.set_register(PPURegisters::OamAddr, value, &mut self.ppu.bus),
                    0x2004 => self.ppu.registers.set_register(PPURegisters::OamData, value, &mut self.ppu.bus),
                    0x2005 => self.ppu.registers.set_register(PPURegisters::PpuScroll, value, &mut self.ppu.bus),
                    0x2006 => self.ppu.registers.set_register(PPURegisters::PpuAddr, value, &mut self.ppu.bus),
                    0x2007 => self.ppu.registers.set_register(PPURegisters::PpuData, value, &mut self.ppu.bus),
                    _ => unreachable!(),
                }
            }

            // APU + I/O
            0x4000..=0x4017 => match address {
                0x4014 => self.ppu.registers.set_register(PPURegisters::OamDma, value, &mut self.ppu.bus),
                0x4016 => (), // Controller strobe,
                0x4017 => (), // APU frame counter,
                _ => () // APU register,
            },

            0x4018..=0x401F => {}

            // Cartridge (PRG RAM, mapper registers, etc.)
            0x4020..=0xFFFF => {
                self.cartridge.write_byte(address, value);
            }
        }
    }
}
