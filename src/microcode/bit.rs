use crate::{
    instruction::{Instr, Operand},
    microcode::{op_to_u8_reg, Exec, ExecRes},
    registers::FlagsRegister,
    CPU,
};

pub struct Bit<'a>(pub &'a mut CPU);

impl Bit<'_> {
    fn bit_position(&self, instr: &Instr) -> u8 {
        match instr.lhs {
            Some(Operand::BitPos(n)) => n,
            _ => {
                panic!("{}: Mismatched operand {:?}", instr, instr.lhs)
            }
        }
    }

    fn val(&self, instr: &Instr) -> u8 {
        match instr.rhs {
            Some(op @ Operand::Reg8(_)) => op_to_u8_reg(&op, &self.0.registers),
            Some(Operand::Reg16Indir(reg)) => self.0.read_at_reg_16(&reg),

            _ => panic!("Mismatched operand {:?}", instr.rhs),
        }
    }
}

impl Exec for Bit<'_> {
    type FlagsData = (FlagsRegister, u8);

    fn run(&mut self, instr: Instr) -> Option<ExecRes> {
        let bit_position = self.bit_position(&instr);
        let val = self.val(&instr);

        let bit = (val >> bit_position) & 0b1;

        self.next_flags((self.0.registers.f, bit))
            .map(|f| self.0.registers.f = f);

        self.0.pc.add(2);
        self.0.clock.add(8);

        Some(ExecRes {
            ticks: 8,
            length: 2,
            instr,
            trace: Some((bit as u16, val as u16)),
        })
    }

    fn next_flags(&self, data: Self::FlagsData) -> Option<FlagsRegister> {
        Some(FlagsRegister {
            zero: data.1 == 0,
            subtract: false,
            half_carry: false,
            carry: data.0.carry,
        })
    }
}
