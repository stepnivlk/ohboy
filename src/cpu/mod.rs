use crate::{
    instr::{Instr, InstrKind},
    microcode,
    mmu::Mmu,
    registers::{Reg16Kind, Registers},
};

pub struct Pc(u16);

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

pub struct Clock(u8);

impl Clock {
    pub fn add(&mut self, val: u8) {
        self.0 = self.0.wrapping_add(val);
    }
}

pub struct Cpu {
    pub registers: Registers,
    pub pc: Pc,
    pub sp: u16,
    pub bus: Mmu,
    pub clock: Clock,
    state: State,
}

impl Cpu {
    pub fn new(
        boot_rom_buffer: Vec<u8>,
        game_rom_buffer: Vec<u8>,
        registers: Option<Registers>,
    ) -> Self {
        Self {
            registers: registers.unwrap_or(Registers::new()),
            pc: Pc::new(),
            sp: 0,
            bus: Mmu::new(boot_rom_buffer, game_rom_buffer),
            state: State::Running,
            clock: Clock(0),
        }
    }

    pub fn step(&mut self) {
        let instruction = self.bus.read_byte(self.pc.get());

        let instruction = if instruction == 0xCB {
            let instruction =
                0xCB00 | self.bus.read_byte(self.pc.peek()) as u16;

            Instr::from(instruction)
        } else {
            Instr::from(instruction)
        };

        let res = self.execute(instruction);

        res.map(|r| {
            println!("{}", r.instr);
        });
    }

    fn execute(&mut self, instr: Instr) -> Option<microcode::ExecRes> {
        use microcode::*;

        if self.state == State::Halted {
            return None;
        }

        match instr.id {
            InstrKind::And => And(self).run(instr),
            InstrKind::Add => Add(self).run(instr),

            InstrKind::Adc => Adc(self).run(instr),

            InstrKind::Nop => {
                self.pc.add(1);
                self.clock.add(4);

                Some(ExecRes {
                    ticks: 4,
                    length: 1,
                    instr,
                    trace: None,
                })
            }

            InstrKind::Halt => {
                self.state = State::Halted;

                Some(ExecRes {
                    ticks: 4,
                    length: 1,
                    instr,
                    trace: None,
                })
            }

            InstrKind::AddHl => AddHl(self).run(instr),

            InstrKind::Sub => Sub(self).run(instr),

            InstrKind::Sbc => Sbc(self).run(instr),

            InstrKind::Or => Or(self).run(instr),

            InstrKind::Xor => Xor(self).run(instr),

            InstrKind::Jp => Jp(self).run(instr),

            InstrKind::Jr => Jr(self).run(instr),

            InstrKind::Push => Push(self).run(instr),

            InstrKind::Pop => Pop(self).run(instr),

            InstrKind::Call => Call(self).run(instr),

            InstrKind::Ret => Ret(self).run(instr),

            InstrKind::Ld => Ld(self).run(instr),

            InstrKind::Cp => Cp(self).run(instr),

            InstrKind::LdWord => LdWord(self).run(instr),

            InstrKind::Inc => Inc(self).run(instr),

            InstrKind::Dec => Dec(self).run(instr),

            InstrKind::RotA => RotA(self).run(instr),

            InstrKind::Rot => Rot(self).run(instr),

            InstrKind::Bit => Bit(self).run(instr),

            _ => {
                panic!("{}: UNIMPLEMENTED", instr);
            }
        }
    }

    pub fn read_at_reg_16(&self, reg: &Reg16Kind) -> u8 {
        let addr = self.registers.get_16(reg);

        self.bus.read_byte(addr)
    }

    pub fn read_next_byte(&self) -> u8 {
        self.bus.read_byte(self.pc.peek())
    }

    pub fn read_next_word(&self) -> u16 {
        let lo = self.bus.read_byte(self.pc.get() + 1) as u16;
        let hi = self.bus.read_byte(self.pc.get() + 2) as u16;

        (hi << 8) | lo
    }
}
