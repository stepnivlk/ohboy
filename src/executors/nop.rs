use crate::{
    executors::{Executor, NullFlagger, Worker},
    values::Value,
    CPU,
};

struct NopWorker;

impl Worker for NopWorker {
    type V = ();
    type D = ();

    fn run(&self, cpu: &mut CPU, _value: Self::V) -> Self::D {
        cpu.pc = cpu.pc.wrapping_add(1);
    }
}

pub fn nop(cpu: &mut CPU) {
    Executor {
        cpu,
        worker: NopWorker,
        flagger: NullFlagger,
        value: Value(()),
    }
    .run();
}
