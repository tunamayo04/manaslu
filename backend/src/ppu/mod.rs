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
                    0 => { /* Blanking */ }
                    1 => {
                        self.registers.reset_ppu_status();
                    }
                    _ => {},
                }
            }
            0..=239 => { // Visible scanlines
                match self.current_cycle {
                    0 => { /* Idle */ }
                    1..=256 if self.current_cycle % 8 == 0 => {
                        let nametable_byte = 0x2000 | (self.registers.v.get() & 0x0FFF);
                    }
                    _ => {

                    },
                }
            }
            240 => { // Post-render scanline

            }
            241..=260 => { // VBlank scanlines

            }
            _ => unreachable!(),
        }
    }
}