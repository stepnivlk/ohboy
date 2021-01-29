use crate::{
    executors::{Executor, NullFlagger, Worker},
    values::Value,
    CPU,
};

struct JpWorker;

impl Worker for JpWorker {
    type V = bool;
    type D = ();

    fn run(&self, cpu: &mut CPU, should_jump: Self::V) -> Self::D {
        if !should_jump {
            cpu.pc = cpu.pc.wrapping_add(3);

            return;
        }

        let least_significant_byte = cpu.bus.read_byte(cpu.pc + 1) as u16;
        let most_significant_byte = cpu.bus.read_byte(cpu.pc + 2) as u16;

        let address = (most_significant_byte << 8) | least_significant_byte;

        cpu.pc = address;
    }
}

struct JpHlWorker;

impl Worker for JpHlWorker {
    type V = ();
    type D = ();

    fn run(&self, cpu: &mut CPU, _value: Self::V) -> Self::D {
        cpu.pc = cpu.registers.get_hl();
    }
}

pub fn jp(cpu: &mut CPU, should_jump: bool) {
    Executor {
        cpu,
        worker: JpWorker,
        flagger: NullFlagger,
        value: Value(should_jump),
    }
    .run();
}

pub fn jp_hl(cpu: &mut CPU) {
    Executor {
        cpu,
        worker: JpHlWorker,
        flagger: NullFlagger,
        value: Value(()),
    }
    .run();
}
