use crate::{
    executors::{op_to_u16_reg, op_to_u16_reg_w},
    instruction::Instr,
    CPU,
};

pub fn push(cpu: &mut CPU, instr: Instr) -> Option<Instr> {
    let val = op_to_u16_reg(&instr.rhs?, &cpu.registers);

    let hi = ((val & 0xFF00) >> 8) as u8;
    let lo = ((val & 0xFF) >> 8) as u8;

    cpu.sp = cpu.sp.wrapping_sub(1);
    cpu.bus.write_byte(cpu.sp, hi);

    cpu.sp = cpu.sp.wrapping_sub(1);
    cpu.bus.write_byte(cpu.sp, lo);

    cpu.pc.add(1);

    Some(instr)
}

pub fn pop(cpu: &mut CPU, instr: Instr) -> Option<Instr> {
    dbg!(cpu.sp);
    let lo = cpu.bus.read_byte(cpu.sp) as u16;
    cpu.sp = cpu.sp.wrapping_add(1);

    let hi = cpu.bus.read_byte(cpu.sp) as u16;
    cpu.sp = cpu.sp.wrapping_add(1);

    cpu.pc.add(1);

    let data = (hi << 8) | lo;

    op_to_u16_reg_w(&instr.rhs?, &mut cpu.registers, data);

    cpu.pc.add(1);

    Some(instr)
}
