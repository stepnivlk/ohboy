use crate::{
    instr::Instr,
    microcode::{op_to_u8_reg, Exec, ExecRes},
    registers::FlagsRegister,
    Cpu,
};

pub struct And<'a>(pub &'a mut Cpu);

impl Exec for And<'_> {
    type FlagsData = u8;

    fn run(&mut self, instr: Instr) -> Option<ExecRes> {
        let val = op_to_u8_reg(&instr.rhs.unwrap(), &self.0.registers);

        let new_val = self.0.registers.a & val;

        self.0.registers.a = new_val;
        self.next_flags(new_val).map(|f| self.0.registers.f = f);

        self.0.pc.add(1);
        self.0.clock.add(4);

        Some(ExecRes {
            ticks: 4,
            length: 1,
            instr,
            trace: None,
        })
    }

    fn next_flags(&self, data: Self::FlagsData) -> Option<FlagsRegister> {
        Some(FlagsRegister {
            zero: data == 0,
            subtract: false,
            half_carry: true,
            carry: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Cpu, Registers};

    fn cpu(registers: Registers) -> Cpu {
        Cpu::new(vec![], vec![], Some(registers))
    }

    #[test]
    fn it_sets_a_correctly() {
        let mut registers = Registers::new();
        registers.a = 0b10001111;
        registers.b = 0b10101001;

        let mut cpu = cpu(registers);

        and(&mut cpu, Reg8Kind::B);

        assert_eq!(cpu.registers.a, 0b10001001);

        assert_eq!(cpu.registers.f.zero, false);
    }

    #[test]
    fn it_sets_flags() {
        let mut registers = Registers::new();
        registers.a = 0x12;
        registers.d = 0x13;

        let mut cpu = cpu(registers);

        and(&mut cpu, Reg8Kind::D);

        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, true);
        assert_eq!(cpu.registers.f.carry, false);
    }

    #[test]
    fn it_sets_zero_flag_when_result_is_zero() {
        let mut registers = Registers::new();
        registers.a = 0x00;
        registers.c = 0xFF;

        let mut cpu = cpu(registers);

        and(&mut cpu, Reg8Kind::C);

        assert_eq!(cpu.registers.a, 0x00);

        assert!(cpu.registers.f.zero);
    }

    #[test]
    fn it_increments_pc() {
        let mut registers = Registers::new();
        registers.a = 0x00;
        registers.d = 0x00;

        let mut cpu = cpu(registers);

        assert_eq!(cpu.pc.get(), 0);

        and(&mut cpu, Reg8Kind::E);

        assert_eq!(cpu.pc.get(), 1);
    }
}
