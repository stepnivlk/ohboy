extern crate minifb;

mod executors;
mod instruction;
mod memory_bus;
mod registers;
mod gpu;

use minifb::{Key, Window, WindowOptions};

use instruction::{Instr, InstrKind, Operand};
use memory_bus::MemoryBus;
use registers::{Reg16Kind, Reg8Kind, Registers};

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

struct Clock {
    m: u8,
    t: u8,
}

pub struct CPU {
    registers: Registers,
    pc: Pc,
    sp: u16,
    bus: MemoryBus,
    state: State,
    clock: Clock,
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
            clock: Clock { m: 0, t: 0 },
        }
    }

    fn step(&mut self) {
        let instruction = self.bus.read_byte(self.pc.get());

        let instruction = if instruction == 0xCB {
            let instruction = 0xCB00 | self.bus.read_byte(self.pc.peek()) as u16;

            Instr::from(instruction)

        } else {
            Instr::from(instruction)
        };

        let i = self.execute(instruction);

        println!("{}", &i.unwrap());
    }

    fn execute(&mut self, instr: Instr) -> Option<Instr> {
        use executors::*;

        if self.state == State::Halted {
            return None;
        }

        match instr.id {
            InstrKind::Nop => {
                self.pc.add(1);

                Some(instr)
            },

            InstrKind::Halt => {
                self.state = State::Halted;

                Some(instr)
            }

            InstrKind::Add => {
                add(self, instr)
            }

            InstrKind::Adc => {
                adc(self, instr)
            }

            InstrKind::AddHl => {
                add_hl(self, instr)
            }

            InstrKind::Sub => {
                sub(self, instr)
            }

            InstrKind::Sbc => {
                sbc(self, instr)
            }

            InstrKind::And => {
                and(self, instr)
            }

            InstrKind::Or => {
                or(self, instr)
            }

            InstrKind::Xor => {
                xor(self, instr)
            }

            InstrKind::Jp => {
                jp(self, instr)
            }

            InstrKind::Jr => {
                jr(self, instr)
            }

            InstrKind::Push => {
                push(self, instr)
            }

            InstrKind::Pop => {
                pop(self, instr)
            }

            InstrKind::Call => {
                call(self, instr)
            }

            InstrKind::Ret => {
                ret(self, instr)
            }

            InstrKind::Ld => {
                ld(self, instr)
            }

            InstrKind::Cp => {
                cp(self, instr)
            }

            InstrKind::LdWord => {
                let word = match instr.rhs {
                    Some(Operand::U16) => self.read_next_word(),
                    Some(Operand::Reg16(Reg16Kind::HL)) => self.registers.get_hl(),
                    _ => {
                        panic!("{}: Mismatched operand {:?}", instr, instr.rhs)
                    }
                };

                match instr.lhs {
                    Some(Operand::Reg16(Reg16Kind::SP)) => self.sp = word,
                    Some(Operand::Reg16(Reg16Kind::BC)) => self.registers.set_bc(word),
                    Some(Operand::Reg16(Reg16Kind::DE)) => self.registers.set_de(word),
                    Some(Operand::Reg16(Reg16Kind::HL)) => self.registers.set_hl(word),
                    _ => {
                        panic!("{}: Mismatched operand {:?}", instr, instr.rhs)
                    }
                };

                self.pc.add(3);

                Some(instr)
            }

            InstrKind::Inc => {
                match instr.rhs {
                    Some(Operand::Reg8(kind)) => {
                        let orig_val = self.registers.get_8(&kind);
                        let val = orig_val.wrapping_add(1);

                        self.registers.set_8(&kind, val);

                        self.registers.f.zero = val == 0;
                        self.registers.f.subtract = false;
                        self.registers.f.half_carry = orig_val & 0xF == 0xF;
                    }

                    Some(Operand::Reg16(kind)) => {
                        let val = self.registers.get_16(&kind).wrapping_add(1);

                        // println!("{}", instr);
                        // panic!();
                        self.registers.set_16(&kind, val);
                    }

                    Some(Operand::Reg16Indir(kind)) => {
                        let addr = self.registers.get_16(&kind);

                        let orig_val = self.bus.read_byte(addr);
                        let val = orig_val.wrapping_add(1);

                        self.bus.write_byte(addr, val);

                        self.registers.f.zero = val == 0;
                        self.registers.f.subtract = false;
                        self.registers.f.half_carry = orig_val & 0xF == 0xF;
                    }

                    _ => {
                        panic!("{}: Mismatched operand {:?}", instr, instr.rhs)
                    }
                };

                self.pc.add(1);

                Some(instr)
            },

            InstrKind::Dec => {
                match instr.rhs {
                    Some(Operand::Reg8(kind)) => {
                        let orig_val = self.registers.get_8(&kind);
                        let val = orig_val.wrapping_sub(1);

                        self.registers.set_8(&kind, val);

                        self.registers.f.zero = val == 0;
                        self.registers.f.subtract = true;
                        self.registers.f.half_carry = orig_val & 0xF == 0x0;
                    }

                    Some(Operand::Reg16(kind)) => {
                        let val = self.registers.get_16(&kind).wrapping_sub(1);

                        self.registers.set_16(&kind, val);
                    }

                    Some(Operand::Reg16Indir(kind)) => {
                        let addr = self.registers.get_16(&kind);

                        let orig_val = self.bus.read_byte(addr);
                        let val = orig_val.wrapping_sub(1);

                        self.bus.write_byte(addr, val);

                        self.registers.f.zero = val == 0;
                        self.registers.f.subtract = true;
                        self.registers.f.half_carry = orig_val & 0xF == 0x0;
                    }

                    _ => {
                        panic!("{}: Mismatched operand {:?}", instr, instr.rhs)
                    }
                };

                self.pc.add(1);

                Some(instr)
            },

            InstrKind::RotA => {
                let mut val = self.registers.a;

                match instr.rhs {
                    Some(Operand::RotLeft) => {
                        self.registers.f.carry = (val & 0x80) == 0x80;

                        val = val << 1;
                    },

                    _ => {
                        panic!("{}: Mismatched operand {:?}", instr, instr.rhs)
                    }
                }

                match instr.post_op {
                    Some(instruction::PostOp::CarryToB0) => {
                        let carry = if self.registers.f.carry { 1 } else { 0 };

                        val = val | carry;
                    },

                    _ => panic!("{}: unsupported post_op {:?}", instr, instr.post_op),
                }

                self.registers.a = val;

                self.registers.f.zero = false;
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;

                self.pc.add(1);

                Some(instr)
            },

            InstrKind::Rot => {
                let lhs = instr.lhs.unwrap();
                let lhs = match lhs {
                    Operand::Reg8(r) => r,
                    _ => panic!(),
                };

                let mut val = self.registers.get_8(&lhs);

                match instr.rhs {
                    Some(Operand::RotLeft) => {
                        val = val << 1;
                    },

                    _ => {
                        panic!("{}: Mismatched operand {:?}", instr, instr.rhs)
                    }
                }

                match instr.post_op {
                    Some(instruction::PostOp::CarryToB0) => {
                        let carry = if self.registers.f.carry { 1 } else { 0 };

                        val = val | carry;
                    },

                    _ => panic!("{}: unsupported post_op {:?}", instr, instr.post_op),
                }

                self.registers.set_8(&lhs, val);

                // TODO: Flags

                self.pc.add(2);

                Some(instr)
            },

            InstrKind::Bit => {
                bit(self, instr)
            },

            _ => {
                panic!("{}: UNIMPLEMENTED", instr);
            }
        }
    }

    fn read_at_reg_16(&self, reg: &Reg16Kind) -> u8 {
        let addr = self.registers.get_16(reg);

        self.bus.read_byte(addr)
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
    let mut window = Window::new("Game On", 160, 144, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    window.limit_update_rate(Some(std::time::Duration::from_micros(64400)));

    let boot_rom_buffer = buffer_from_file("b_rom.gb");
    let game_rom_buffer = buffer_from_file("tetris_rom.gb");

    let mut cpu = CPU::new(boot_rom_buffer, game_rom_buffer, None);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        cpu.step();
    }
}
