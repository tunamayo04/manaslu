use crate::bus::SerialInterface;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum NametableMirroring {
    Horizontal,
    Vertical,
}

#[derive(Clone)]
pub struct INesHeader {
    prg_rom_size: u8, // Size of PRG ROM in KB
    chr_rom_size: u8, // Size of CHR ROM in KB (value 0 means the board uses CHR RAM)
    pub nametable_mirroring: NametableMirroring,
    contains_battery_pack: bool,
    has_trainer: bool,
    alternative_nametable_layout: bool,
    mapper_number: u8,
    vs_unisystem: bool,
    playchoice_10: bool,
    is_nes_2_0: bool,
    prg_ram_size: u8,
}
impl INesHeader {
    pub fn from_bytes(bytes: &[u8; 16]) -> Result<Self, std::io::Error> {
        if bytes[0..4] != [0x4E, 0x45, 0x53, 0x1A] {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid NES header",
            ));
        }

        let prg_rom_size = bytes[4];
        let chr_rom_size = bytes[5];
        let flags_6 = bytes[6];
        let flags_7 = bytes[7];
        let prg_ram_size = bytes[8];
        let _flags_9 = bytes[9];
        let _flags_10 = bytes[10];

        Ok(INesHeader {
            prg_rom_size,
            chr_rom_size,
            nametable_mirroring: if flags_6 & 0b0000_0001 != 0 {
                NametableMirroring::Vertical
            } else {
                NametableMirroring::Horizontal
            },
            contains_battery_pack: flags_6 & 0b0000_0010 != 0,
            has_trainer: flags_6 & 0b0000_0100 != 0,
            alternative_nametable_layout: flags_6 & 0b0000_1000 != 0,
            mapper_number: ((flags_6 & 0xF0) >> 4) | (flags_7 & 0xF0),
            vs_unisystem: flags_7 & 0b0000_0001 != 0,
            playchoice_10: flags_7 & 0b0000_0010 != 0,
            is_nes_2_0: flags_7 & 0b0000_1100 == 0b1100,
            prg_ram_size,
        })
    }
}

#[derive(Clone)]
pub struct Cartridge {
    path: PathBuf,
    header: INesHeader,
    trainer: Option<[u8; 512]>,
    prg_rom: Vec<u8>,
    chr_rom: Option<Vec<u8>>,
    playchoice_inst_rom: Option<[u8; 8192]>,
    playchoice_prom: Option<[u8; 32]>,
}
impl Cartridge {
    pub fn from_file(path: PathBuf) -> Result<Self, std::io::Error> {
        let mut rom = std::fs::File::open(&path)?;

        // iNes Header
        let mut header_buffer = [0; 16];
        rom.read_exact(&mut header_buffer)?;
        let header = INesHeader::from_bytes(&header_buffer)?;

        // Trainer
        let trainer = if header.has_trainer {
            let mut trainer_buffer = [0; 512];
            rom.read_exact(&mut trainer_buffer)?;
            Some(trainer_buffer)
        } else {
            None
        };

        // PRG ROM
        let prg_rom_size_bytes = header.prg_rom_size as usize * 16384;
        let mut prg_rom = vec![0; prg_rom_size_bytes];
        rom.read_exact(&mut prg_rom)?;

        // CHR ROM/RAM
        let chr_rom = if header.chr_rom_size > 0 {
            let chr_rom_size_bytes = header.chr_rom_size as usize * 8192;
            let mut chr_rom = vec![0; chr_rom_size_bytes];
            rom.read_exact(&mut chr_rom)?;
            Some(chr_rom)
        } else {
            //let mut chr_ram = vec![0; 8192];
            //rom.read_exact(&mut chr_ram)?;
            //Some(chr_ram)
            None
        };

        // PlayChoice INST-ROM
        let playchoice_inst_rom = if header.playchoice_10 {
            let mut playchoice_inst_rom = [0; 8192];
            rom.read_exact(&mut playchoice_inst_rom)?;
            Some(playchoice_inst_rom)
        } else {
            None
        };

        // PlayChoice PROM
        let playchoice_prom = if header.playchoice_10 {
            let mut playchoice_prom = [0; 32];
            rom.read_exact(&mut playchoice_prom)?;
            Some(playchoice_prom)
        } else {
            None
        };

        Ok(Cartridge {
            path,
            header,
            trainer,
            prg_rom,
            chr_rom,
            playchoice_inst_rom,
            playchoice_prom,
        })
    }
    pub fn mirroring(&self) -> NametableMirroring {
        self.header.nametable_mirroring
    }
}
impl SerialInterface for Cartridge {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x6000..=0x7FFF => {
                todo!("PRG RAM");
            }
            0x8000..=0xFFFF => {
                let mut offset = (address - 0x8000) as usize;

                // 16 KiB ROMs are mirrored twice
                if self.prg_rom.len() == 0x4000 {
                    offset &= 0x3FFF;
                }

                self.prg_rom[offset]
            }

