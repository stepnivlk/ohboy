use crate::{
    instr::Instr,
    microcode::{op_to_u16_reg, op_to_u16_reg_w, Exec, ExecRes},
    Cpu,
};

pub struct Push<'a>(pub &'a mut Cpu);

impl Exec for Push<'_> {
    type FlagsData = ();

    fn run(&mut self, instr: Instr) -> Option<ExecRes> {
        let cpu = &mut self.0;

        let val = op_to_u16_reg(&instr.rhs?, &cpu.registers);

        let hi = ((val & 0xFF00) >> 8) as u8;
        let lo = ((val & 0xFF) >> 8) as u8;

        cpu.sp = cpu.sp.wrapping_sub(1);
        cpu.bus.write_byte(cpu.sp, hi);

        cpu.sp = cpu.sp.wrapping_sub(1);
        cpu.bus.write_byte(cpu.sp, lo);

        cpu.pc.add(1);
        cpu.clock.add(16);

        Some(ExecRes {
            ticks: 16,
            length: 1,
            instr,
            trace: None,
        })
    }
}

pub struct Pop<'a>(pub &'a mut Cpu);

impl Exec for Pop<'_> {
    type FlagsData = ();

    fn run(&mut self, instr: Instr) -> Option<ExecRes> {
        let cpu = &mut self.0;

        let lo = cpu.bus.read_byte(cpu.sp) as u16;
        cpu.sp = cpu.sp.wrapping_add(1);

        let hi = cpu.bus.read_byte(cpu.sp) as u16;
        cpu.sp = cpu.sp.wrapping_add(1);

        let data = (hi << 8) | lo;

        op_to_u16_reg_w(&instr.rhs?, &mut cpu.registers, data);

        cpu.pc.add(1);

        Some(ExecRes {
            ticks: 12,
            length: 1,
            instr,
            trace: None,
        })
    }
}
