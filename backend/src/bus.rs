pub trait MemoryIndexer {
    fn read_byte(&self, address: u16) -> u8;
    fn write_byte(&mut self, address: u16, value: u8);

    fn read_word(&self, addr: u16) -> u16 {
        let low = self.read_byte(addr) as u16;
        // Only wrap within zero page if addr is in zero page range
        let high_addr = if addr < 0x0100 {
            (addr & 0xFF00) | ((addr + 1) & 0xFF)
        } else {
            addr + 1
        };
        let high = self.read_byte(high_addr) as u16;
        (high << 8) | low
    }

    fn write_word(&mut self, address: u16, value: u16) {
        let [lo, hi] = value.to_le_bytes();
        self.write_byte(address, lo);

        let high_addr = if address < 0x0100 {
            (address & 0xFF00) | ((address + 1) & 0xFF)
        } else {
            address.wrapping_add(1)
        };
        self.write_byte(high_addr, hi);
    }
}
