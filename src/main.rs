mod executors;
mod instruction;
mod memory_bus;
mod registers;

use executors::values;
use instruction::{Instr, InstrKind};
use memory_bus::MemoryBus;
use registers::Registers;

pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub enum ADDHLTarget {
    BC,
    DE,
    HL,
    SP,
}

#[derive(Copy, Clone)]
pub enum LoadByteTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
}

pub enum LoadWordTarget {
    BC,
    DE,
    HL,
    SP,
}

#[derive(Copy, Clone)]
pub enum LoadByteSource {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    D8,
}

#[derive(Copy, Clone)]
pub enum StackTarget {
    BC,
    DE,
    HL,
}

fn dbg_b<T: std::fmt::LowerHex>(b: T) {
    dbg!(format!("{:x}", b));
}

struct Pc(u16);

impl Pc {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn get(&self) -> u16 {
        self.0
    }

    pub fn peek(&self) -> u16 {
        self.0 + 1
    }

    pub fn add(&mut self, val: u16) {
        self.0 = self.0.wrapping_add(val);
    }

    pub fn set(&mut self, val: u16) {
        self.0 = val;
    }
}

#[derive(PartialEq)]
enum State {
    Running,
    Halted,
}

pub struct CPU {
    registers: Registers,
    pc: Pc,
    sp: u16,
    bus: MemoryBus,
    state: State,
}

impl CPU {
    fn new(
        boot_rom_buffer: Vec<u8>,
        game_rom_buffer: Vec<u8>,
        registers: Option<Registers>,
    ) -> Self {
        Self {
            registers: registers.unwrap_or(Registers::new()),
            pc: Pc::new(),
            sp: 0,
            bus: MemoryBus::new(boot_rom_buffer, game_rom_buffer),
            state: State::Running,
        }
    }

    fn step(&mut self) {
        let instruction = self.bus.read_byte(self.pc.get());

        if instruction == 0xCB {
            let instruction = self.bus.read_byte(self.pc.peek());

            self.execute_prefixed(instruction)
        } else {
            let i = Instr::from(instruction);
            println!("{}", i);

            self.execute(instruction, i)
        };
    }

