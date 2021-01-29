use crate::{
    executors::{Executor, NullFlagger, Worker},
    values::Value,
    CPU,
};

struct CallWorker;

impl Worker for CallWorker {
    type V = bool;
    type D = ();

    fn run(&self, cpu: &mut CPU, should_jump: Self::V) -> Self::D {
        let next_pc = cpu.pc.wrapping_add(3);

        if should_jump {
            let hi = ((next_pc & 0xFF00) >> 8) as u8;
            let lo = ((next_pc & 0xFF) >> 8) as u8;

            cpu.sp = cpu.sp.wrapping_sub(1);
            cpu.bus.write_byte(cpu.sp, hi);

            cpu.sp = cpu.sp.wrapping_sub(1);
            cpu.bus.write_byte(cpu.sp, lo);

            cpu.pc = cpu.read_next_word();
        } else {
            cpu.pc = next_pc;
        }
    }
}

struct RetWorker;

impl Worker for RetWorker {
    type V = bool;
    type D = ();

    fn run(&self, cpu: &mut CPU, should_jump: Self::V) -> Self::D {
        if should_jump {
            let lo = cpu.bus.read_byte(cpu.sp) as u16;
            cpu.sp = cpu.sp.wrapping_add(1);

            let hi = cpu.bus.read_byte(cpu.sp) as u16;
            cpu.sp = cpu.sp.wrapping_add(1);

            let address = (hi << 8) | lo;

            cpu.pc = address;
        } else {
            cpu.pc = cpu.pc.wrapping_add(1);
        }
    }
}

pub fn call(cpu: &mut CPU, should_jump: bool) {
    Executor {
        cpu,
        worker: CallWorker,
        flagger: NullFlagger,
        value: Value(should_jump),
    }
    .run();
}

pub fn ret(cpu: &mut CPU, should_jump: bool) {
    Executor {
        cpu,
        worker: CallWorker,
        flagger: NullFlagger,
        value: Value(should_jump),
    }
    .run();
}
