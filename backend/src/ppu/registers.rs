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

pub struct Registers {
    pub ppu_ctrl: u8,
    pub ppu_mask: u8,
    pub ppu_status: u8,
    pub oam_addr: u8,
    pub oam_data: u8,
    pub ppu_scroll: u8,
    pub ppu_addr: u8,
    pub ppu_data: u8,
    pub oam_dma: u8,
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
        }
    }
    
    pub fn reset(&mut self, reset_address: u16) {
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

    pub fn set_ppuctrl_flag(&mut self, flag: PpuCtrlFlags, value: bool) {
        if value {
            self.ppu_ctrl |= 1 << flag as u8;
        } else {
            self.ppu_ctrl &= !(1 << flag as u8);
        }
    }

    pub fn set_ppumask_flag(&mut self, flag: PpuMaskFlags, value: bool) {
        if value {
            self.ppu_mask |= 1 << flag as u8;
        } else {
            self.ppu_mask &= !(1 << flag as u8);
        }
    }

    pub fn set_ppustatus_flag(&mut self, flag: PpuStatusFlags, value: bool) {
        if value {
            self.ppu_status |= 1 << flag as u8;
        } else {
            self.ppu_status &= !(1 << flag as u8);
        }
    }

    pub fn reset_ppu_status(&mut self) {
        self.ppu_status &= 0b0001_0000;
    }
}