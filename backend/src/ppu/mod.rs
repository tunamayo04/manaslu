use crate::ppu::registers::Registers;

pub mod registers;
pub mod ppu_bus;

pub struct Ppu {
    pub(crate) registers: Registers,
    current_scanline: u16,
    current_cycle: u16,
    is_odd_frame: bool,
}
impl Ppu {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            current_scanline: 261,
            current_cycle: 0,
            is_odd_frame: false,
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
                        // let nametable_byte = 0x2000 | (self.registers.v.get() & 0x0FFF);

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