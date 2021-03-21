use crate::{
    instr::Instr,
    microcode::{op_to_u16_reg, op_to_u8_reg, Exec, ExecRes},
    registers::FlagsRegister,
    Cpu,
};

type FlagsData = (u8, bool);

fn next_flags(data: FlagsData) -> Option<FlagsRegister> {
    Some(FlagsRegister {
        zero: data.0 == 0,
        subtract: false,
        // TODO: Incorrect
        half_carry: data.0 > 0xF,
        carry: data.1,
    })
}

pub struct Add<'a>(pub &'a mut Cpu);

impl Exec for Add<'_> {
    type FlagsData = FlagsData;

    fn run(&mut self, instr: Instr) -> Option<ExecRes> {
        let val = op_to_u8_reg(&instr.rhs.unwrap(), &self.0.registers);
        let (new_val, carry) = self.0.registers.a.overflowing_add(val);

        self.next_flags((new_val, carry))
            .map(|f| self.0.registers.f = f);

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
        next_flags(data)
    }
}

pub struct Adc<'a>(pub &'a mut Cpu);

impl Exec for Adc<'_> {
    type FlagsData = FlagsData;

    fn run(&mut self, instr: Instr) -> Option<ExecRes> {
        let cpu = &self.0;

        let val = op_to_u8_reg(&instr.rhs.unwrap(), &cpu.registers);
        let additinal_carry = if cpu.registers.f.carry { 1 } else { 0 };
        let (mid_value, mid_carry) = cpu.registers.a.overflowing_add(val);
        let (new_val, carry) = mid_value.overflowing_add(additinal_carry);

        self.next_flags((new_val, mid_carry || carry))
            .map(|f| self.0.registers.f = f);

        self.0.registers.a = new_val;

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
        next_flags(data)
    }
}

pub struct AddHl<'a>(pub &'a mut Cpu);

