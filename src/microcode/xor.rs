use crate::{
    instr::Instr,
    microcode::{op_to_u8_reg, Exec, ExecRes},
    Cpu,
};

pub struct Xor<'a>(pub &'a mut Cpu);

impl Exec for Xor<'_> {
    type FlagsData = ();

    fn run(&mut self, instr: Instr) -> Option<ExecRes> {
        let cpu = &mut self.0;

        let val = op_to_u8_reg(&instr.rhs?, &cpu.registers);

        let new_value = cpu.registers.a ^ val;

        cpu.registers.f.zero = new_value == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.half_carry = false;
        cpu.registers.f.carry = false;

        cpu.registers.a = new_value;
        cpu.pc.add(1);
        cpu.clock.add(4);

        Some(ExecRes {
            ticks: 4,
            length: 1,
            instr,
            trace: None,
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
    fn it_sets_a_register_correctly() {
        let mut registers = Registers::new();
        registers.a = 0b1000_1111;
        registers.b = 0b1010_1001;

        let mut cpu = cpu(registers);

        xor(&mut cpu, Reg8Kind::B);

        assert_eq!(cpu.registers.a, 0b0010_0110);

        assert_eq!(cpu.registers.f.zero, false);
    }

    #[test]
    fn it_sets_flags() {
        let mut registers = Registers::new();
        registers.a = 0x12;
        registers.d = 0x13;

        let mut cpu = cpu(registers);

        xor(&mut cpu, Reg8Kind::D);

        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);
    }

    #[test]
    fn it_sets_zero_flag_when_result_is_zero() {
        let mut registers = Registers::new();
        registers.a = 0x00;
        registers.c = 0x00;

        let mut cpu = cpu(registers);

        xor(&mut cpu, Reg8Kind::C);

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

        xor(&mut cpu, Reg8Kind::E);

        assert_eq!(cpu.pc.get(), 1);
    }
}
