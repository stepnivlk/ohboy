use crate::{
    executors::{op_to_u8_reg, ExecRes, Executor},
    instruction::Instr,
    CPU,
};

pub struct And<'a>(pub &'a mut CPU);

impl<'a> And<'a> {
    fn set_flags(&mut self, new_val: u8) {
        self.0.registers.f.zero = new_val == 0;
        self.0.registers.f.subtract = false;
        self.0.registers.f.half_carry = true;
        self.0.registers.f.carry = false;
    }
}

impl<'a> Executor for And<'a> {
    fn run(&mut self, instr: Instr) -> ExecRes {
        let val = op_to_u8_reg(&instr.rhs.unwrap(), &self.0.registers);

        let new_val = self.0.registers.a & val;

        self.set_flags(new_val);

        self.0.registers.a = new_val;

        ExecRes {
            ticks: 4,
            length: 1,
            instr,
            trace: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Registers, CPU};

    fn cpu(registers: Registers) -> CPU {
        CPU::new(vec![], vec![], Some(registers))
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
