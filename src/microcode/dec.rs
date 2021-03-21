use crate::{
    instr::{Instr, Operand},
    microcode::{Exec, ExecRes},
    Cpu,
};

pub struct Dec<'a>(pub &'a mut Cpu);

impl Exec for Dec<'_> {
    type FlagsData = ();

    fn run(&mut self, instr: Instr) -> Option<ExecRes> {
        let cpu = &mut self.0;

        let mut ticks = 4;

        match instr.rhs {
            Some(Operand::Reg8(kind)) => {
                let orig_val = cpu.registers.get_8(&kind);
                let val = orig_val.wrapping_sub(1);

                cpu.registers.set_8(&kind, val);

                cpu.registers.f.zero = val == 0;
                cpu.registers.f.subtract = true;
                cpu.registers.f.half_carry = orig_val & 0xF == 0x0;
            }

            Some(Operand::Reg16(kind)) => {
                let val = cpu.registers.get_16(&kind).wrapping_sub(1);

                cpu.registers.set_16(&kind, val);

                ticks += 4;
            }

            Some(Operand::Reg16Indir(kind)) => {
                let addr = cpu.registers.get_16(&kind);

                let orig_val = cpu.bus.read_byte(addr);
                let val = orig_val.wrapping_sub(1);

                cpu.bus.write_byte(addr, val);

                cpu.registers.f.zero = val == 0;
                cpu.registers.f.subtract = true;
                cpu.registers.f.half_carry = orig_val & 0xF == 0x0;

                ticks += 8;
            }

            _ => {
                panic!("{}: Mismatched operand {:?}", instr, instr.rhs)
            }
        };

        cpu.pc.add(1);
        cpu.clock.add(ticks);

        Some(ExecRes {
            ticks,
            length: 1,
            instr,
            trace: None,
        })
    }
}
