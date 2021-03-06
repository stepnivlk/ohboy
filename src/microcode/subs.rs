use crate::{
    instr::{Instr, Operand},
    microcode::{op_to_u8_reg, Exec, ExecRes},
    Cpu,
};

pub struct Sub<'a>(pub &'a mut Cpu);

impl Exec for Sub<'_> {
    type FlagsData = ();

    fn run(&mut self, instr: Instr) -> Option<ExecRes> {
        let cpu = &mut self.0;

        let val = op_to_u8_reg(&instr.rhs?, &cpu.registers);
        let a = cpu.registers.a;

        let (new_value, carry) = a.overflowing_sub(val);

        cpu.registers.f.zero = new_value == 0;
        cpu.registers.f.subtract = true;
        // TODO: Should add a carry?
        cpu.registers.f.half_carry = (a & 0xF) < (val & 0xF);
        cpu.registers.f.carry = carry;

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

pub struct Cp<'a>(pub &'a mut Cpu);

impl Exec for Cp<'_> {
    type FlagsData = ();

    fn run(&mut self, instr: Instr) -> Option<ExecRes> {
        let cpu = &mut self.0;

        let val = match instr.rhs {
            Some(Operand::U8) => {
                cpu.pc.add(1);
                cpu.clock.add(4);
                cpu.read_next_byte()
            }
            _ => panic!(
                "[{:X} | {}] unsupported operand {:?}",
                instr.pos, instr.tag, instr.rhs
            ),
        };

        let a = cpu.registers.a;

        let (new_value, carry) = a.overflowing_sub(val);

        cpu.registers.f.zero = new_value == 0;
        cpu.registers.f.subtract = true;
        // TODO: Should add a carry?
        cpu.registers.f.half_carry = (a & 0xF) < (val & 0xF);
        cpu.registers.f.carry = carry;

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

pub struct Sbc<'a>(pub &'a mut Cpu);

impl Exec for Sbc<'_> {
    type FlagsData = ();

    fn run(&mut self, instr: Instr) -> Option<ExecRes> {
        let cpu = &mut self.0;

        let val = op_to_u8_reg(&instr.rhs?, &cpu.registers);
        let a = cpu.registers.a;
        let additinal_carry = cpu.registers.f.carry as u8;

        let (mid_value, mid_carry) = a.overflowing_sub(val);
        let (new_value, carry) = mid_value.overflowing_sub(additinal_carry);

        cpu.registers.f.zero = cpu.registers.a == 0;
        cpu.registers.f.subtract = true;
        cpu.registers.f.half_carry =
            (a & 0xF) < (val & 0xF) + (cpu.registers.f.carry as u8);
        cpu.registers.f.carry = mid_carry || carry;

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
    use crate::{memory_bus, Cpu, Registers};

    fn cpu(registers: Registers) -> Cpu {
        Cpu::new(
            vec![0; memory_bus::BOOT_ROM_SIZE],
            vec![0; memory_bus::ROM_BANK_0_SIZE],
            Some(registers),
        )
    }

    #[test]
    fn sub_increments_pc() {
        let mut registers = Registers::new();
        registers.a = 0x00;

        let mut cpu = cpu(registers);

        assert_eq!(cpu.pc.get(), 0);

        sub(&mut cpu, 0x00);

        assert_eq!(cpu.pc.get(), 1);
    }

    #[test]
    fn sub_subs_value_from_target() {
        let mut registers = Registers::new();
        registers.a = 0x03;

        let mut cpu = cpu(registers);

        sub(&mut cpu, 0x02);

        assert_eq!(cpu.registers.a, 0x01);

        assert!(!cpu.registers.f.zero);
        assert!(!cpu.registers.f.carry);
        assert!(!cpu.registers.f.half_carry);
        assert!(cpu.registers.f.subtract);
    }

    #[test]
    fn sub_subs_with_carry() {
        let mut registers = Registers::new();
        registers.a = 0x10;

        let mut cpu = cpu(registers);

        sub(&mut cpu, 0x20);

        assert_eq!(cpu.registers.a, 240);

        assert!(cpu.registers.f.carry);
        assert!(!cpu.registers.f.half_carry);
    }

    #[test]
    fn sub_subs_with_half_carry() {
        let mut registers = Registers::new();
        registers.a = 0b0001_0111;

        let mut cpu = cpu(registers);

        sub(&mut cpu, 0b0000_1111);

        assert_eq!(cpu.registers.a, 0x08);

        assert!(cpu.registers.f.half_carry);

        assert!(!cpu.registers.f.carry);
    }

    #[test]
    fn sbc_increments_pc() {
        let mut registers = Registers::new();
        registers.a = 0x00;

        let mut cpu = cpu(registers);

        assert_eq!(cpu.pc.get(), 0);

        sbc(&mut cpu, 0x00);

        assert_eq!(cpu.pc.get(), 1);
    }

    #[test]
    fn sbc_without_carry_subs_value() {
        let mut registers = Registers::new();
        registers.a = 0x02;

        let mut cpu = cpu(registers);

        sbc(&mut cpu, 0x01);

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

        let mut cpu = cpu(registers);

        sbc(&mut cpu, 0b0000_1001);

        assert_eq!(cpu.registers.a, 0b0001_1110);

        assert!(cpu.registers.f.half_carry);
        assert!(!cpu.registers.f.carry);
    }

    #[test]
    fn sbc_subs_with_carry() {
        let mut registers = Registers::new();
        registers.a = 0b1111_1110;

        let mut cpu = cpu(registers);

        sbc(&mut cpu, 0b1111_1111);

        assert_eq!(cpu.registers.a, 0xFF);

        assert!(cpu.registers.f.carry);
    }

    #[test]
    fn sbc_through_carry() {
        let mut registers = Registers::new();
        registers.a = 0b1;
        // overflows to 0xFF
        registers.f.carry = true;

        let mut cpu = cpu(registers);

        sbc(&mut cpu, 0b1);

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

        let mut cpu = cpu(registers);

        sbc(&mut cpu, 0b0001_0001);

        assert_eq!(cpu.registers.a, 0x0F);

        assert!(!cpu.registers.f.carry);

        assert!(cpu.registers.f.half_carry);
    }
}
