use std::convert::TryInto;

const BOOT_ROM_START: usize = 0x00;
const BOOT_ROM_END: usize = 0xFF;
pub const BOOT_ROM_SIZE: usize = BOOT_ROM_END - BOOT_ROM_START + 1;

const ROM_BANK_0_START: usize = 0x000;
const ROM_BANK_0_END: usize = 0x3FFF;
pub const ROM_BANK_0_SIZE: usize = ROM_BANK_0_END - ROM_BANK_0_START + 1;

const V_RAM_START: usize = 0x8000;
const V_RAM_END: usize = 0x9FFF;
pub const V_RAM_SIZE: usize = V_RAM_END - V_RAM_START + 1;

const Z_RAM_START: usize = 0xFF80;
const Z_RAM_END: usize = 0xFFFE;
pub const Z_RAM_SIZE: usize = Z_RAM_END - Z_RAM_START + 1;

const IO_REGS_START: usize = 0xFF00;
const IO_REGS_END: usize = 0xFF7F;
pub const IO_REGS_SIZE: usize = IO_REGS_END - IO_REGS_START + 1;

pub struct MemoryBus {
    boot_rom: [u8; BOOT_ROM_SIZE],
    rom_bank_0: [u8; ROM_BANK_0_SIZE],
    z_ram: [u8; Z_RAM_SIZE],
    // TODO: Move to gpu
    pub v_ram: [u8; V_RAM_SIZE],
}

impl MemoryBus {
    pub fn new(boot_rom_buffer: Vec<u8>, game_rom_buffer: Vec<u8>) -> Self {
        let boot_rom: [u8; BOOT_ROM_SIZE] = boot_rom_buffer.try_into().unwrap();

        let mut rom_bank_0 = [0; ROM_BANK_0_SIZE];

        for i in 0..ROM_BANK_0_SIZE {
            rom_bank_0[i] = game_rom_buffer[i];
        }

        Self {
            boot_rom,
            rom_bank_0,
            v_ram: [0; V_RAM_SIZE],
            z_ram: [0; Z_RAM_SIZE],
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        let address = address as usize;

        match address {
            BOOT_ROM_START..=BOOT_ROM_END => {
                // TODO: Test for rom0 too
                let r = self.boot_rom[address];

                r
            },
            ROM_BANK_0_START..=ROM_BANK_0_END => {
                let r = self.rom_bank_0[address];

                r
            },
            IO_REGS_START..=IO_REGS_END => {
                // TODO: Read IO
                0
            },
            V_RAM_START..=V_RAM_END => self.v_ram[address - V_RAM_START],
            Z_RAM_START..=Z_RAM_END => self.z_ram[address - Z_RAM_START],
            _ => {
                panic!("unimplemented mem read at: 0x{:x}", address);
            },
        }
    }

    pub fn write_byte(&mut self, address: u16, byte: u8) {
        let address = address as usize;

        match address {
            ROM_BANK_0_START..=ROM_BANK_0_END => {
                self.rom_bank_0[address] = byte;
            },
            IO_REGS_START..=IO_REGS_END => {
                // TODO: Write IO
            },
            V_RAM_START..=V_RAM_END => {
                self.v_ram[address - V_RAM_START] = byte;
            },
            Z_RAM_START..=Z_RAM_END => {
                self.z_ram[address - Z_RAM_START] = byte;
            },
            _ => {
                panic!("unimplemented mem write at: 0x{:x}", address);
            },
        }
    }
}
