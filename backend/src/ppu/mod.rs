use crate::bus::SerialInterface;
use crate::cartridge::Cartridge;
use crate::ppu::registers::Registers;
pub use crate::utils::{PPU_HEIGHT, PPU_WIDTH};

pub mod registers;
pub mod ppu_bus;

pub struct Ppu {
    pub(crate) registers: Registers,
    pub(crate) bus: ppu_bus::PpuBus,

    current_scanline: u16,
    current_cycle: u16,
    is_odd_frame: bool,
    pixel_buffer: [u8; PPU_WIDTH * PPU_HEIGHT * 4],
    current_frame: u64,
}
impl Ppu {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            registers: Registers::new(),
            bus: ppu_bus::PpuBus::new(cartridge),

            current_scanline: 261,
            current_cycle: 0,
            is_odd_frame: false,
            pixel_buffer: [255; PPU_WIDTH * PPU_HEIGHT * 4],
            current_frame: 0,
        }
    }

    pub fn reset(&mut self) {
        self.registers = Registers::new();
    }

    pub fn step(&mut self) {
        match self.current_scanline {
            261 => { // Pre-render scanline
                //self.registers.is_rendering = true;
                match self.current_cycle {
                    0 => {
                        // Idle
                        self.current_cycle += 1;
                        self.current_frame += 1;
                    }
                    1 => {
                        self.registers.reset_ppu_status();
                        self.current_cycle += 1;
                    }
                    2..=339 => {
                        self.current_cycle += 1;
                    }
                    340 => {
                        self.current_cycle = 0;
                        self.current_scanline = 0;
                    }
                    _ => unreachable!(),
                }
            }
            0..=239 => { // Visible scanlines
                match self.current_cycle {
                    0 => {
                        // Idle
                        self.current_cycle += 1;
                    }

                    1..=256 => {
                        let x = self.current_cycle as u8;
                        let y = self.current_scanline as u8;
                        let t = self.current_frame as u8;

                        let index = ((y as usize * PPU_WIDTH) + x as usize) * 4;
                        let u = x.wrapping_sub(128);
                        let v = y.wrapping_sub(120);

                        let pattern = (u ^ v).wrapping_mul(4);
                        let depth = (u.wrapping_mul(u).wrapping_add(v.wrapping_mul(v))) >> 5;

                        let intensity = pattern.wrapping_sub(depth).wrapping_add(t.wrapping_mul(4));

                        let r = intensity;
                        let g = intensity.wrapping_mul(2);
                        let b = intensity.wrapping_add(128);

                        self.pixel_buffer[index..index + 4].copy_from_slice(&[r, g, b, 255]);


                        if self.current_cycle % 8 == 0 {
                            // Reload background shift registers
                            let nametable_tile = self.bus.read_byte(0x2000 | self.registers.v.get() & 0xFFF);
                        }


                        self.current_cycle += 1;
                    }
                    257..=320 => {
                        self.current_cycle += 1;
                    }
                    321..=336 => {
                        self.current_cycle += 1;
                    }
                    337..=339 => {
                        self.current_cycle += 1;
                    }
                    340 => {
                        self.current_cycle = 0;
                        self.current_scanline += 1;
                    }
                    _ => unreachable!(),
                }
            }
            240 => { // Post-render scanline
                //self.registers.is_rendering = false;

                match self.current_cycle {
                    340 => {
                        self.current_cycle = 0;
                        self.current_scanline += 1;
                    }
                    _ => self.current_cycle += 1,
                }
            }
            241 => {
                match self.current_cycle {
                    1 => {
                        self.registers.set_status_flag(registers::PpuStatusFlags::VBlank, true);
                        self.current_cycle += 1;
                    },
                    340 => {
                        self.current_cycle = 0;
                        self.current_scanline += 1;
                    }
                    _ => self.current_cycle += 1,
                }
            }
            241..=260 => { // VBlank scanlines
                if self.current_cycle == 340 {
                    self.current_cycle = 0;
                    self.current_scanline += 1;
                } else {
                    self.current_cycle += 1;
                }
            }
            _ => unreachable!(),
        }
    }
}

impl Ppu {


    pub fn current_scanline(&self) -> u16 {
        self.current_scanline
    }

    pub fn current_cycle(&self) -> u16 {
        self.current_cycle
    }
}

impl Ppu {
    pub fn registers(&self) -> &Registers {
        &self.registers
    }

    pub fn pixel_buffer(&self) -> &[u8] {
        &self.pixel_buffer
    }

    pub fn bus(&self) -> &ppu_bus::PpuBus {
        &self.bus
    }
}