use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use std::path::{Path, PathBuf};
use crate::cpu::cpu_bus::CpuBus;
use crate::cpu::instruction_set::{DebugInstruction, Instruction};

pub struct Manaslu {
    cpu: CPU<CpuBus>,
    cpu_bus: CpuBus,
}
impl Manaslu {
    pub fn new(path: PathBuf) -> Result<Self, std::io::Error> {
        let cartridge = Cartridge::from_file(path)?;
        Ok(Self {
            cpu: CPU::new(),
            cpu_bus: CpuBus::new(cartridge),
        })
    }

    pub fn run(&mut self) {
        self.cpu.reset(&self.cpu_bus);
        loop {
            let cycles = self.cpu.step(&mut self.cpu_bus).expect("oopsie daisy");

            for _ in 0..cycles {
                self.cpu_bus.ppu().step();
                self.cpu_bus.ppu().step();
                self.cpu_bus.ppu().step();
            }
        }
    }

    pub fn run_from_address(&mut self, address: u16) {
        self.cpu.reset_at_address(address);
        loop {
            let cycles = self.cpu.step(&mut self.cpu_bus).expect("oopsie daisy");

            for _ in 0..cycles {
                self.cpu_bus.ppu().step();
                self.cpu_bus.ppu().step();
                self.cpu_bus.ppu().step();
            }
        }
    }

    pub fn cpu(&self) -> &CPU<CpuBus> {
        &self.cpu
    }

    pub fn cpu_bus(&self) -> &CpuBus {
        &self.cpu_bus
    }
}
