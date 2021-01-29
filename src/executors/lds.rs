use crate::{
    executors::{Executor, NullFlagger, Worker},
    values::Value,
    LoadByteSource, LoadByteTarget, CPU,
};

struct LdWorker;

impl Worker for LdWorker {
    type V = (u8, LoadByteTarget, LoadByteSource);
    type D = ();

    fn run(&self, cpu: &mut CPU, value: Self::V) -> Self::D {
        let (source_value, target, source) = value;

        match target {
            LoadByteTarget::A => cpu.registers.a = source_value,
            LoadByteTarget::B => cpu.registers.b = source_value,
            LoadByteTarget::C => cpu.registers.c = source_value,
            LoadByteTarget::D => cpu.registers.d = source_value,
            LoadByteTarget::E => cpu.registers.e = source_value,
            LoadByteTarget::H => cpu.registers.h = source_value,
            LoadByteTarget::L => cpu.registers.l = source_value,
            LoadByteTarget::HL => {
                cpu.bus.write_byte(cpu.registers.get_hl(), source_value)
            }
        };

        match source {
            LoadByteSource::D8 => cpu.pc = cpu.pc.wrapping_add(2),
            _ => cpu.pc = cpu.pc.wrapping_add(1),
        };
    }
}

pub fn ld(cpu: &mut CPU, target: LoadByteTarget, source: LoadByteSource) {
    Executor {
        cpu,
        worker: LdWorker,
        flagger: NullFlagger,
        value: Value((target, source)),
    }
    .run();
}
