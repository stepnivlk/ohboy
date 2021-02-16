use crate::{executors::should_jump, instruction::Instr, CPU};

pub fn call(cpu: &mut CPU, mut instr: Instr) -> Option<Instr> {
    let next_pc = cpu.pc.get().wrapping_add(3);

    if should_jump(cpu, instr.lhs?) {
        let hi = ((next_pc & 0xFF00) >> 8) as u8;
        let lo = (next_pc & 0xFF) as u8;

        cpu.sp = cpu.sp.wrapping_sub(1);
        cpu.bus.write_byte(cpu.sp, hi);

        cpu.sp = cpu.sp.wrapping_sub(1);
        cpu.bus.write_byte(cpu.sp, lo);

        let jump_addr = cpu.read_next_word();

        instr.trace((1, jump_addr));

        cpu.pc.set(jump_addr);
    } else {
        instr.trace((0, next_pc));

        cpu.pc.set(next_pc);
    }

    Some(instr)
}

pub fn ret(cpu: &mut CPU, instr: Instr) -> Option<Instr> {
    if should_jump(cpu, instr.lhs?) {
        let lo = cpu.bus.read_byte(cpu.sp) as u16;
        cpu.sp = cpu.sp.wrapping_add(1);

        let hi = cpu.bus.read_byte(cpu.sp) as u16;
        cpu.sp = cpu.sp.wrapping_add(1);

        let address = (hi << 8) | lo;

        cpu.pc.set(address);
    } else {
        cpu.pc.add(1);
    }

    Some(instr)
}
