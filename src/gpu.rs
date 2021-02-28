use crate::memory_bus::{V_RAM_SIZE, OAM_SIZE};

const SCREEN_SIZE: usize = 168 * 144 * 4;

pub struct GPU {
    vram: [u8; V_RAM_SIZE],
    oam: [i8; OAM_SIZE],
    // TODO: Has to be accessible by external framebuffer
    // Push to fb every vblank
    screen: [u8; SCREEN_SIZE],
}

impl GPU {
    pub fn new() -> Self {
        Self {
            vram: [0; V_RAM_SIZE],
            oam: [0; OAM_SIZE],
            screen: [0; SCREEN_SIZE],
        }
    }
}
