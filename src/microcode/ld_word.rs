use crate::{
    instruction::{Instr, Operand},
    microcode::{Exec, ExecRes},
    registers::Reg16Kind,
    CPU,
};

pub struct LdWord<'a>(pub &'a mut CPU);

impl Exec for LdWord<'_> {
    type FlagsData = ();

    fn run(&mut self, instr: Instr) -> Option<ExecRes> {
        let word = match instr.rhs {
            Some(Operand::U16) => self.0.read_next_word(),
            Some(Operand::Reg16(Reg16Kind::HL)) => self.0.registers.get_hl(),
            _ => {
                panic!("{}: Mismatched operand {:?}", instr, instr.rhs)
            }
        };

        match instr.lhs {
            Some(Operand::Reg16(Reg16Kind::SP)) => self.0.sp = word,
            Some(Operand::Reg16(Reg16Kind::BC)) => {
                self.0.registers.set_bc(word)
            }
            Some(Operand::Reg16(Reg16Kind::DE)) => {
                self.0.registers.set_de(word)
            }
            Some(Operand::Reg16(Reg16Kind::HL)) => {
                self.0.registers.set_hl(word)
            }
            _ => {
                panic!("{}: Mismatched operand {:?}", instr, instr.rhs)
            }
        };

        self.0.pc.add(3);
        self.0.clock.add(12);

        Some(ExecRes {
            ticks: 12,
            length: 3,
            instr,
            trace: None,
        })
    }
}
