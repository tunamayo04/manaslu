use crate::bus::CpuBus;
use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use std::path::Path;

pub struct Manaslu {
    cpu: CPU<CpuBus>,
    cpu_bus: CpuBus,
}
impl Manaslu {
    pub fn new(path: &str) -> Result<Self, std::io::Error> {
        let cartridge = Cartridge::from_file(Path::new(path))?;
        Ok(Self {
            cpu: CPU::new(),
            cpu_bus: CpuBus::new(cartridge),
        })
    }

    pub fn run(&mut self) {
        self.cpu.reset(&self.cpu_bus);
        loop {
            self.cpu.step(&mut self.cpu_bus).expect("oopsie daisy");
        }
    }

    pub fn run_from_address(&mut self, address: u16) {
        self.cpu.reset_at_address(address);
        loop {
            self.cpu.step(&mut self.cpu_bus).expect("oopsie daisy")
        }
    }
}
