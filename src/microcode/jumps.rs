use crate::{
    instr::Instr,
    microcode::{should_jump, Exec, ExecRes},
    Cpu,
};

pub struct Jp<'a>(pub &'a mut Cpu);

impl Exec for Jp<'_> {
    type FlagsData = ();

    fn run(&mut self, instr: Instr) -> Option<ExecRes> {
        let cpu = &mut self.0;

        if !should_jump(cpu, instr.lhs.unwrap()) {
            cpu.pc.add(3);
            cpu.clock.add(12);

            return Some(ExecRes {
                ticks: 12,
                length: 3,
                instr,
                trace: None,
            });
        }

        let lsb = cpu.bus.read_byte(cpu.pc.get() + 1) as u16;
        let msb = cpu.bus.read_byte(cpu.pc.get() + 2) as u16;

        let address = (msb << 8) | lsb;

        cpu.pc.set(address);
        cpu.clock.add(16);

        Some(ExecRes {
            ticks: 16,
            length: 3,
            instr,
            trace: None,
        })
    }
}

pub struct Jr<'a>(pub &'a mut Cpu);

impl Exec for Jr<'_> {
    type FlagsData = ();

    fn run(&mut self, instr: Instr) -> Option<ExecRes> {
        let cpu = &mut self.0;

        let next_step = cpu.pc.get().wrapping_add(2);
        let mut ticks = 8;

        if !should_jump(cpu, instr.lhs.unwrap()) {
            // do not jump
            cpu.pc.set(next_step);
        } else {
            let offset = cpu.read_next_byte() as i8;

            let next_pc = if offset >= 0 {
                next_step.wrapping_add(offset as u16)
            } else {
                next_step.wrapping_sub(offset.abs() as u16)
            };

            cpu.pc.set(next_pc);
            ticks += 4;
        }

        cpu.clock.add(ticks);

        Some(ExecRes {
            ticks,
            length: 3,
            instr,
            trace: None,
        })
    }
}
