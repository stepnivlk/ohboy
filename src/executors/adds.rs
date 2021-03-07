use crate::{
    executors::{op_to_u16_reg, op_to_u8_reg, ExecRes, Executor},
    instruction::Instr,
    CPU,
};

pub struct Add<'a>(pub &'a mut CPU);

impl<'a> Add<'a> {
    fn set_flags(&mut self, val: u8, carry: bool) {
        self.0.registers.f.zero = val == 0;
        self.0.registers.f.subtract = false;
        // TODO: Incorrect
        self.0.registers.f.half_carry = val > 0xF;
        self.0.registers.f.carry = carry;
    }
}

impl<'a> Executor for Add<'a> {
    fn run(&mut self, instr: Instr) -> ExecRes {
        let val = op_to_u8_reg(&instr.rhs.unwrap(), &self.0.registers);
        let (new_val, carry) = self.0.registers.a.overflowing_add(val);

        self.set_flags(val, carry);

        self.0.registers.a = new_val;

        // TODO:
        ExecRes {
            ticks: 4,
            length: 1,
            instr,
            trace: None,
        }
    }
}

pub fn add(cpu: &mut CPU, instr: Instr) -> Option<Instr> {
    let val = op_to_u8_reg(&instr.rhs?, &cpu.registers);
    let (new_val, carry) = cpu.registers.a.overflowing_add(val);

    cpu.registers.f.zero = val == 0;
    cpu.registers.f.subtract = false;
    // TODO: Incorrect
    cpu.registers.f.half_carry = val > 0xF;
    cpu.registers.f.carry = carry;

    cpu.registers.a = new_val;
    cpu.pc.add(1);

    Some(instr)
}

pub fn adc(cpu: &mut CPU, instr: Instr) -> Option<Instr> {
    let val = op_to_u8_reg(&instr.rhs?, &cpu.registers);
    let additinal_carry = if cpu.registers.f.carry { 1 } else { 0 };
    let (mid_value, mid_carry) = cpu.registers.a.overflowing_add(val);
    let (new_value, carry) = mid_value.overflowing_add(additinal_carry);

    cpu.registers.f.zero = new_value == 0;
    cpu.registers.f.subtract = false;
    // TODO: Incorrect
    cpu.registers.f.half_carry = new_value > 0xF;
    cpu.registers.f.carry = mid_carry || carry;

    cpu.registers.a = new_value;
    cpu.pc.add(1);

    Some(instr)
}

pub fn add_hl(cpu: &mut CPU, instr: Instr) -> Option<Instr> {
    let val = op_to_u16_reg(&instr.rhs?, &cpu.registers);
    let curr_hl = cpu.registers.get_hl();
    let (new_value, carry) = curr_hl.overflowing_add(val);

    cpu.registers.f.carry = carry;
    cpu.registers.f.subtract = false;
    // TODO: Incorrect
    cpu.registers.f.half_carry = val > 0xFF;

    cpu.registers.set_hl(new_value);
    cpu.pc.add(1);

    Some(instr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Registers, CPU};

    fn cpu(registers: Registers) -> CPU {
        CPU::new(vec![], vec![], Some(registers))
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
