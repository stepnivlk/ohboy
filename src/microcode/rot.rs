use crate::{
    instr::{Instr, Operand, PostOp},
    microcode::{Exec, ExecRes},
    Cpu,
};

pub struct Rot<'a>(pub &'a mut Cpu);

impl Exec for Rot<'_> {
    type FlagsData = ();

    fn run(&mut self, instr: Instr) -> Option<ExecRes> {
        let cpu = &mut self.0;

        let lhs = instr.lhs.unwrap();
        let lhs = match lhs {
            Operand::Reg8(r) => r,
            _ => panic!(),
        };

        let mut val = cpu.registers.get_8(&lhs);

        match instr.rhs {
            Some(Operand::RotLeft) => {
                val = val << 1;
            }

            _ => {
                panic!("{}: Mismatched operand {:?}", instr, instr.rhs)
            }
        }

        match instr.post_op {
            Some(PostOp::CarryToB0) => {
                let carry = if cpu.registers.f.carry { 1 } else { 0 };

                val = val | carry;
            }

            _ => panic!("{}: unsupported post_op {:?}", instr, instr.post_op),
        }

        cpu.registers.set_8(&lhs, val);

        // TODO: Flags

        cpu.pc.add(2);
        // TODO:
        cpu.clock.add(8);

        Some(ExecRes {
            ticks: 8,
            length: 2,
            instr,
            trace: None,
        })
    }
}

pub struct RotA<'a>(pub &'a mut Cpu);

impl Exec for RotA<'_> {
    type FlagsData = ();

    fn run(&mut self, instr: Instr) -> Option<ExecRes> {
        let mut val = self.0.registers.a;

        match instr.rhs {
            Some(Operand::RotLeft) => {
                self.0.registers.f.carry = (val & 0x80) == 0x80;

                val = val << 1;
            }

            _ => {
                panic!("{}: Mismatched operand {:?}", instr, instr.rhs)
            }
        }

        match instr.post_op {
            Some(PostOp::CarryToB0) => {
                let carry = if self.0.registers.f.carry { 1 } else { 0 };

                val = val | carry;
            }

            _ => panic!("{}: unsupported post_op {:?}", instr, instr.post_op),
        }

        self.0.registers.a = val;

        self.0.registers.f.zero = false;
        self.0.registers.f.subtract = false;
        self.0.registers.f.half_carry = false;

        self.0.pc.add(1);
        self.0.clock.add(4);

        Some(ExecRes {
            ticks: 4,
            length: 1,
            instr,
            trace: None,
        })
    }
}