    fn execute(&mut self, instruction: u8, instr: Instr) {
        use executors::*;

        if self.state == State::Halted {
            return;
        }

        match instr.kind {
            InstrKind::Nop => self.pc.add(1),
            InstrKind::Halt => self.state = State::Halted,
            InstrKind::Add(from) => {
                let val = self.registers.get(from);
                let (new_val, carry) = self.registers.a.overflowing_add(val);

                self.registers.f.zero = val == 0;
                self.registers.f.subtract = false;
                // TODO: Incorrect
                self.registers.f.half_carry = val > 0xF;
                self.registers.f.carry = carry;

                self.registers.a = new_val;
                self.pc.add(1);
            },
            InstrKind::Adc(from) => {
                let val = self.registers.get(from);
                let additinal_carry = if self.registers.f.carry { 1 } else { 0 };
                let (mid_value, mid_carry) = self.registers.a.overflowing_add(val);
                let (new_value, carry) = mid_value.overflowing_add(additinal_carry);

                self.registers.f.zero = new_value == 0;
                self.registers.f.subtract = false;
                // TODO: Incorrect
                self.registers.f.half_carry = new_value > 0xF;
                self.registers.f.carry = mid_carry || carry;

                self.registers.a = new_value;
                self.pc.add(1);
            },
            InstrKind::AddHl(from) => {
                let val = self.registers.get_word(from);
                let curr_hl = self.registers.get_hl();
                let (new_value, carry) = curr_hl.overflowing_add(val);

                self.registers.f.carry = carry;
                self.registers.f.subtract = false;
                // TODO: Incorrect
                self.registers.f.half_carry = val > 0xFF;

                self.registers.set_hl(new_value);
                self.pc.add(1);
            },
            InstrKind::Sub(from) => {
                let val = self.registers.get(from);
                let a = self.registers.a;

                let (new_value, carry) = a.overflowing_sub(val);

                self.registers.f.zero = new_value == 0;
                self.registers.f.subtract = true;
                // TODO: Should add a carry?
                self.registers.f.half_carry = (a & 0xF) < (val & 0xF);
                self.registers.f.carry = carry;


                self.registers.a = new_value;
                self.pc.add(1);
            },

            _ => {}
        };

        match instruction {
            // 0x00 => nop(self),
//
            // 0x76 => halt(self),
//
            // 0x80 => add(self, ArithmeticTarget::B),
            // 0x81 => add(self, ArithmeticTarget::C),
            // 0x82 => add(self, ArithmeticTarget::D),
            // 0x83 => add(self, ArithmeticTarget::E),
            // 0x84 => add(self, ArithmeticTarget::H),
            // 0x85 => add(self, ArithmeticTarget::L),
            // 0x87 => add(self, ArithmeticTarget::A),
//
            // 0x88 => adc(self, ArithmeticTarget::B),
            // 0x89 => adc(self, ArithmeticTarget::C),
            // 0x8A => adc(self, ArithmeticTarget::D),
            // 0x8B => adc(self, ArithmeticTarget::E),
            // 0x8C => adc(self, ArithmeticTarget::H),
            // 0x8D => adc(self, ArithmeticTarget::L),
            // 0x8F => adc(self, ArithmeticTarget::A),
//
            // 0x09 => add_hl(self, ADDHLTarget::BC),
            // 0x19 => add_hl(self, ADDHLTarget::DE),
            // 0x29 => add_hl(self, ADDHLTarget::HL),
            // 0x39 => add_hl(self, ADDHLTarget::SP),
//
            0x90 => sub(self, ArithmeticTarget::B),
            0x91 => sub(self, ArithmeticTarget::C),
            0x92 => sub(self, ArithmeticTarget::D),
            0x93 => sub(self, ArithmeticTarget::E),
            0x94 => sub(self, ArithmeticTarget::H),
            0x95 => sub(self, ArithmeticTarget::L),
            0x97 => sub(self, ArithmeticTarget::A),

            0x98 => sbc(self, ArithmeticTarget::B),
            0x99 => sbc(self, ArithmeticTarget::C),
            0x9A => sbc(self, ArithmeticTarget::D),
            0x9B => sbc(self, ArithmeticTarget::E),
            0x9C => sbc(self, ArithmeticTarget::H),
            0x9D => sbc(self, ArithmeticTarget::L),
            0x9F => sbc(self, ArithmeticTarget::A),

            0xA0 => and(self, ArithmeticTarget::B),
            0xA1 => and(self, ArithmeticTarget::C),
            0xA2 => and(self, ArithmeticTarget::D),
            0xA3 => and(self, ArithmeticTarget::E),
            0xA4 => and(self, ArithmeticTarget::H),
            0xA5 => and(self, ArithmeticTarget::L),
            0xA7 => and(self, ArithmeticTarget::A),

            0xB0 => or(self, ArithmeticTarget::B),
            0xB1 => or(self, ArithmeticTarget::C),
            0xB2 => or(self, ArithmeticTarget::D),
            0xB3 => or(self, ArithmeticTarget::E),
            0xB4 => or(self, ArithmeticTarget::H),
            0xB5 => or(self, ArithmeticTarget::L),
            0xB7 => or(self, ArithmeticTarget::A),

            0xA8 => xor(self, ArithmeticTarget::B),
            0xA9 => xor(self, ArithmeticTarget::C),
            0xAA => xor(self, ArithmeticTarget::D),
            0xAB => xor(self, ArithmeticTarget::E),
            0xAC => xor(self, ArithmeticTarget::H),
            0xAD => xor(self, ArithmeticTarget::L),
            0xAF => xor(self, ArithmeticTarget::A),

            0xC3 => jp(self, true),
            0xC2 => jp(self, !self.registers.f.zero),
            0xD2 => jp(self, !self.registers.f.carry),
            0xCA => jp(self, self.registers.f.zero),
            0xDA => jp(self, self.registers.f.carry),

            // JR NZ,i8
            0x20 => {
                let next_step = self.pc.get().wrapping_add(2);

                if self.registers.f.zero {
                    // if true {
                    // do not jump
                    self.pc.set(next_step);
                } else {
                    // jump
                    let offset = self.read_next_byte() as i8;

                    let next_pc = if offset >= 0 {
                        next_step.wrapping_add(offset as u16)
                    } else {
                        next_step.wrapping_sub(offset.abs() as u16)
                    };

                    self.pc.set(next_pc);
                }
            }

            0x21 => {
                let word = self.read_next_word();

                self.registers.set_hl(word);

                self.pc.add(3);
            }

            0x31 => {
                let word = self.read_next_word();

                self.sp = word;

                self.pc.add(3);
            }

            // LD (HL-),A
            // Load from A to addr pointed by HL and decrement HL
            0x32 => {
                let a = self.registers.a;
                let hl = self.registers.get_hl();

                self.bus.write_byte(hl, a);

                self.registers.set_hl(hl.wrapping_sub(1));

                self.pc.add(1);
            }

            // LD (FF00+C),A
            // Store value in register A into byte at address $FF00+C.
            0xE2 => {
                let addr = 0xFF00 + (self.registers.c as u16);

                self.bus.write_byte(addr, self.registers.a);

                self.pc.add(1);
            }

            // INC C
            // Increment value in register r8 by 1.
            0x0C => {
                let c = self.registers.c.wrapping_add(1);

                self.registers.f.zero = c == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = c & 0xF == 0xF;

                self.registers.c = c;

                self.pc.add(1);
            }

            // LD (FF00+u8),A
            0xE0 => {
                let payload = self.read_next_byte();

                let addr = 0xFF00 + (payload as u16);

                self.bus.write_byte(addr, self.registers.a);

                self.pc.add(2);
            }

            // LD DE,u16
            0x11 => {
                let payload = self.read_next_word();

                self.registers.set_de(payload);

                self.pc.add(3);
            }

            // LD A,(DE)
            // Load value in register A from byte pointed to by register r16.
            0x1A => {
                let data = self.bus.read_byte(self.registers.get_de());

                self.registers.a = data;

                self.pc.add(1);
            }

            0x40 => ld(self, LoadByteTarget::B, LoadByteSource::B),
            0x41 => ld(self, LoadByteTarget::B, LoadByteSource::C),
            0x42 => ld(self, LoadByteTarget::B, LoadByteSource::D),
            0x43 => ld(self, LoadByteTarget::B, LoadByteSource::E),
            0x44 => ld(self, LoadByteTarget::B, LoadByteSource::H),
            0x45 => ld(self, LoadByteTarget::B, LoadByteSource::L),
            0x46 => ld(self, LoadByteTarget::B, LoadByteSource::HL),
            0x47 => ld(self, LoadByteTarget::B, LoadByteSource::A),

            0x48 => ld(self, LoadByteTarget::C, LoadByteSource::B),
            0x49 => ld(self, LoadByteTarget::C, LoadByteSource::C),
            0x4A => ld(self, LoadByteTarget::C, LoadByteSource::D),
            0x4B => ld(self, LoadByteTarget::C, LoadByteSource::E),
            0x4D => ld(self, LoadByteTarget::C, LoadByteSource::L),
            0x4E => ld(self, LoadByteTarget::C, LoadByteSource::HL),
            0x4F => ld(self, LoadByteTarget::C, LoadByteSource::A),

            0x50 => ld(self, LoadByteTarget::D, LoadByteSource::B),
            0x51 => ld(self, LoadByteTarget::D, LoadByteSource::C),
            0x52 => ld(self, LoadByteTarget::D, LoadByteSource::D),
            0x53 => ld(self, LoadByteTarget::D, LoadByteSource::E),
            0x54 => ld(self, LoadByteTarget::D, LoadByteSource::H),
            0x55 => ld(self, LoadByteTarget::D, LoadByteSource::L),
            0x56 => ld(self, LoadByteTarget::D, LoadByteSource::HL),
            0x57 => ld(self, LoadByteTarget::D, LoadByteSource::A),

            0x58 => ld(self, LoadByteTarget::E, LoadByteSource::B),
            0x59 => ld(self, LoadByteTarget::E, LoadByteSource::C),
            0x5A => ld(self, LoadByteTarget::E, LoadByteSource::D),
            0x5B => ld(self, LoadByteTarget::E, LoadByteSource::E),
            0x5D => ld(self, LoadByteTarget::E, LoadByteSource::L),
            0x5E => ld(self, LoadByteTarget::E, LoadByteSource::HL),
            0x5F => ld(self, LoadByteTarget::E, LoadByteSource::A),

            0x60 => ld(self, LoadByteTarget::H, LoadByteSource::B),
            0x61 => ld(self, LoadByteTarget::H, LoadByteSource::C),
            0x62 => ld(self, LoadByteTarget::H, LoadByteSource::D),
            0x63 => ld(self, LoadByteTarget::H, LoadByteSource::E),
            0x64 => ld(self, LoadByteTarget::H, LoadByteSource::H),
            0x65 => ld(self, LoadByteTarget::H, LoadByteSource::L),
            0x66 => ld(self, LoadByteTarget::H, LoadByteSource::HL),
            0x67 => ld(self, LoadByteTarget::H, LoadByteSource::A),

            0x68 => ld(self, LoadByteTarget::L, LoadByteSource::B),
            0x69 => ld(self, LoadByteTarget::L, LoadByteSource::C),
            0x6A => ld(self, LoadByteTarget::L, LoadByteSource::D),
            0x6B => ld(self, LoadByteTarget::L, LoadByteSource::E),
            0x6D => ld(self, LoadByteTarget::L, LoadByteSource::L),
            0x6E => ld(self, LoadByteTarget::L, LoadByteSource::HL),
            0x6F => ld(self, LoadByteTarget::L, LoadByteSource::A),

            0x70 => ld(self, LoadByteTarget::HL, LoadByteSource::B),
            0x71 => ld(self, LoadByteTarget::HL, LoadByteSource::C),
            0x72 => ld(self, LoadByteTarget::HL, LoadByteSource::D),
            0x73 => ld(self, LoadByteTarget::HL, LoadByteSource::E),
            0x74 => ld(self, LoadByteTarget::HL, LoadByteSource::H),
            0x75 => ld(self, LoadByteTarget::HL, LoadByteSource::L),
            0x77 => ld(self, LoadByteTarget::HL, LoadByteSource::A),

            0x78 => ld(self, LoadByteTarget::A, LoadByteSource::B),
            0x79 => ld(self, LoadByteTarget::A, LoadByteSource::C),
            0x7A => ld(self, LoadByteTarget::A, LoadByteSource::D),
            0x7B => ld(self, LoadByteTarget::A, LoadByteSource::E),
            0x7D => ld(self, LoadByteTarget::A, LoadByteSource::L),
            0x7E => ld(self, LoadByteTarget::A, LoadByteSource::HL),
            0x7F => ld(self, LoadByteTarget::A, LoadByteSource::A),

            0x3E => ld(self, LoadByteTarget::A, LoadByteSource::D8),
            0x06 => ld(self, LoadByteTarget::B, LoadByteSource::D8),
            0x0E => ld(self, LoadByteTarget::C, LoadByteSource::D8),
            0x16 => ld(self, LoadByteTarget::D, LoadByteSource::D8),
            0x1E => ld(self, LoadByteTarget::E, LoadByteSource::D8),
            0x26 => ld(self, LoadByteTarget::H, LoadByteSource::D8),
            0x2E => ld(self, LoadByteTarget::L, LoadByteSource::D8),
            0x36 => ld(self, LoadByteTarget::HL, LoadByteSource::D8),

            0xC1 => pop(self, StackTarget::BC),
            0xD1 => pop(self, StackTarget::DE),
            0xE1 => pop(self, StackTarget::HL),

            0xC5 => push(self, StackTarget::BC),
            0xD5 => push(self, StackTarget::DE),
            0xE5 => push(self, StackTarget::HL),

            0xC4 => call(self, !self.registers.f.zero),
            0xD4 => call(self, !self.registers.f.carry),
            0xCC => call(self, self.registers.f.zero),
            0xDC => call(self, self.registers.f.carry),
            0xCD => call(self, true),

            0xC0 => ret(self, !self.registers.f.zero),
            0xD0 => ret(self, !self.registers.f.carry),
            0xC8 => ret(self, self.registers.f.zero),
            0xD8 => ret(self, self.registers.f.carry),
            0xC9 => ret(self, true),

            _ => {
                dbg!(self.pc.get());
                dbg!(self.sp);
                dbg!(&self.registers);
                panic!("Unknown instruction for 0x{:x}", instruction);
            }
        }
    }

