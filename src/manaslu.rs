use std::path::Path;
use crate::bus::{CpuBus, MemoryIndexer};
use crate::cartridge::Cartridge;
use crate::cpu::CPU;

pub struct Manaslu {
    cpu: CPU<CpuBus>,
    cpu_bus: CpuBus,
}
impl Manaslu {
    pub fn new() -> Result<Self, std::io::Error> {
        let cartridge = Cartridge::from_file(Path::new("/cartridge"))?;
        Ok(Self {
            cpu: CPU::new(),
            cpu_bus: CpuBus::new(cartridge),
        })
    }
}