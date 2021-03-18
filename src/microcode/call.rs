use crate::{
    instr::Instr,
    microcode::{should_jump, Exec, ExecRes},
    Cpu,
};

pub struct Call<'a>(pub &'a mut Cpu);

impl Exec for Call<'_> {
    type FlagsData = ();

    fn run(&mut self, instr: Instr) -> Option<ExecRes> {
        let next_pc = self.0.pc.get().wrapping_add(3);

        let mut ticks = 12;

        let trace;

        if should_jump(self.0, instr.lhs.unwrap()) {
            let hi = ((next_pc & 0xFF00) >> 8) as u8;
            let lo = (next_pc & 0xFF) as u8;

            self.0.sp = self.0.sp.wrapping_sub(1);
            self.0.bus.write_byte(self.0.sp, hi);

            self.0.sp = self.0.sp.wrapping_sub(1);
            self.0.bus.write_byte(self.0.sp, lo);

            let jump_addr = self.0.read_next_word();

            trace = (1, jump_addr);

            self.0.pc.set(jump_addr);

            ticks = 24;
        } else {
            trace = (0, next_pc);

            self.0.pc.set(next_pc);
        }

        self.0.clock.add(ticks);

        Some(ExecRes {
            ticks,
            length: next_pc,
            instr,
            trace: Some(trace),
        })
    }
}

pub struct Ret<'a>(pub &'a mut Cpu);

impl Exec for Ret<'_> {
    type FlagsData = ();

    fn run(&mut self, instr: Instr) -> Option<ExecRes> {
        let next_pc = self.0.pc.get();

        if should_jump(self.0, instr.lhs.unwrap()) {
            let lo = self.0.bus.read_byte(self.0.sp) as u16;
            self.0.sp = self.0.sp.wrapping_add(1);

            let hi = self.0.bus.read_byte(self.0.sp) as u16;
            self.0.sp = self.0.sp.wrapping_add(1);

            let address = (hi << 8) | lo;

            self.0.pc.set(address);
        } else {
            self.0.pc.add(1);
        }

        self.0.clock.add(16);

        Some(ExecRes {
            ticks: 16,
            length: next_pc,
            instr,
            trace: None,
        })
    }
}