    fn execute_prefixed(&mut self, instruction: u8) {
        match instruction {
            // BIT 7,H
            0x7C => {
                let bit_position = 7;
                let bit = (self.registers.h >> bit_position) & 0b1;

                self.registers.f.zero = bit == 0;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;

                // let h_reg = format!("{:b}", self.registers.h);
                // dbg!(h_reg);

                self.pc.add(2);
            }

            _ => {
                dbg!(self.pc.get());
                dbg!(self.sp);
                dbg!(&self.registers);
                panic!("Unknown instruction for 0xCB{:X}", instruction);
            }
        }
    }

    fn read_at_hl(&self) -> u8 {
        self.bus.read_byte(self.registers.get_hl())
    }

    fn read_next_byte(&self) -> u8 {
        self.bus.read_byte(self.pc.peek())
    }

    fn read_next_word(&self) -> u16 {
        let lo = self.bus.read_byte(self.pc.get() + 1) as u16;
        let hi = self.bus.read_byte(self.pc.get() + 2) as u16;

        (hi << 8) | lo
    }
}

fn buffer_from_file(path: &str) -> Vec<u8> {
    use std::io::Read;

    let mut file = std::fs::File::open(path).unwrap();
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).unwrap();

    buffer
}

fn main() {
    // use std::{thread, time};

    // let sleep_t = time::Duration::from_millis(100);

    let boot_rom_buffer = buffer_from_file("b_rom.gb");
    let game_rom_buffer = buffer_from_file("tetris_rom.gb");

    let mut cpu = CPU::new(boot_rom_buffer, game_rom_buffer, None);

    loop {
        cpu.step();

        // thread::sleep(sleep_t);
    }
}
