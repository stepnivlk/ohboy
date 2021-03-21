mod adds;
mod and;
mod bit;
mod call;
mod dec;
mod inc;
mod jumps;
mod ld;
mod ld_word;
mod or;
mod rot;
mod stack;
mod subs;
mod xor;

use crate::{
    instr::{CondKind, Instr, Operand},
    registers::{FlagsRegister, Reg16Kind, Reg8Kind, Registers},
    Cpu,
};

pub use adds::{Adc, Add, AddHl};
pub use and::And;
pub use bit::Bit;
pub use call::{Call, Ret};
pub use dec::Dec;
pub use inc::Inc;
pub use jumps::{Jp, Jr};
pub use ld::Ld;
pub use ld_word::LdWord;
pub use or::Or;
pub use rot::{Rot, RotA};
pub use stack::{Pop, Push};
pub use subs::{Cp, Sbc, Sub};
pub use xor::Xor;

pub trait Exec {
    type FlagsData;

    fn run(&mut self, instr: Instr) -> Option<ExecRes>;

    fn res(&self, ticks: u8, length: u16, instr: Instr) -> ExecRes {
        ExecRes {
            ticks,
            length,
            instr,
            trace: None,
        }
    }

    fn next_flags(&self, _data: Self::FlagsData) -> Option<FlagsRegister> {
        None
    }
}

pub struct ExecRes {
    pub ticks: u8,
    pub length: u16,
    pub instr: Instr,
    pub trace: Option<(u16, u16)>,
}

pub fn should_jump(cpu: &Cpu, op: Operand) -> bool {
    use Operand::*;

    match op {
        Cond(CondKind::Always) => true,
        Cond(CondKind::NotCarry) => !cpu.registers.f.carry,
        Cond(CondKind::NotZero) => !cpu.registers.f.zero,
        Cond(CondKind::Carry) => cpu.registers.f.carry,
        Cond(CondKind::Zero) => cpu.registers.f.zero,
        _ => panic!("Mismatched operand {:?}", op),
    }
}

pub fn op_to_u8_reg(op: &Operand, registers: &Registers) -> u8 {
    use Operand::*;

    match op {
        Reg8(Reg8Kind::A) => registers.a,
        Reg8(Reg8Kind::B) => registers.b,
        Reg8(Reg8Kind::C) => registers.c,
        Reg8(Reg8Kind::D) => registers.d,
        Reg8(Reg8Kind::E) => registers.e,
        Reg8(Reg8Kind::H) => registers.h,
        Reg8(Reg8Kind::L) => registers.l,
        _ => panic!("Unsupported operand: {:?}", op),
    }
}

pub fn op_to_u16_reg(op: &Operand, registers: &Registers) -> u16 {
    use Operand::*;

    match op {
        Reg16(Reg16Kind::SP) => {
            panic!("not implemented");
        }
        Reg16(Reg16Kind::BC) => registers.get_bc(),
        Reg16(Reg16Kind::DE) => registers.get_de(),
        Reg16(Reg16Kind::HL) => registers.get_hl(),
        _ => panic!("Unsupported operand: {:?}", op),
    }
}

pub fn op_to_u16_reg_w(op: &Operand, registers: &mut Registers, val: u16) {
    use Operand::*;

    match op {
        Reg16(Reg16Kind::SP) => {
            panic!("not implemented");
        }
        Reg16(Reg16Kind::BC) => registers.set_bc(val),
        Reg16(Reg16Kind::DE) => registers.set_de(val),
        Reg16(Reg16Kind::HL) => registers.set_hl(val),
        _ => panic!("Unsupported operand: {:?}", op),
    }
}
