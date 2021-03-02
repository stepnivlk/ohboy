use std::convert::TryInto;

const BOOT_ROM_START: usize = 0x00;
const BOOT_ROM_END: usize = 0xFF;
pub const BOOT_ROM_SIZE: usize = BOOT_ROM_END - BOOT_ROM_START + 1;

const ROM_BANK_0_START: usize = 0x000;
const ROM_BANK_0_END: usize = 0x3FFF;
pub const ROM_BANK_0_SIZE: usize = ROM_BANK_0_END - ROM_BANK_0_START + 1;

const ROM_BANK_N_START: usize = 0x4000;
const ROM_BANK_N_END: usize = 0x7FFF;
const ROM_BANK_N_SIZE: usize = ROM_BANK_N_END - ROM_BANK_N_START + 1;

const V_RAM_START: usize = 0x8000;
const V_RAM_END: usize = 0x9FFF;
pub const V_RAM_SIZE: usize = V_RAM_END - V_RAM_START + 1;

const E_RAM_START: usize = 0xA000;
const E_RAM_END: usize = 0xBFFF;
const E_RAM_SIZE: usize = E_RAM_END - E_RAM_START + 1;

const W_RAM_START: usize = 0xC000;
const W_RAM_END: usize = 0xDFFF;
const W_RAM_SIZE: usize = W_RAM_END - W_RAM_START + 1;

const W_RAM_SHAD_START: usize = 0xE000;
const W_RAM_SHAD_END: usize = 0xFDFF;
const W_RAM_SHAD_SIZE: usize = W_RAM_SHAD_END - W_RAM_SHAD_START + 1;

// TODO: Maps to GPU; 160B data, rest 0.
const OAM_START: usize = 0xFE00;
const OAM_END: usize = 0xFE9F;
pub const OAM_SIZE: usize = OAM_END - OAM_START + 1;

const IO_REGS_START: usize = 0xFF00;
const IO_REGS_END: usize = 0xFF7F;
const IO_REGS_SIZE: usize = IO_REGS_END - IO_REGS_START + 1;

const Z_RAM_START: usize = 0xFF80;
const Z_RAM_END: usize = 0xFFFE;
const Z_RAM_SIZE: usize = Z_RAM_END - Z_RAM_START + 1;

pub struct MemoryBus {
    in_bios: bool,
    boot_rom: [u8; BOOT_ROM_SIZE],
    rom_bank_0: [u8; ROM_BANK_0_SIZE],
    rom_bank_n: [u8; ROM_BANK_N_SIZE],
    e_ram: [u8; E_RAM_SIZE],
    w_ram: [u8; W_RAM_SIZE],
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
            in_bios: false,
            boot_rom,
            rom_bank_0,
            // TODO: Is this correct? How do the switchable banks (if present) map?
            rom_bank_n: [0; ROM_BANK_N_SIZE],
            // TODO: Use GPU instance.
            v_ram: [0; V_RAM_SIZE],
            e_ram: [0; E_RAM_SIZE],
            w_ram: [0; W_RAM_SIZE],
            z_ram: [0; Z_RAM_SIZE],
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        let address = address as usize;

        match address {
            BOOT_ROM_START..=BOOT_ROM_END => {
                return self.boot_rom[address];

                if self.in_bios {
                    if address < 0x0100 {
                        return self.boot_rom[address];
                    // TODO: Check if PC points to 0x0100 here
                    // This means we're first time behind the boot (0xFF)
                    // How can be PC passed though?
                    // Can there be some simpler approach without need for mut?
                    // CPU should tell membus when to switch
                    } else if false {
                        // self.in_bios = false;
                    }
                };

                self.rom_bank_0[address]
            }
            ROM_BANK_0_START..=ROM_BANK_0_END => self.rom_bank_0[address],
            ROM_BANK_N_START..=ROM_BANK_N_END => {
                panic!("ROM N read at {:x}", address);
            }
            IO_REGS_START..=IO_REGS_END => {
                if address == 0xFF44 {
                    panic!("FF41 Bit 4 - Mode 1 V-Blank Interrupt");
                }
                // TODO: Read IO
                0
            }
            E_RAM_START..=E_RAM_END => self.e_ram[address - E_RAM_START],
            W_RAM_START..=W_RAM_END => self.w_ram[address - W_RAM_START],
            V_RAM_START..=V_RAM_END => self.v_ram[address - V_RAM_START],
            Z_RAM_START..=Z_RAM_END => self.z_ram[address - Z_RAM_START],
            _ => {
                panic!("unimplemented mem read at: 0x{:x}", address);
            }
        }
    }

    pub fn write_byte(&mut self, address: u16, byte: u8) {
        let address = address as usize;

        match address {
            ROM_BANK_0_START..=ROM_BANK_0_END => {
                self.rom_bank_0[address] = byte;
            }
            IO_REGS_START..=IO_REGS_END => {
                // TODO: Write IO
            }
            E_RAM_START..=E_RAM_END => {
                self.e_ram[address - E_RAM_START] = byte;
            }
            W_RAM_START..=W_RAM_END => {
                self.w_ram[address - W_RAM_START] = byte;
            }
            V_RAM_START..=V_RAM_END => {
                self.v_ram[address - V_RAM_START] = byte;
            }
            Z_RAM_START..=Z_RAM_END => {
                self.z_ram[address - Z_RAM_START] = byte;
            }
            _ => {
                panic!("unimplemented mem write at: 0x{:x}", address);
            }
        }
    }
}
