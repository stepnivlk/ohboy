extern crate minifb;

mod cpu;
mod gpu;
mod instr;
mod microcode;
mod mmu;
mod registers;

use minifb::{Key, Window, WindowOptions};

use cpu::Cpu;
use mmu::Mmu;

struct Board {
    cpu: Cpu,
    mmu: Mmu,
}

fn buffer_from_file(path: &str) -> Vec<u8> {
    use std::io::Read;

    let mut file = std::fs::File::open(path).unwrap();
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).unwrap();

    buffer
}

fn main() {
    let mut window = Window::new("Game On", 160, 144, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    window.limit_update_rate(Some(std::time::Duration::from_micros(64400)));

    let boot_rom_buffer = buffer_from_file("b_rom.gb");
    let game_rom_buffer = buffer_from_file("tetris_rom.gb");

    let mut cpu = Cpu::new(boot_rom_buffer, game_rom_buffer, None);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        cpu.step();
    }
}
