use crate::mmu::{OAM_SIZE, V_RAM_SIZE};

const SCREEN_SIZE: usize = 168 * 144 * 4;

enum Mode {
    ScanlineOam,
    ScanlineVram,
    Hblank,
    Vblank,
}

// TODO: Who should own the CPU, what is the hiearchy of components?
// RN: CPU -> Bus -> GPU
// Q? Bus -> CPU
//        -> GPU
pub struct Gpu {
    pub v_ram: [u8; V_RAM_SIZE],
    oam: [i8; OAM_SIZE],
    // TODO: Has to be accessible by external framebuffer
    // Push to fb every vblank
    screen: [u8; SCREEN_SIZE],
    modeclock: u32,
    mode: Mode,
    line: u8,
}

impl Gpu {
    pub fn new() -> Self {
        Self {
            v_ram: [0; V_RAM_SIZE],
            oam: [0; OAM_SIZE],
            screen: [0; SCREEN_SIZE],
            modeclock: 0,
            mode: Mode::Hblank,
            line: 0,
        }
    }

    pub fn step(&mut self, ticks: u8) {
        self.modeclock += ticks as u32;

        match self.mode {
            Mode::ScanlineOam => {
                if self.modeclock >= 80 {
                    self.mode = Mode::ScanlineVram;
                    self.modeclock = 0;
                }
            },

            Mode::ScanlineVram => {
                if self.modeclock >= 172 {
                    self.mode = Mode::Hblank;
                    self.modeclock = 0;

                    // TODO: Write scanline
                }
            },

            Mode::Hblank => {
                if self.modeclock >= 204 {
                    self.modeclock = 0;
                    self.line += self.line;

                    if self.line == 143 {
                        self.mode = Mode::Vblank;

                        // TODO: screen to framebuffer
                    } else {
                        self.mode = Mode::ScanlineOam;
                    }
                }
            },

            Mode::Vblank => {
                if self.modeclock >= 456 {
                    self.modeclock = 0;
                    self.line += self.line;

                    if self.line > 153 {
                        self.mode = Mode::ScanlineOam;
                        self.line = 0;
                    }
                }
            },

            _ => panic!("unimplemented GPU mode"),
        }
    }
}
