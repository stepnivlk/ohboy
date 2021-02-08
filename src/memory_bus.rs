use std::convert::TryInto;

const BOOT_ROM_START: usize = 0x00;
const BOOT_ROM_END: usize = 0xFF;
pub const BOOT_ROM_SIZE: usize = BOOT_ROM_END - BOOT_ROM_START + 1;

const ROM_BANK_0_START: usize = 0x000;
const ROM_BANK_0_END: usize = 0x3FFF;
pub const ROM_BANK_0_SIZE: usize = ROM_BANK_0_END - ROM_BANK_0_START + 1;

pub struct MemoryBus {
    memory: [u8; 0xFFFF],
    boot_rom: [u8; BOOT_ROM_SIZE],
    rom_bank_0: [u8; ROM_BANK_0_SIZE],
}

impl MemoryBus {
    pub fn new(boot_rom_buffer: Vec<u8>, game_rom_buffer: Vec<u8>) -> Self {
        let boot_rom: [u8; BOOT_ROM_SIZE] = boot_rom_buffer.try_into().unwrap();

        let mut rom_bank_0 = [0; ROM_BANK_0_SIZE];

        for i in 0..ROM_BANK_0_SIZE {
            rom_bank_0[i] = game_rom_buffer[i];
        }

        Self {
            memory: [0; 0xFFFF],
            boot_rom,
            rom_bank_0,
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        let address = address as usize;

        match address {
            BOOT_ROM_START..=BOOT_ROM_END => {
                // TODO: Test for rom0 too
                let r = self.boot_rom[address];

                r
            }
            ROM_BANK_0_START..=ROM_BANK_0_END => {
                let r = self.rom_bank_0[address];

                r
            }
            _ => {
                // self.memory[address]
                panic!("unimplemented mem access at: 0x{:x}", address);
            }
        }
    }

    pub fn write_byte(&mut self, address: u16, byte: u8) {
        // TODO: Remove & use segmented write
        self.memory[address as usize] = byte;
    }
}
