use std::cell::Cell;

pub enum PpuCtrlFlags {
    BaseNameTableAddress = 0b0000_0011,
    VRamAddressIncrement = 0b0000_0100,
    SpritePatternAddress = 0b0000_1000,
    BackgroundPatternAddress = 0b0001_0000,
    SpriteSize = 0b0010_0000,
    MasterSlaveSelect = 0b0100_0000,
    VBlankNmi = 0b1000_0000,
}

pub enum PpuMaskFlags {
    Greyscale = 0b0000_0001,
    ShowBackground = 0b0000_0010,
    ShowSprites = 0b0000_0100,
    EnableBackgroundRendering = 0b0000_1000,
    EnableSpriteRendering = 0b0001_0000,
    EmphasizeRed = 0b0010_0000,
    EmphasizeGreen = 0b0100_0000,
    EmphasizeBlue = 0b1000_0000,
}

pub enum PpuStatusFlags {
    SpriteOverflow = 0b0010_0000,
    SpriteZeroHit = 0b0100_0000,
    VBlank = 0b1000_0000,
}

pub enum VramAddressFlags {
    CoarseXScroll = 0b000_00_00000_11111,
    CoarseYScroll = 0b000_00_11111_00000,
    NametableSelect = 0b000_11_00000_00000,
    FineYScroll = 0b111_00_00000_00000,
}

pub enum PPURegisters {
    PpuCtrl,
    PpuMask,
    PpuStatus,
    OamAddr,
    OamData,
    PpuScroll,
    PpuAddr,
    PpuData,
    OamDma,
}
pub struct Registers {
    ppu_ctrl: u8,
    ppu_mask: u8,
    ppu_status: u8,
    oam_addr: u8,
    oam_data: u8,
    ppu_scroll: u8,
    ppu_addr: u8,
    ppu_data: u8,
    oam_dma: u8,
    pub(crate) v: Cell<u16>, // Current VRAM address
    t: u16, // Temporary VRAM address
    x: u8, // Fine X scroll
    w: Cell<bool>, // First or second write toggle (0 = first, 1 = second)
    is_rendering: bool, // True during pre-render line and through visible lines 0-239

}

impl Registers {
    pub fn new() -> Self {
        Self {
            ppu_ctrl: 0,
            ppu_mask: 0,
            ppu_status: 0,
            oam_addr: 0,
            oam_data: 0,
            ppu_scroll: 0,
            ppu_addr: 0,
            ppu_data: 0,
            oam_dma: 0,
            v: Cell::from(0),
            t: 0,
            x: 0,
            w: Cell::from(false),
            is_rendering: false,
        }
    }
    
    pub fn reset(&mut self) {
        self.ppu_ctrl = 0;
        self.ppu_mask = 0;
        self.ppu_status = 0b1000_0000;
        self.oam_addr = 0;
        self.oam_data = 0;
        self.ppu_scroll = 0;
        self.ppu_addr = 0;
        self.ppu_data = 0;
        self.oam_dma = 0;
    }

    pub(crate) fn set_status_flag(&mut self, flag: PpuStatusFlags, value: bool) {
        let mask = flag as u8;
        self.ppu_status = (self.ppu_status & !mask) | (if value { mask } else { 0 });
    }

    fn set_v_flag(&mut self, flag: VramAddressFlags, value: u16) {
        let mask = flag as u16;
        let shift = mask.trailing_zeros() as u8;

        self.v.set((self.v.get() & !mask) | value << shift);
    }

    fn set_t_flag(&mut self, flag: VramAddressFlags, value: u16) {
        let mask = flag as u16;
        let shift = mask.trailing_zeros() as u8;

        self.t = (self.t & !mask) | value << shift
    }

    pub(crate) fn reset_ppu_status(&mut self) {
        self.ppu_status &= 0b0001_1111;
    }