            _ => panic!("Invalid cartridge read at {:04X}", address),
        }
    }

    fn write_byte(&mut self, address: u16, _value: u8) {
        match address {
            0x8000..=0xFFFF => {
                // NROM has no writable mapper registers.
            }

            0x6000..=0x7FFF => {
                todo!("PRG RAM");
            }

            _ => panic!("Invalid cartridge write at {:04X}", address),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;
    use sha1::Digest;

    #[test]
    fn read_cartridge_correctly_from_file() {
        /* <game>
            <!-- Licensed Japan\Hello Kitty World.nes -->
            <prgrom size="131072" crc32="67D5C3F9" sha1="42E0AFDD1E603C4F301AEB030B799F69EEBE2E15" sum16="1FFF"/>
            <rom size="131072" crc32="67D5C3F9" sha1="42E0AFDD1E603C4F301AEB030B799F69EEBE2E15"/>
            <chrram size="8192"/>
            <pcb mapper="2" submapper="2" mirroring="V" battery="0"/>
            <console type="0" region="0"/>
            <expansion type="1"/>
        </game> */
        let path = PathBuf::from("../roms/hellokitty.nes");
        let cartridge = Cartridge::from_file(path).unwrap();

        // Header
        assert_eq!(cartridge.header.prg_rom_size, 8);
        assert_eq!(cartridge.header.chr_rom_size, 0);
        assert_eq!(
            cartridge.header.nametable_mirroring,
            NametableMirroring::Vertical
        );
        assert_eq!(cartridge.header.contains_battery_pack, false);
        assert_eq!(cartridge.header.has_trainer, false);
        assert_eq!(cartridge.header.alternative_nametable_layout, false);
        assert_eq!(cartridge.header.mapper_number, 2);
        assert_eq!(cartridge.header.vs_unisystem, false);
        assert_eq!(cartridge.header.playchoice_10, false);
        assert_eq!(cartridge.header.is_nes_2_0, false);
        assert_eq!(cartridge.header.prg_ram_size, 0);

        // PRG ROM
        assert_eq!(cartridge.prg_rom.len(), 131072);
        assert_eq!(
            sha1::Sha1::digest(&cartridge.prg_rom),
            hex!("42E0AFDD1E603C4F301AEB030B799F69EEBE2E15")
        );
    }

    #[test]
    fn read_herebreke_cartridge_correctly_from_file() {
        /*
        <game>
            <!-- Licensed Japan\Hebereke.nes -->
            <prgrom size="131072" crc32="70E0B7D8" sha1="64F18CA61861B6C4070B1450135D3160D8468F43" sum16="9EEF"/>
            <chrrom size="131072" crc32="8953EEDF" sha1="5010BDBECE12FD0D19D2CB34F88EF3A5C05DC196" sum16="C314"/>
            <rom size="262144" crc32="72928698" sha1="7D10C6DD141DA35A1672F127791639B5816C692D"/>
            <pcb mapper="69" submapper="0" mirroring="H" battery="0"/>
            <console type="0" region="0"/>
            <expansion type="1"/>
        </game>
         */
        let path = PathBuf::from("../roms/herebreke.nes");
        let cartridge = Cartridge::from_file(path).unwrap();

        // Header
        assert_eq!(cartridge.header.prg_rom_size, 8);
        assert_eq!(cartridge.header.chr_rom_size, 16);
        assert_eq!(
            cartridge.header.nametable_mirroring,
            NametableMirroring::Horizontal
        );
        assert_eq!(cartridge.header.contains_battery_pack, false);
        assert_eq!(cartridge.header.has_trainer, false);
        assert_eq!(cartridge.header.alternative_nametable_layout, false);
        assert_eq!(cartridge.header.mapper_number, 69);
        assert_eq!(cartridge.header.vs_unisystem, false);
        assert_eq!(cartridge.header.playchoice_10, false);
        assert_eq!(cartridge.header.is_nes_2_0, false);
        assert_eq!(cartridge.header.prg_ram_size, 0);

        // PRG ROM
        assert_eq!(cartridge.prg_rom.len(), 131072);
        assert_eq!(
            sha1::Sha1::digest(&cartridge.prg_rom),
            hex!("64F18CA61861B6C4070B1450135D3160D8468F43")
        );

        // CHR ROM
        assert!(cartridge.chr_rom.is_some());
        assert_eq!(cartridge.chr_rom.as_ref().unwrap().len(), 131072);
        assert_eq!(
            sha1::Sha1::digest(&cartridge.chr_rom.unwrap()),
            hex!("5010BDBECE12FD0D19D2CB34F88EF3A5C05DC196")
        );
    }
}
