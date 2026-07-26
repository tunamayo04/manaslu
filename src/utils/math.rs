pub fn is_negative(value: u8) -> bool {
    value & 0b1000_0000 != 0
}