    pub fn set_register(&mut self, register: PPURegisters, value: u8) {
        match register {
            PPURegisters::PpuCtrl => {
                self.ppu_ctrl = value;

                // t: ...GH.. ........ <- d: ......GH
                let mask = PpuCtrlFlags::BaseNameTableAddress as u16;
                let nametable_select = (value as u16) & mask;
                self.set_t_flag(VramAddressFlags::NametableSelect, nametable_select);
            },
            PPURegisters::PpuMask => self.ppu_mask = value,
            PPURegisters::PpuStatus => self.ppu_status = value,
            PPURegisters::OamAddr => self.oam_addr = value,
            PPURegisters::OamData => self.oam_data = value,
            PPURegisters::PpuScroll => {
                self.ppu_scroll = value;

                if !self.w.get() {
                    // t: ....... ...ABCDE <- d: ABCDE...
                    self.set_t_flag(
                        VramAddressFlags::CoarseXScroll,
                        ((value as u16) >> 3) & 0x1F,
                    );

                    // x:              FGH <- d: .....FGH
                    self.x = value & 0x07;

                    // w <- 1
                    self.w.set(true);
                } else {
                    // t: .... .ABCDE ..... <- d: ABCDE...
                    self.set_t_flag(
                        VramAddressFlags::CoarseYScroll,
                        ((value as u16) >> 3) & 0x1F,
                    );

                    // t: FGH.. ........... <- d: .....FGH
                    self.set_t_flag(
                        VramAddressFlags::FineYScroll,
                        (value as u16) & 0x07,
                    );

                    // w <- 0
                    self.w.set(false);
                }
            }
            PPURegisters::PpuAddr => {
                self.ppu_addr = value;

                if !self.w.get() {
                    // t: .HIJKLM ........ <- d: ..HIJKLM
                    self.t = (self.t & 0x00FF) | (((value as u16 & 0x3F) << 8));
                    self.t &= 0x3FFF;

                    self.w.set(true);
                } else {
                    // t: ........ LMNOPQRS <- d: LMNOPQRS
                    self.t = (self.t & 0xFF00) | value as u16;
                    self.t &= 0x3FFF;

                    // v = t
                    self.v.set(self.t);

                    self.w.set(false);
                }
            }
            PPURegisters::PpuData => {
                self.ppu_data = value;

                if self.is_rendering {
                    self.coarse_x_increment();
                    self.coarse_y_increment();
                } else {
                    let vram_increment = self.ppu_ctrl & PpuCtrlFlags::VRamAddressIncrement as u8 != 0;
                    self.v.set(self.v.get() + if vram_increment { 32 } else { 1 });
                }
            }
            PPURegisters::OamDma => self.oam_dma = value,
        }
    }

    pub fn get_register(&self, register: PPURegisters) -> u8 {
        match register {
            PPURegisters::PpuCtrl => self.ppu_ctrl,
            PPURegisters::PpuMask => self.ppu_mask,
            PPURegisters::PpuStatus => {
                self.w.set(false);
                self.ppu_status
            },
            PPURegisters::OamAddr => self.oam_addr,
            PPURegisters::OamData => self.oam_data,
            PPURegisters::PpuScroll => self.ppu_scroll,
            PPURegisters::PpuAddr => self.ppu_addr,
            PPURegisters::PpuData => {
                if self.is_rendering {
                    self.coarse_x_increment();
                    self.coarse_y_increment();
                } else {
                    let vram_increment = self.ppu_ctrl & PpuCtrlFlags::VRamAddressIncrement as u8 != 0;
                    self.v.set(self.v.get() + if vram_increment { 32 } else { 1 });
                }

                self.ppu_data
            },
            PPURegisters::OamDma => self.oam_dma,
        }
    }

    fn coarse_x_increment(&self) {
        if (self.v.get() & 0x001F) == 31 { // if coarse X == 31
            self.v.set(self.v.get() & !0x001F); // coarse X = 0
            self.v.set(self.v.get() ^ 0x0400); // switch horizontal nametable
        } else {
            self.v.set(self.v.get() + 1); // increment coarse X
        }
    }

