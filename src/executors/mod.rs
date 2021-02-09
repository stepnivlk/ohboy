mod adds;
mod and;
mod call;
mod jumps;
mod lds;
mod or;
mod stack;
mod subs;
mod xor;
mod bit;

use crate::{
    instruction::{CondKind, Operand},
    registers::{Reg16Kind, Reg8Kind, Registers},
    CPU,
};

pub use adds::{adc, add, add_hl};
pub use and::and;
pub use call::{call, ret};
pub use jumps::{jp, jr};
pub use lds::ld;
pub use or::or;
pub use stack::{pop, push};
pub use subs::{sbc, sub};
pub use xor::xor;
pub use bit::bit;

pub fn should_jump(cpu: &CPU, op: Operand) -> bool {
    match op {
        Operand::Cond(CondKind::Always) => true,
        Operand::Cond(CondKind::NotCarry) => !cpu.registers.f.carry,
        Operand::Cond(CondKind::NotZero) => !cpu.registers.f.zero,
        Operand::Cond(CondKind::Carry) => cpu.registers.f.carry,
        Operand::Cond(CondKind::Zero) => cpu.registers.f.zero,
        _ => panic!("Mismatched operand {:?}", op),
    }
}

pub fn op_to_u8_reg(op: &Operand, registers: &Registers) -> u8 {
    match op {
        Operand::Reg8(Reg8Kind::A) => registers.a,
        Operand::Reg8(Reg8Kind::B) => registers.b,
        Operand::Reg8(Reg8Kind::C) => registers.c,
        Operand::Reg8(Reg8Kind::D) => registers.d,
        Operand::Reg8(Reg8Kind::E) => registers.e,
        Operand::Reg8(Reg8Kind::H) => registers.h,
        Operand::Reg8(Reg8Kind::L) => registers.l,
        _ => panic!("Unsupported operand: {:?}", op),
    }
}

pub fn op_to_u16_reg(op: &Operand, registers: &Registers) -> u16 {
    match op {
        Operand::Reg16(Reg16Kind::SP) => {
            panic!("not implemented");
        }
        Operand::Reg16(Reg16Kind::BC) => registers.get_bc(),
        Operand::Reg16(Reg16Kind::DE) => registers.get_de(),
        Operand::Reg16(Reg16Kind::HL) => registers.get_hl(),
        _ => panic!("Unsupported operand: {:?}", op),
    }
}

pub fn op_to_u16_reg_w(op: &Operand, registers: &mut Registers, val: u16) {
    match op {
        Operand::Reg16(Reg16Kind::SP) => {
            panic!("not implemented");
        }
        Operand::Reg16(Reg16Kind::BC) => registers.set_bc(val),
        Operand::Reg16(Reg16Kind::DE) => registers.set_de(val),
        Operand::Reg16(Reg16Kind::HL) => registers.set_hl(val),
        _ => panic!("Unsupported operand: {:?}", op),
    }
}
