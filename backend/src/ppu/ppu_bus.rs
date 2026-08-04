use crate::bus::MemoryIndexer;
use crate::cartridge::{Cartridge, NametableMirroring};

pub struct PpuBus {
    vram: [u8; 0x0800],
    palette: [u8; 0x0020],
    cartridge: Cartridge,
}

impl PpuBus {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            vram: [0; 0x0800],
            palette: [0; 0x0020],
            cartridge,
        }
    }

    fn mirror_vram_address(&self, address: u16) -> usize {
        let address = (address - 0x2000) & 0x0FFF;

        let table = address / 0x0400;
        let offset = address & 0x03FF;

        match self.cartridge.mirroring() {
            // Vertical mirroring:
            // 2000 = 2800, 2400 = 2C00
            NametableMirroring::Vertical => match table {
                0 | 2 => offset as usize,
                1 | 3 => (0x0400 + offset) as usize,
                _ => unreachable!(),
            },

            // Horizontal mirroring:
            // 2000 = 2400, 2800 = 2C00
            NametableMirroring::Horizontal => match table {
                0 | 1 => offset as usize,
                2 | 3 => (0x0400 + offset) as usize,
                _ => unreachable!(),
            },
        }
    }

    fn mirror_palette_address(address: u16) -> usize {
        let mut address = (address - 0x3F00) & 0x001F;

        // Palette mirrors:
        match address {
            0x10 | 0x14 | 0x18 | 0x1C => {
                address -= 0x10;
            }
            _ => {}
        }

        address as usize
    }
}

impl MemoryIndexer for PpuBus {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            // CHR-ROM / CHR-RAM
            0x0000..=0x1FFF => {
                self.cartridge.read_byte(address)
            }

            // Nametable RAM
            0x2000..=0x3EFF => {
                let index = self.mirror_vram_address(address);
                self.vram[index]
            }

            // Palette RAM
            0x3F00..=0x3FFF => {
                let index = Self::mirror_palette_address(address);
                self.palette[index]
            }

            _ => unreachable!(),
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            // CHR-RAM only
            0x0000..=0x1FFF => {
                // if self.cartridge.has_chr_ram() {
                    self.cartridge.write_byte(address, value);
                // }
            }

            // Nametable RAM
            0x2000..=0x3EFF => {
                let index = self.mirror_vram_address(address);
                self.vram[index] = value;
            }

            // Palette RAM
            0x3F00..=0x3FFF => {
                let index = Self::mirror_palette_address(address);
                self.palette[index] = value;
            }

            _ => unreachable!(),
        }
    }
}
