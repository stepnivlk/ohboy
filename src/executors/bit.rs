use crate::{
    executors::op_to_u8_reg,
    instruction::{Instr, Operand},
    CPU,
};

pub fn bit(cpu: &mut CPU, mut instr: Instr) -> Option<Instr> {
    let bit_position = match instr.lhs {
        Some(Operand::BitPos(n)) => n,
        _ => {
            panic!("{}: Mismatched operand {:?}", instr, instr.lhs)
        }
    };

    let val = match instr.rhs {
        Some(op @ Operand::Reg8(_)) => op_to_u8_reg(&op, &cpu.registers),
        Some(Operand::Reg16Indir(reg)) => cpu.read_at_reg_16(&reg),

        _ => panic!("Mismatched operand {:?}", instr.rhs),
    };

    let bit = (val >> bit_position) & 0b1;

    cpu.registers.f.zero = bit == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;

    cpu.pc.add(2);

    instr.trace((bit as u16, val as u16));

    Some(instr)
}