impl Exec for AddHl<'_> {
    type FlagsData = (FlagsRegister, u16, bool);

    fn run(&mut self, instr: Instr) -> Option<ExecRes> {
        let val = op_to_u16_reg(&instr.rhs.unwrap(), &self.0.registers);
        let curr_hl = self.0.registers.get_hl();
        let (new_value, carry) = curr_hl.overflowing_add(val);

        self.0.registers.set_hl(new_value);
        self.next_flags((self.0.registers.f, new_value, carry))
            .map(|f| self.0.registers.f = f);

        self.0.pc.add(1);
        self.0.clock.add(8);

        Some(ExecRes {
            ticks: 8,
            length: 1,
            instr,
            trace: None,
        })
    }

    fn next_flags(&self, data: Self::FlagsData) -> Option<FlagsRegister> {
        Some(FlagsRegister {
            zero: data.0.zero,
            carry: data.2,
            subtract: false,
            // TODO: Incorrect
            half_carry: data.1 > 0xFF,
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
    fn add_increments_pc() {
        let mut registers = Registers::new();
        registers.a = 0x00;

        let mut cpu = cpu(registers);

        assert_eq!(cpu.pc.get(), 0);

        add(&mut cpu, Reg8Kind::B);

        assert_eq!(cpu.pc.get(), 1);
    }

    #[test]
    fn add_adds_value_from_target() {
        let mut registers = Registers::new();
        registers.a = 0x01;
        registers.c = 0x02;

        let mut cpu = cpu(registers);

        add(&mut cpu, Reg8Kind::C);

        assert_eq!(cpu.registers.a, 0x03);

        assert!(!cpu.registers.f.zero);
        assert!(!cpu.registers.f.carry);
        assert!(!cpu.registers.f.half_carry);
        assert!(!cpu.registers.f.subtract);
    }

    #[test]
    fn add_adds_value_from_a() {
        let mut registers = Registers::new();
        registers.a = 0x02;

        let mut cpu = cpu(registers);

        add(&mut cpu, Reg8Kind::A);

        assert_eq!(cpu.registers.a, 0x04);
    }

    #[test]
    fn add_adds_with_carry() {
        let mut registers = Registers::new();
        registers.a = 0b1111_1111;
        registers.d = 0b1;

        let mut cpu = cpu(registers);

        add(&mut cpu, Reg8Kind::D);

        assert_eq!(cpu.registers.a, 0);

        assert!(cpu.registers.f.carry);
        assert!(cpu.registers.f.zero);
    }

    #[test]
    fn add_adds_with_half_carry() {
        let mut registers = Registers::new();
        registers.a = 0b0000_1111;
        registers.e = 0b1;

        let mut cpu = cpu(registers);

        add(&mut cpu, Reg8Kind::E);

        assert_eq!(cpu.registers.a, 0b0001_0000);

        assert!(cpu.registers.f.half_carry);
    }

    #[test]
    fn adc_increments_pc() {
        let mut registers = Registers::new();
        registers.a = 0x00;
        registers.b = 0x00;

        let mut cpu = cpu(registers);

        assert_eq!(cpu.pc.get(), 0);

        adc(&mut cpu, Reg8Kind::B);

        assert_eq!(cpu.pc.get(), 1);
    }

    #[test]
    fn adc_without_carry_adds_value() {
        let mut registers = Registers::new();
        registers.a = 0x01;
        registers.b = 0x02;

        let mut cpu = cpu(registers);

        adc(&mut cpu, Reg8Kind::B);

        assert_eq!(cpu.registers.a, 0x03);

        assert!(!cpu.registers.f.zero);
        assert!(!cpu.registers.f.carry);
        assert!(!cpu.registers.f.half_carry);
        assert!(!cpu.registers.f.subtract);
    }

    #[test]
    fn adc_adds_with_half_carry() {
        let mut registers = Registers::new();
        registers.a = 0b0000_1111;
        registers.e = 0b1;

        let mut cpu = cpu(registers);

        adc(&mut cpu, Reg8Kind::E);

        assert_eq!(cpu.registers.a, 0b0001_0000);

        assert!(cpu.registers.f.half_carry);
    }

    #[test]
    fn adc_adds_with_carry() {
        let mut registers = Registers::new();
        registers.a = 0b1111_1111;
        registers.d = 0b1;

        let mut cpu = cpu(registers);

        adc(&mut cpu, Reg8Kind::D);

        assert_eq!(cpu.registers.a, 0);

        assert!(cpu.registers.f.carry);
        assert!(cpu.registers.f.zero);
    }

    #[test]
    fn adc_adds_with_value_of_carry() {
        let mut registers = Registers::new();
        registers.a = 0b1111_1111;
        // overflows to 0
        registers.d = 0b1;
        // 0 + 1
        registers.f.carry = true;

        let mut cpu = cpu(registers);

        adc(&mut cpu, Reg8Kind::D);

        assert_eq!(cpu.registers.a, 1);

        assert!(cpu.registers.f.carry);
    }

    #[test]
    fn add_hl_increments_pc() {
        let mut registers = Registers::new();
        registers.set_hl(0x00);
        registers.set_bc(0x00);

        let mut cpu = cpu(registers);

        assert_eq!(cpu.pc.get(), 0);

        add_hl(&mut cpu, Reg16Kind::BC);

        assert_eq!(cpu.pc.get(), 1);
    }

    #[test]
    fn add_hl_adds_value_from_target() {
        let mut registers = Registers::new();
        registers.set_hl(0x00_0A);
        registers.set_de(0x00_0B);

        let mut cpu = cpu(registers);

        add_hl(&mut cpu, Reg16Kind::DE);

        assert_eq!(cpu.registers.get_hl(), 0x15);

        assert!(!cpu.registers.f.zero);
        assert!(!cpu.registers.f.carry);
        assert!(!cpu.registers.f.half_carry);
        assert!(!cpu.registers.f.subtract);
    }

    #[test]
    fn add_hl_adds_with_carry() {
        let mut registers = Registers::new();
        registers.set_hl(0xFF_FF);
        registers.set_bc(0x00_01);

        let mut cpu = cpu(registers);

        add_hl(&mut cpu, Reg16Kind::BC);

        assert_eq!(cpu.registers.get_hl(), 0);

        assert!(cpu.registers.f.carry);
    }

    #[test]
    fn add_hl_adds_with_half_carry() {
        let mut registers = Registers::new();
        registers.set_hl(0x00_FF);
        registers.set_bc(0x00_01);

        let mut cpu = cpu(registers);

        add_hl(&mut cpu, Reg16Kind::BC);

        assert_eq!(cpu.registers.get_hl(), 0b0000_0001_0000_0000);

        assert!(cpu.registers.f.half_carry);
    }
}
