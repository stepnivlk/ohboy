use crate::{
    instruction::{Instr, Operand, PostOp},
    microcode::{Exec, ExecRes},
    registers::{Reg16Kind, Reg8Kind},
    CPU,
};

pub struct Ld<'a>(pub &'a mut CPU);

impl Ld<'_> {
    fn rhs(&self, instr: &Instr) -> u8 {
        use Reg8Kind::*;

        let cpu = &self.0;

        match instr.rhs {
            Some(Operand::Reg8(reg)) => match reg {
                A => cpu.registers.a,
                B => cpu.registers.b,
                C => cpu.registers.c,
                D => cpu.registers.d,
                E => cpu.registers.e,
                H => cpu.registers.h,
                L => cpu.registers.l,
            },
            Some(Operand::U8Indir(offset)) => {
                let addr = offset + (cpu.read_next_byte() as u16);

                cpu.bus.read_byte(addr)
            }
            Some(Operand::Reg16Indir(reg)) => cpu.read_at_reg_16(&reg),
            Some(Operand::U8) => cpu.read_next_byte(),
            _ => panic!(
                "[{:X} | {}] unsupported operand {:?}",
                instr.pos, instr.tag, instr.rhs
            ),
        }
    }

    fn lhs(&mut self, instr: &Instr, rhs: u8) -> u16 {
        use Reg8Kind::*;

        let cpu = &mut self.0;

        match &instr.lhs {
            Some(Operand::Reg8(reg)) => {
                match reg {
                    A => cpu.registers.a = rhs,
                    B => cpu.registers.b = rhs,
                    C => cpu.registers.c = rhs,
                    D => cpu.registers.d = rhs,
                    E => cpu.registers.e = rhs,
                    H => cpu.registers.h = rhs,
                    L => cpu.registers.l = rhs,
                };

                rhs as u16
            }
            Some(Operand::Reg16Indir(Reg16Kind::HL)) => {
                let addr = cpu.registers.get_hl();

                cpu.bus.write_byte(addr, rhs);

                addr
            }
            Some(Operand::Reg8Indir(Reg8Kind::C, offset)) => {
                let addr = offset + (cpu.registers.c as u16);

                cpu.bus.write_byte(addr, rhs);

                addr
            }
            Some(Operand::U8Indir(offset)) => {
                let addr = offset + (cpu.read_next_byte() as u16);

                cpu.bus.write_byte(addr, rhs);

                addr
            }
            Some(Operand::U16Indir) => {
                let addr = cpu.read_next_word();

                cpu.bus.write_byte(addr, rhs);

                addr
            }
            _ => panic!("{}: unsupported operand {:?}", instr, instr.lhs),
        }
    }
}

impl Exec for Ld<'_> {
    type FlagsData = ();

    fn run(&mut self, instr: Instr) -> Option<ExecRes> {
        let rhs = self.rhs(&instr);

        let lhs = self.lhs(&instr, rhs);

        let mut length = 1;
        let mut ticks = 8;

        match &instr.rhs {
            Some(Operand::U8Indir(_)) => {
                length += 1;
                ticks += 4;
            }
            Some(Operand::U8) => {
                length += 1;
                ticks += 4;
            }
            _ => match &instr.lhs {
                Some(Operand::U8Indir(_)) => {
                    length += 1;
                    ticks += 4;
                }
                Some(Operand::U16Indir) => {
                    length += 1;
                    ticks += 4;
                }
                _ => {}
            },
        };

        self.0.pc.add(length);
        self.0.clock.add(ticks);

        match &instr.post_op {
            Some(PostOp::Dec(Reg16Kind::HL)) => self
                .0
                .registers
                .set_hl(self.0.registers.get_hl().wrapping_sub(1)),
            Some(PostOp::Inc(Reg16Kind::HL)) => self
                .0
                .registers
                .set_hl(self.0.registers.get_hl().wrapping_add(1)),
            None => {}
            _ => panic!("{}: unsupported post_op {:?}", instr, instr.post_op),
        };

        Some(ExecRes {
            ticks,
            length,
            instr,
            trace: Some((lhs, rhs as u16)),
        })
    }
}
