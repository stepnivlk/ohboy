use crate::{
    executors::{Executor, Flagger, Worker},
    values::Value,
    ADDHLTarget, ArithmeticTarget, CPU,
    registers::{Reg8Kind, Reg16Kind},
};

type AddU8Data = (u8, bool);

struct AddFlagger;

impl Flagger for AddFlagger {
    type D = AddU8Data;

    fn run(&self, cpu: &mut CPU, data: Self::D) {
        let (value, carry) = data;

        cpu.registers.f.zero = value == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.half_carry = value > 0xF;
        cpu.registers.f.carry = carry;
    }
}

struct AddWorker;

impl Worker for AddWorker {
    type V = u8;
    type D = AddU8Data;

    fn run(&self, cpu: &mut CPU, value: Self::V) -> Self::D {
        let (new_value, carry) = cpu.registers.a.overflowing_add(value);

        cpu.registers.a = new_value;
        cpu.pc = cpu.pc.wrapping_add(1);

        (new_value, carry)
    }
}

struct AdcWorker;

impl Worker for AdcWorker {
    type V = u8;
    type D = AddU8Data;

    fn run(&self, cpu: &mut CPU, value: Self::V) -> Self::D {
        let additinal_carry = if cpu.registers.f.carry { 1 } else { 0 };
        let (mid_value, mid_carry) = cpu.registers.a.overflowing_add(value);
        let (new_value, carry) = mid_value.overflowing_add(additinal_carry);

        cpu.registers.a = new_value;
        cpu.pc = cpu.pc.wrapping_add(1);

        (new_value, mid_carry || carry)
    }
}

type AddU16Data = (u16, bool);

struct AddHlWorker;

impl Worker for AddHlWorker {
    type V = u16;
    type D = AddU16Data;

    fn run(&self, cpu: &mut CPU, value: Self::V) -> Self::D {
        let curr_hl = cpu.registers.get_hl();
        let (new_value, carry) = curr_hl.overflowing_add(value);

        cpu.registers.set_hl(new_value);
        cpu.pc = cpu.pc.wrapping_add(1);

        (new_value, carry)
    }
}

struct AddHlFlagger;

impl Flagger for AddHlFlagger {
    type D = AddU16Data;

    fn run(&self, cpu: &mut CPU, data: Self::D) {
        let (value, carry) = data;

        cpu.registers.f.carry = carry;
        cpu.registers.f.subtract = false;
        cpu.registers.f.half_carry = value > 0xFF;
    }
}

pub fn add(cpu: &mut CPU, from: Reg8Kind) {
    let val = cpu.registers.get(from);
    let (new_val, carry) = cpu.registers.a.overflowing_add(val);

    cpu.registers.f.zero = val == 0;
    cpu.registers.f.subtract = false;
    // TODO: Incorrect
    cpu.registers.f.half_carry = val > 0xF;
    cpu.registers.f.carry = carry;

    cpu.registers.a = new_val;
    cpu.pc.add(1);
}

pub fn adc(cpu: &mut CPU, from: Reg8Kind) {
    let val = cpu.registers.get(from);
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

}

