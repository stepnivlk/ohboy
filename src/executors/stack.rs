use crate::{
    executors::{Executor, NullFlagger, Worker},
    values::Value,
    StackTarget, CPU,
};

struct PushWorker;

impl Worker for PushWorker {
    type V = (u16, StackTarget);
    type D = ();

    fn run(&self, cpu: &mut CPU, value: Self::V) -> Self::D {
        let (data, _) = value;

        let hi = ((data & 0xFF00) >> 8) as u8;
        let lo = ((data & 0xFF) >> 8) as u8;

        cpu.sp = cpu.sp.wrapping_sub(1);
        cpu.bus.write_byte(cpu.sp, hi);

        cpu.sp = cpu.sp.wrapping_sub(1);
        cpu.bus.write_byte(cpu.sp, lo);

        cpu.pc = cpu.pc.wrapping_add(1);
    }
}

struct PopWorker;

impl Worker for PopWorker {
    type V = (u16, StackTarget);
    type D = ();

    fn run(&self, cpu: &mut CPU, value: Self::V) -> Self::D {
        let (_, target) = value;

        let lo = cpu.bus.read_byte(cpu.sp) as u16;
        cpu.sp = cpu.sp.wrapping_add(1);

        let hi = cpu.bus.read_byte(cpu.sp) as u16;
        cpu.sp = cpu.sp.wrapping_add(1);

        cpu.pc = cpu.pc.wrapping_add(1);

        let data = (hi << 8) | lo;

        match target {
            StackTarget::BC => cpu.registers.set_bc(data),
            StackTarget::DE => cpu.registers.set_de(data),
            StackTarget::HL => cpu.registers.set_hl(data),
        }

        cpu.pc = cpu.pc.wrapping_add(1);
    }
}

pub fn push(cpu: &mut CPU, value: StackTarget) {
    Executor {
        cpu,
        worker: PushWorker,
        flagger: NullFlagger,
        value: Value(value),
    }
    .run();
}

pub fn pop(cpu: &mut CPU, value: StackTarget) {
    Executor {
        cpu,
        worker: PopWorker,
        flagger: NullFlagger,
        value: Value(value),
    }
    .run();
}
