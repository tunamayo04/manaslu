use crate::bus::MemoryIndexer;

pub struct TestBus {
    memory: Vec<u8>,
}
impl Default for TestBus {
    fn default() -> Self {
        Self::new()
    }
}

impl TestBus {
    pub fn new() -> Self {
        TestBus {
            memory: vec![0; 0xFFFF],
        }
    }
}
impl MemoryIndexer for TestBus {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
    fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    fn read_word(&self, address: u16) -> u16 {
        let lower_byte = self.memory[address as usize];
        let upper_byte = self.memory[address.wrapping_add(1) as usize];
        u16::from_le_bytes([lower_byte, upper_byte])
    }

    fn write_word(&mut self, address: u16, value: u16) {
        let [lower_byte, upper_byte] = value.to_le_bytes();

        self.memory[address as usize] = lower_byte;
        self.memory[address.wrapping_add(1) as usize] = upper_byte;
    }
}