pub fn add_hl(cpu: &mut CPU, from: Reg16Kind) {
    let val = cpu.registers.get_word(from);

    let curr_hl = cpu.registers.get_hl();
    let (new_value, carry) = curr_hl.overflowing_add(val);

    cpu.registers.f.carry = carry;
    cpu.registers.f.subtract = false;
    // TODO: Incorrect
    cpu.registers.f.half_carry = val > 0xFF;

    cpu.registers.set_hl(new_value);
    cpu.pc.add(1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Registers, CPU};

    #[test]
    fn add_increments_pc() {
        let mut registers = Registers::new();
        registers.a = 0x00;
        registers.b = 0x00;

        let mut cpu = CPU::new(Some(registers));

        assert_eq!(cpu.pc, 0);

        add(&mut cpu, ArithmeticTarget::B);

        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn add_adds_value_from_target() {
        let mut registers = Registers::new();
        registers.a = 0x01;
        registers.c = 0x02;

        let mut cpu = CPU::new(Some(registers));

        add(&mut cpu, ArithmeticTarget::C);

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

        let mut cpu = CPU::new(Some(registers));

        add(&mut cpu, ArithmeticTarget::A);

        assert_eq!(cpu.registers.a, 0x04);
    }

    #[test]
    fn add_adds_with_carry() {
        let mut registers = Registers::new();
        registers.a = 0b1111_1111;
        registers.d = 0b1;

        let mut cpu = CPU::new(Some(registers));

        add(&mut cpu, ArithmeticTarget::D);

        assert_eq!(cpu.registers.a, 0);

        assert!(cpu.registers.f.carry);
        assert!(cpu.registers.f.zero);
    }

    #[test]
    fn add_adds_with_half_carry() {
        let mut registers = Registers::new();
        registers.a = 0b0000_1111;
        registers.e = 0b1;

        let mut cpu = CPU::new(Some(registers));

        add(&mut cpu, ArithmeticTarget::E);

        assert_eq!(cpu.registers.a, 0b0001_0000);

        assert!(cpu.registers.f.half_carry);
    }

    #[test]
    fn adc_increments_pc() {
        let mut registers = Registers::new();
        registers.a = 0x00;
        registers.b = 0x00;

        let mut cpu = CPU::new(Some(registers));

        assert_eq!(cpu.pc, 0);

        adc(&mut cpu, ArithmeticTarget::B);

        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn adc_without_carry_adds_value() {
        let mut registers = Registers::new();
        registers.a = 0x01;
        registers.b = 0x02;

        let mut cpu = CPU::new(Some(registers));

        adc(&mut cpu, ArithmeticTarget::B);

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

        let mut cpu = CPU::new(Some(registers));

        adc(&mut cpu, ArithmeticTarget::E);

        assert_eq!(cpu.registers.a, 0b0001_0000);

        assert!(cpu.registers.f.half_carry);
    }

    #[test]
    fn adc_adds_with_carry() {
        let mut registers = Registers::new();
        registers.a = 0b1111_1111;
        registers.d = 0b1;

        let mut cpu = CPU::new(Some(registers));

        adc(&mut cpu, ArithmeticTarget::D);

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

        let mut cpu = CPU::new(Some(registers));

        adc(&mut cpu, ArithmeticTarget::D);

        assert_eq!(cpu.registers.a, 1);

        assert!(cpu.registers.f.carry);
    }

    #[test]
    fn add_hl_increments_pc() {
        let mut registers = Registers::new();
        registers.set_hl(0x00);
        registers.set_bc(0x00);

        let mut cpu = CPU::new(Some(registers));

        assert_eq!(cpu.pc, 0);

        add_hl(&mut cpu, ADDHLTarget::BC);

        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn add_hl_adds_value_from_target() {
        let mut registers = Registers::new();
        registers.set_hl(0x00_0A);
        registers.set_de(0x00_0B);

        let mut cpu = CPU::new(Some(registers));

        add_hl(&mut cpu, ADDHLTarget::DE);

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

        let mut cpu = CPU::new(Some(registers));

        add_hl(&mut cpu, ADDHLTarget::BC);

        assert_eq!(cpu.registers.get_hl(), 0);

        assert!(cpu.registers.f.carry);
    }

    #[test]
    fn add_hl_adds_with_half_carry() {
        let mut registers = Registers::new();
        registers.set_hl(0x00_FF);
        registers.set_bc(0x00_01);

        let mut cpu = CPU::new(Some(registers));

        add_hl(&mut cpu, ADDHLTarget::BC);

        assert_eq!(cpu.registers.get_hl(), 0b0000_0001_0000_0000);

        assert!(cpu.registers.f.half_carry);
    }
}
