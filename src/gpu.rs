use crate::memory_bus::{OAM_SIZE, V_RAM_SIZE};

const SCREEN_SIZE: usize = 168 * 144 * 4;

// TODO: Who should own the CPU, what is the hiearchy of components?
// RN: CPU -> Bus -> GPU
// Q? Bus -> CPU
//        -> GPU
pub struct GPU {
    vram: [u8; V_RAM_SIZE],
    oam: [i8; OAM_SIZE],
    // TODO: Has to be accessible by external framebuffer
    // Push to fb every vblank
    screen: [u8; SCREEN_SIZE],
    modeclock: u32,
    mode: u8,
    line: u8,
}

impl GPU {
    pub fn new() -> Self {
        Self {
            vram: [0; V_RAM_SIZE],
            oam: [0; OAM_SIZE],
            screen: [0; SCREEN_SIZE],
            modeclock: 0,
            mode: 0,
            line: 0,
        }
    }

    pub fn step(&self) {
        // TODO: Add the latest CPU clock to modeclock

        match self.mode {
            _ => panic!("unimplemented GPU mode"),
        }
    }
}
