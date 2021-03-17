use crate::{
    instruction::Instr,
    microcode::{op_to_u8_reg, Exec, ExecRes},
    registers::FlagsRegister,
    CPU,
};

pub struct Or<'a>(pub &'a mut CPU);

impl Exec for Or<'_> {
    type FlagsData = u8;

    fn run(&mut self, instr: Instr) -> Option<ExecRes> {
        let cpu = &mut self.0;

        let val = op_to_u8_reg(&instr.rhs?, &cpu.registers);

        let next_val = cpu.registers.a | val;

        cpu.registers.a = next_val;

        cpu.pc.add(1);
        cpu.clock.add(4);

        None
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
    use crate::{Registers, CPU};

    fn cpu(registers: Registers) -> CPU {
        CPU::new(vec![], vec![], Some(registers))
    }

    #[test]
    fn it_sets_a_register_correctly() {
        let mut registers = Registers::new();
        registers.a = 0b1000_1111;
        registers.b = 0b1010_1001;

        let mut cpu = cpu(registers);

        or(&mut cpu, Reg8Kind::B);

        assert_eq!(cpu.registers.a, 0b1010_1111);

        assert_eq!(cpu.registers.f.zero, false);
    }

    #[test]
    fn it_sets_flags() {
        let mut registers = Registers::new();
        registers.a = 0x12;
        registers.d = 0x13;

        let mut cpu = cpu(registers);

        or(&mut cpu, Reg8Kind::D);

        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, true);
        assert_eq!(cpu.registers.f.carry, false);
    }

    #[test]
    fn it_sets_zero_flag_when_result_is_zero() {
        let mut registers = Registers::new();
        registers.a = 0x00;
        registers.c = 0x00;

        let mut cpu = cpu(registers);

        or(&mut cpu, Reg8Kind::C);

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

        or(&mut cpu, Reg8Kind::E);

        assert_eq!(cpu.pc.get(), 1);
    }
}
