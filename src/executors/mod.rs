mod adds;
mod and;
mod call;
mod halt;
mod jumps;
mod lds;
mod nop;
mod or;
mod stack;
mod subs;
mod xor;

pub mod values;

use crate::CPU;
use values::ValueGetter;

pub use adds::{adc, add, add_hl};
pub use and::and;
pub use call::{call, ret};
pub use halt::halt;
pub use jumps::jp;
pub use lds::ld;
pub use nop::nop;
pub use or::or;
pub use stack::{pop, push};
pub use subs::{sbc, sub};
pub use xor::xor;

trait Worker {
    type V;
    type D;

    fn run(&self, cpu: &mut CPU, value: Self::V) -> Self::D;
}

trait Flagger {
    type D;

    fn run(&self, cpu: &mut CPU, data: Self::D);
}

struct NullFlagger;

impl Flagger for NullFlagger {
    type D = ();

    fn run(&self, _cpu: &mut CPU, _data: Self::D) {}
}

struct Executor<'a, V, D, W, F, VG>
where
    W: Worker<V = V, D = D>,
    F: Flagger<D = D>,
    VG: ValueGetter<V = V>,
{
    cpu: &'a mut CPU,
    worker: W,
    flagger: F,
    value: VG,
}

impl<'a, V, D, W, F, VG> Executor<'a, V, D, W, F, VG>
where
    W: Worker<V = V, D = D>,
    F: Flagger<D = D>,
    VG: ValueGetter<V = V>,
{
    fn run(&mut self) {
        let value = self.value.get_one(&self.cpu);
        let data = self.worker.run(self.cpu, value);

        self.flagger.run(self.cpu, data);
    }
}
