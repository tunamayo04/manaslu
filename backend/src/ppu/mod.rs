use rand::RngExt;
use crate::ppu::registers::Registers;

pub mod registers;
pub mod ppu_bus;

pub const PPU_WIDTH: usize = 256;
pub const PPU_HEIGHT: usize = 240;

pub struct Ppu {
    pub(crate) registers: Registers,
    current_scanline: u16,
    current_cycle: u16,
    is_odd_frame: bool,
    pixel_buffer: [u8; PPU_WIDTH * PPU_HEIGHT * 4],
}
impl Ppu {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            current_scanline: 261,
            current_cycle: 0,
            is_odd_frame: false,
            pixel_buffer: [255; PPU_WIDTH * PPU_HEIGHT * 4],
        }
    }

    pub fn reset(&mut self) {
        self.registers = Registers::new();
    }

    pub fn step(&mut self) {
        match self.current_scanline {
            261 => { // Pre-render scanline
                match self.current_cycle {
                    0 => {
                        // Idle
                        self.current_cycle += 1;
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
                        let index = ((self.current_scanline as usize * PPU_WIDTH)
                            + (self.current_cycle - 1) as usize) * 4;

                        self.pixel_buffer[index..index + 4].copy_from_slice(&[self.current_scanline as u8, 0, self.current_cycle as u8, 255]);

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
}