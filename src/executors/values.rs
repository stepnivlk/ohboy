use crate::{
    ADDHLTarget, ArithmeticTarget, LoadByteSource, LoadByteTarget, StackTarget,
    CPU,
};

pub trait ValueGetter {
    type V;

    fn get_one(&self, cpu: &CPU) -> Self::V;
}

pub struct Value<T>(pub T);

impl ValueGetter for Value<ArithmeticTarget> {
    type V = u8;

    fn get_one(&self, cpu: &CPU) -> Self::V {
        match self.0 {
            ArithmeticTarget::A => cpu.registers.a,
            ArithmeticTarget::B => cpu.registers.b,
            ArithmeticTarget::C => cpu.registers.c,
            ArithmeticTarget::D => cpu.registers.d,
            ArithmeticTarget::E => cpu.registers.e,
            ArithmeticTarget::H => cpu.registers.h,
            ArithmeticTarget::L => cpu.registers.l,
        }
    }
}

impl ValueGetter for Value<ADDHLTarget> {
    type V = u16;

    fn get_one(&self, cpu: &CPU) -> Self::V {
        match self.0 {
            ADDHLTarget::BC => cpu.registers.get_bc(),
            ADDHLTarget::DE => cpu.registers.get_de(),
            ADDHLTarget::HL => cpu.registers.get_hl(),
            _ => 0,
        }
    }
}

impl ValueGetter for Value<bool> {
    type V = bool;

    fn get_one(&self, _cpu: &CPU) -> Self::V {
        self.0
    }
}

impl ValueGetter for Value<()> {
    type V = ();

    fn get_one(&self, _cpu: &CPU) -> Self::V {
        self.0
    }
}

impl ValueGetter for Value<(LoadByteTarget, LoadByteSource)> {
    type V = (u8, LoadByteTarget, LoadByteSource);

    fn get_one(&self, cpu: &CPU) -> Self::V {
        let (target, source) = self.0;

        let source_value = match source {
            LoadByteSource::A => cpu.registers.a,
            LoadByteSource::B => cpu.registers.b,
            LoadByteSource::C => cpu.registers.c,
            LoadByteSource::D => cpu.registers.d,
            LoadByteSource::E => cpu.registers.e,
            LoadByteSource::H => cpu.registers.h,
            LoadByteSource::L => cpu.registers.l,
            LoadByteSource::HL => cpu.read_at_hl(),
            LoadByteSource::D8 => cpu.read_next(),
        };

        (source_value, target, source)
    }
}

impl ValueGetter for Value<StackTarget> {
    type V = (u16, StackTarget);

    fn get_one(&self, cpu: &CPU) -> Self::V {
        match self.0 {
            StackTarget::BC => (cpu.registers.get_bc(), self.0),
            StackTarget::DE => (cpu.registers.get_de(), self.0),
            StackTarget::HL => (cpu.registers.get_hl(), self.0),
        }
    }
}
