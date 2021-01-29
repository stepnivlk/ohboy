use crate::{
    executors::{Executor, Flagger, Worker},
    values::Value,
    ArithmeticTarget, CPU,
};

struct XorFlagger;

impl Flagger for XorFlagger {
    type D = u8;

    fn run(&self, cpu: &mut CPU, data: Self::D) {
        cpu.registers.f.zero = data == 0;

        cpu.registers.f.subtract = false;
        cpu.registers.f.half_carry = false;
        cpu.registers.f.carry = false;
    }
}

struct XorWorker;

impl Worker for XorWorker {
    type V = u8;
    type D = u8;

    fn run(&self, cpu: &mut CPU, value: Self::V) -> Self::D {
        let new_value = cpu.registers.a ^ value;

        cpu.registers.a = new_value;
        cpu.pc = cpu.pc.wrapping_add(1);

        new_value
    }
}

pub fn xor(cpu: &mut CPU, register: ArithmeticTarget) {
    Executor {
        cpu,
        worker: XorWorker,
        flagger: XorFlagger,
        value: Value(register),
    }
    .run();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Registers, CPU};

    #[test]
    fn it_sets_a_register_correctly() {
        let mut registers = Registers::new();
        registers.a = 0b1000_1111;
        registers.b = 0b1010_1001;

        let mut cpu = CPU::new(Some(registers));

        xor(&mut cpu, ArithmeticTarget::B);

        assert_eq!(cpu.registers.a, 0b0010_0110);

        assert_eq!(cpu.registers.f.zero, false);
    }

    #[test]
    fn it_sets_flags() {
        let mut registers = Registers::new();
        registers.a = 0x12;
        registers.d = 0x13;

        let mut cpu = CPU::new(Some(registers));

        xor(&mut cpu, ArithmeticTarget::D);

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

        let mut cpu = CPU::new(Some(registers));

        xor(&mut cpu, ArithmeticTarget::C);

        assert_eq!(cpu.registers.a, 0x00);

        assert!(cpu.registers.f.zero);
    }

    #[test]
    fn it_increments_pc() {
        let mut registers = Registers::new();
        registers.a = 0x00;
        registers.d = 0x00;

        let mut cpu = CPU::new(Some(registers));

        assert_eq!(cpu.pc, 0);

        xor(&mut cpu, ArithmeticTarget::E);

        assert_eq!(cpu.pc, 1);
    }
}
