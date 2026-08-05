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
            cartridge,
            ppu: Ppu::new(),
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
                    0x2000 => self.ppu.registers.get_register(PPURegisters::PpuCtrl),
                    0x2001 => self.ppu.registers.get_register(PPURegisters::PpuMask),
                    0x2002 => self.ppu.registers.get_register(PPURegisters::PpuStatus),
                    0x2003 => self.ppu.registers.get_register(PPURegisters::OamAddr),
                    0x2004 => self.ppu.registers.get_register(PPURegisters::OamData),
                    0x2005 => self.ppu.registers.get_register(PPURegisters::PpuScroll),
                    0x2006 => self.ppu.registers.get_register(PPURegisters::PpuAddr),
                    0x2007 => self.ppu.registers.get_register(PPURegisters::PpuData),
                    _ => unreachable!(),
                }
            }

            // APU + I/O registers
            0x4000..=0x4017 => match address {
                0x4014 => self.ppu.registers.get_register(PPURegisters::OamDma),
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
                    0x2000 => self.ppu.registers.set_register(PPURegisters::PpuCtrl, value),
                    0x2001 => self.ppu.registers.set_register(PPURegisters::PpuMask, value),
                    0x2002 => self.ppu.registers.set_register(PPURegisters::PpuStatus, value),
                    0x2003 => self.ppu.registers.set_register(PPURegisters::OamAddr, value),
                    0x2004 => self.ppu.registers.set_register(PPURegisters::OamData, value),
                    0x2005 => self.ppu.registers.set_register(PPURegisters::PpuScroll, value),
                    0x2006 => self.ppu.registers.set_register(PPURegisters::PpuAddr, value),
                    0x2007 => self.ppu.registers.set_register(PPURegisters::PpuData, value),
                    _ => unreachable!(),
                }
            }

            // APU + I/O
            0x4000..=0x4017 => match address {
                0x4014 => self.ppu.registers.set_register(PPURegisters::OamDma, value),
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