    fn coarse_y_increment(&self) {
        if (self.v.get() & 0x7000) != 0x7000 { // if fine Y < 7
            self.v.set(self.v.get() + 0x1000); // increment fine Y
        } else {
            self.v.set(self.v.get() & !0x7000); // fine Y = 0
            let mut y = (self.v.get() & 0x03E0) >> 5; // let y = coarse Y

            if y == 29 {
                y = 0;
                self.v.set(self.v.get() ^ 0x0800); // switch vertical nametable
            } else if y == 31 {
                y = 0; // coarse Y = 0, nametable not switched
            } else {
                y += 1;
            }

            self.v.set((self.v.get() & !0x03E0) | (y << 5));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_ppuctrl_sets_nametable_select_bits() {
        // Arrange
        let mut registers = Registers::new();

        // Act & Assert
        registers.set_register(PPURegisters::PpuCtrl, 0b0000_0000);
        assert_eq!(registers.t, 0);

        registers.set_register(PPURegisters::PpuCtrl, 0b0000_0001);
        assert_eq!(registers.t, 0b0000100_00000000);

        registers.set_register(PPURegisters::PpuCtrl, 0b0000_0010);
        assert_eq!(registers.t, 0b0001000_00000000);

        registers.set_register(PPURegisters::PpuCtrl, 0b0000_0011);
        assert_eq!(registers.t, 0b0001100_00000000);
    }

    #[test]
    fn read_ppustatus_resets_w_flag() {
        // Arrange
        let mut registers = Registers::new();
        registers.w.set(true);

        // Act
        registers.get_register(PPURegisters::PpuStatus);

        // Assert
        assert_eq!(registers.w.get(), false);
    }

    #[test]
    fn write_ppuscroll_first_write_sets_t_x_w_flags() {
        // Arrange
        let mut registers = Registers::new();
        registers.w.set(false);

        // Act
        registers.set_register(PPURegisters::PpuScroll, 0b11101_111);

        // Assert
        assert_eq!(registers.t, 0b000_00_00000_11101);
        assert_eq!(registers.x, 0b111);
        assert_eq!(registers.w.get(), true);
    }

    #[test]
    fn write_ppuscroll_second_write_sets_t_w_flags() {
        // Arrange
        let mut registers = Registers::new();
        registers.w.set(true);

        // Act
        registers.set_register(PPURegisters::PpuScroll, 0b11111111);

        // Assert
        assert_eq!(registers.t, 0b111_00_11111_00000);
        assert_eq!(registers.w.get(), false);
    }

    #[test]
    fn write_ppuaddr_first_write_sets_t_w_flags() {
        // Arrange
        let mut registers = Registers::new();
        registers.w.set(false);
        registers.t = 0b100_00_00000_00000;

        // Act
        registers.set_register(PPURegisters::PpuAddr, 0b11110011);

        // Assert
        assert_eq!(registers.t, 0b0110011_00000000);
        assert_eq!(registers.w.get(), true);
    }

    #[test]
    fn write_ppu_addr_second_write_sets_t_w_v_flags() {
        // Arrange
        let mut registers = Registers::new();
        registers.w.set(true);

        // Act
        registers.set_register(PPURegisters::PpuAddr, 0b11111001);

        // Assert
        assert_eq!(registers.t, 0b11111001);
        assert_eq!(registers.w.get(), false);
        assert_eq!(registers.v.get(), registers.t);
    }

    #[test]
    fn write_ppudata_increments_v_by_one_when_increment_disabled() {
        // Arrange
        let mut registers = Registers::new();
        registers.v.set(0x2000);

        // Act
        registers.set_register(PPURegisters::PpuData, 0xFF);

        // Assert
        assert_eq!(registers.v.get(), 0x2001);
    }

    #[test]
    fn write_ppudata_increments_v_by_32_when_increment_enabled() {
        // Arrange
        let mut registers = Registers::new();
        registers.v.set(0x2000);

        // Enable VRAM increment by 32
        registers.set_register(PPURegisters::PpuCtrl, 0b0000_0100);

        // Act
        registers.set_register(PPURegisters::PpuData, 0xFF);

        // Assert
        assert_eq!(registers.v.get(), 0x2020);
    }

    #[test]
    fn read_ppudata_increments_v_by_one_when_increment_disabled() {
        // Arrange
        let mut registers = Registers::new();
        registers.v.set(0x2000);

        // Act
        registers.get_register(PPURegisters::PpuData);

        // Assert
        assert_eq!(registers.v.get(), 0x2001);
    }

    #[test]
    fn read_ppudata_increments_v_by_32_when_increment_enabled() {
        // Arrange
        let mut registers = Registers::new();
        registers.v.set(0x2000);

        // Enable VRAM increment by 32
        registers.set_register(PPURegisters::PpuCtrl, 0b0000_0100);

        // Act
        registers.get_register(PPURegisters::PpuData);

        // Assert
        assert_eq!(registers.v.get(), 0x2020);
    }

    #[test]
    fn write_ppudata_during_rendering_increments_x_and_y() {
        // Arrange
        let mut registers = Registers::new();

        // coarse X = 31, coarse Y = 0, fine Y = 0
        registers.v.set(0b000_00_00000_11111);

        registers.is_rendering = true;

        // Act
        registers.set_register(PPURegisters::PpuData, 0xFF);

        // Assert
        // coarse X wrapped:
        // 31 -> 0
        // horizontal nametable flipped:
        // 0 -> 1
        // coarse Y incremented:
        // 0 -> 1
        assert_eq!(registers.v.get(), 0b100_00_00101_00001);    }

    #[test]
    fn write_ppudata_during_rendering_increments_fine_y() {
        // Arrange
        let mut registers = Registers::new();

        // fine Y = 3, coarse Y = 5, coarse X = 0
        registers.v.set(0b011_00_00101_00000);

        registers.is_rendering = true;

        // Act
        registers.set_register(PPURegisters::PpuData, 0xFF);

        // Assert
        // fine Y: 3 -> 4
        assert_eq!(registers.v.get(), 0b100_00_00101_00001);    }

    #[test]
    fn set_v_flags() {
        let mut registers = Registers::new();
        
        registers.set_v_flag(VramAddressFlags::CoarseXScroll, 0b01110);
        assert_eq!(registers.v.get(), 0b000_00_00000_01110);
        
        registers.set_v_flag(VramAddressFlags::FineYScroll, 0b111);
        assert_eq!(registers.v.get(), 0b111_00_00000_01110);
        
        registers.set_v_flag(VramAddressFlags::NametableSelect, 0b11);
        assert_eq!(registers.v.get(), 0b111_11_00000_01110);
    }

    #[test]
    fn set_t_flags() {
        let mut registers = Registers::new();

        registers.set_t_flag(VramAddressFlags::CoarseXScroll, 0b01110);
        assert_eq!(registers.t, 0b000_00_00000_01110);

        registers.set_t_flag(VramAddressFlags::FineYScroll, 0b111);
        assert_eq!(registers.t, 0b111_00_00000_01110);

        registers.set_t_flag(VramAddressFlags::NametableSelect, 0b11);
        assert_eq!(registers.t, 0b111_11_00000_01110);

    }

    #[test]
    fn write_ppuaddr_two_write_mechanism() {
        let mut registers = Registers::new();
        
        // First write: high byte
        registers.set_register(PPURegisters::PpuAddr, 0x21);
        assert_eq!(registers.w.get(), true);
        assert_eq!(registers.t, 0x2100);
        assert_eq!(registers.v.get(), 0); // v only updated on second write
        
        // Second write: low byte
        registers.set_register(PPURegisters::PpuAddr, 0x08);
        assert_eq!(registers.w.get(), false);
        assert_eq!(registers.t, 0x2108);
        assert_eq!(registers.v.get(), 0x2108);
    }
}