use crate::{
    microcode::{should_jump, Exec, ExecRes},
    instruction::Instr, CPU
};

pub struct Call<'a>(pub &'a mut CPU);

impl Exec for Call<'_> {
    type FlagsData = ();

    fn run(&mut self, instr: Instr) -> ExecRes {
        let cpu = self.0;

        let next_pc = cpu.pc.get().wrapping_add(3);

        let ticks = 12;

        if should_jump(cpu, instr.lhs.unwrap()) {
            let hi = ((next_pc & 0xFF00) >> 8) as u8;
            let lo = (next_pc & 0xFF) as u8;

            cpu.sp = cpu.sp.wrapping_sub(1);
            cpu.bus.write_byte(cpu.sp, hi);

            cpu.sp = cpu.sp.wrapping_sub(1);
            cpu.bus.write_byte(cpu.sp, lo);

            let jump_addr = cpu.read_next_word();

            instr.trace((1, jump_addr));

            next_pc = jump_addr;

            ticks = 24;
        } else {
            instr.trace((0, next_pc));
        }

        self.res(ticks, next_pc, instr)
    }
}

pub struct Ret<'a>(pub &'a mut CPU);

impl Exec for Ret<'_> {
    type FlagsData = ();

    fn run(&mut self, instr: Instr) -> ExecRes {
        let cpu = self.0;
        let next_pc = cpu.pc.get();

        if should_jump(cpu, instr.lhs.unwrap()) {
            let lo = cpu.bus.read_byte(cpu.sp) as u16;
            cpu.sp = cpu.sp.wrapping_add(1);

            let hi = cpu.bus.read_byte(cpu.sp) as u16;
            cpu.sp = cpu.sp.wrapping_add(1);

            let address = (hi << 8) | lo;

            next_pc = address;
        } else {
            next_pc = next_pc + 1;
        }

        self.res(16, next_pc, instr)
    }
}
 
