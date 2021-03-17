use crate::{
    instruction::Instr,
    microcode::{should_jump, Exec, ExecRes},
    CPU,
};

pub struct Jp<'a>(pub &'a mut CPU);

impl Exec for Jp<'_> {
    type FlagsData = ();

    fn run(&mut self, instr: Instr) -> Option<ExecRes> {
        let cpu = &mut self.0;

        if !should_jump(cpu, instr.lhs.unwrap()) {
            cpu.pc.add(3);
            cpu.clock.add(12);

            return None;
        }

        let lsb = cpu.bus.read_byte(cpu.pc.get() + 1) as u16;
        let msb = cpu.bus.read_byte(cpu.pc.get() + 2) as u16;

        let address = (msb << 8) | lsb;

        cpu.pc.set(address);

        None
    }
}

pub struct Jr<'a>(pub &'a mut CPU);

impl Exec for Jr<'_> {
    type FlagsData = ();

    fn run(&mut self, instr: Instr) -> Option<ExecRes> {
        let cpu = &mut self.0;

        let next_step = cpu.pc.get().wrapping_add(2);

        if !should_jump(cpu, instr.lhs.unwrap()) {
            // do not jump
            cpu.pc.set(next_step);
            cpu.clock.add(8);
        } else {
            let offset = cpu.read_next_byte() as i8;

            let next_pc = if offset >= 0 {
                next_step.wrapping_add(offset as u16)
            } else {
                next_step.wrapping_sub(offset.abs() as u16)
            };

            cpu.pc.set(next_pc);
            cpu.clock.add(12);
        }

        None
    }
}
