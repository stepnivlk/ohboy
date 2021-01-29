use crate::{
    executors::{Executor, Flagger, Worker},
    values::Value,
    ArithmeticTarget, CPU,
};

#[derive(Debug)]
struct Sub8Data {
    value: u8,
    carry: bool,
    prev_a: u8,
}

struct SubFlagger;

impl Flagger for SubFlagger {
    type D = Sub8Data;

    fn run(&self, cpu: &mut CPU, data: Self::D) {
        let carry = cpu.registers.f.carry as u8;

        cpu.registers.f.zero = cpu.registers.a == 0;
        cpu.registers.f.subtract = true;
        cpu.registers.f.half_carry =
            (data.prev_a & 0xF) < (data.value & 0xF) + carry;
        cpu.registers.f.carry = data.carry;
    }
}

struct SubWorker;

impl Worker for SubWorker {
    type V = u8;
    type D = Sub8Data;

    fn run(&self, cpu: &mut CPU, value: Self::V) -> Self::D {
        let prev_a = cpu.registers.a;
        let (new_value, carry) = cpu.registers.a.overflowing_sub(value);

        cpu.registers.a = new_value;
        cpu.pc.add(1);

        Sub8Data {
            value,
            carry,
            prev_a,
        }
    }
}

struct SbcWorker;

impl Worker for SbcWorker {
    type V = u8;
    type D = Sub8Data;

    fn run(&self, cpu: &mut CPU, value: Self::V) -> Self::D {
        let prev_a = cpu.registers.a;
        let additinal_carry = cpu.registers.f.carry as u8;

        let (mid_value, mid_carry) = cpu.registers.a.overflowing_sub(value);
        let (new_value, carry) = mid_value.overflowing_sub(additinal_carry);

        cpu.registers.a = new_value;
        cpu.pc.add(1);

        Sub8Data {
            value,
            carry: mid_carry || carry,
            prev_a,
        }
    }
}

pub fn sub(cpu: &mut CPU, register: ArithmeticTarget) {
    Executor {
        cpu,
        worker: SubWorker,
        flagger: SubFlagger,
        value: Value(register),
    }
    .run();
}
pub fn sbc(cpu: &mut CPU, register: ArithmeticTarget) {
    Executor {
        cpu,
        worker: SbcWorker,
        flagger: SubFlagger,
        value: Value(register),
    }
    .run();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Registers, CPU};

    #[test]
    fn sub_increments_pc() {
        let mut registers = Registers::new();
        registers.a = 0x00;
        registers.b = 0x00;

        let mut cpu = CPU::new(Some(registers));

        assert_eq!(cpu.pc, 0);

        sub(&mut cpu, ArithmeticTarget::B);

        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn sub_subs_value_from_target() {
        let mut registers = Registers::new();
        registers.a = 0x03;
        registers.c = 0x02;

        let mut cpu = CPU::new(Some(registers));

        sub(&mut cpu, ArithmeticTarget::C);

        assert_eq!(cpu.registers.a, 0x01);

        assert!(!cpu.registers.f.zero);
        assert!(!cpu.registers.f.carry);
        assert!(!cpu.registers.f.half_carry);

        assert!(cpu.registers.f.subtract);
    }

    #[test]
    fn sub_subs_value_from_a() {
        let mut registers = Registers::new();
        registers.a = 0x02;

        let mut cpu = CPU::new(Some(registers));

        sub(&mut cpu, ArithmeticTarget::A);

        assert_eq!(cpu.registers.a, 0x00);

        assert!(cpu.registers.f.zero);
    }

    #[test]
    fn sub_subs_with_carry() {
        let mut registers = Registers::new();
        registers.a = 0x10;
        registers.d = 0x20;

        let mut cpu = CPU::new(Some(registers));

        sub(&mut cpu, ArithmeticTarget::D);

        assert_eq!(cpu.registers.a, 240);

        assert!(cpu.registers.f.carry);
        assert!(!cpu.registers.f.half_carry);
    }

    #[test]
    fn sub_subs_with_half_carry() {
        let mut registers = Registers::new();
        registers.a = 0b0001_0111;
        registers.e = 0b0000_1111;

        let mut cpu = CPU::new(Some(registers));

        sub(&mut cpu, ArithmeticTarget::E);

        assert_eq!(cpu.registers.a, 0x08);

        assert!(cpu.registers.f.half_carry);

        assert!(!cpu.registers.f.carry);
    }

    #[test]
    fn sbc_increments_pc() {
        let mut registers = Registers::new();
        registers.a = 0x00;
        registers.b = 0x00;

        let mut cpu = CPU::new(Some(registers));

        assert_eq!(cpu.pc, 0);

        sbc(&mut cpu, ArithmeticTarget::B);

        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn sbc_without_carry_subs_value() {
        let mut registers = Registers::new();
        registers.a = 0x02;
        registers.b = 0x01;

        let mut cpu = CPU::new(Some(registers));

        sbc(&mut cpu, ArithmeticTarget::B);

        assert_eq!(cpu.registers.a, 0x01);

        assert!(!cpu.registers.f.zero);
        assert!(!cpu.registers.f.carry);
        assert!(!cpu.registers.f.half_carry);
        assert!(cpu.registers.f.subtract);
    }

    #[test]
    fn sbc_subs_with_half_carry() {
        let mut registers = Registers::new();
        registers.a = 0b0010_0111;
        registers.e = 0b0000_1001;

        let mut cpu = CPU::new(Some(registers));

        sbc(&mut cpu, ArithmeticTarget::E);

        assert_eq!(cpu.registers.a, 0b0001_1110);

        assert!(cpu.registers.f.half_carry);
        assert!(!cpu.registers.f.carry);
    }

    #[test]
    fn sbc_subs_with_carry() {
        let mut registers = Registers::new();
        registers.a = 0b1111_1110;
        registers.d = 0b1111_1111;

        let mut cpu = CPU::new(Some(registers));

        sbc(&mut cpu, ArithmeticTarget::D);

        assert_eq!(cpu.registers.a, 0xFF);

        assert!(cpu.registers.f.carry);
    }

    #[test]
    fn sbc_through_carry() {
        let mut registers = Registers::new();
        registers.a = 0b1;
        registers.d = 0b1;
        // overflows to 0xFF
        registers.f.carry = true;

        let mut cpu = CPU::new(Some(registers));

        sbc(&mut cpu, ArithmeticTarget::D);

        assert_eq!(cpu.registers.a, 0xFF);

        assert!(cpu.registers.f.carry);
    }

    #[test]
    fn sbc_through_carry_sets_half_carry() {
        let mut registers = Registers::new();
        registers.a = 0b0010_0001;
        registers.d = 0b0001_0001;
        // 1 - 1 - 1 -> overflows
        registers.f.carry = true;

        let mut cpu = CPU::new(Some(registers));

        sbc(&mut cpu, ArithmeticTarget::D);

        assert_eq!(cpu.registers.a, 0x0F);

        assert!(!cpu.registers.f.carry);

        assert!(cpu.registers.f.half_carry);
    }
}
