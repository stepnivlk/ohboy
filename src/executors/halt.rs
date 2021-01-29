use crate::{
    executors::{Executor, NullFlagger, Worker},
    values::Value,
    CPU,
};

struct HaltWorker;

impl Worker for HaltWorker {
    type V = ();
    type D = ();

    fn run(&self, cpu: &mut CPU, _value: Self::V) -> Self::D {
        cpu.is_halted = true;
    }
}

pub fn halt(cpu: &mut CPU) {
    Executor {
        cpu,
        worker: HaltWorker,
        flagger: NullFlagger,
        value: Value(()),
    }
    .run();
}
