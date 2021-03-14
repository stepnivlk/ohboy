use crate::{executors::should_jump, instruction::Instr, CPU};

pub fn jp(cpu: &mut CPU, instr: Instr) -> Option<Instr> {
    if !should_jump(cpu, instr.lhs?) {
        cpu.pc.add(3);

        return Some(instr);
    }

    let lsb = cpu.bus.read_byte(cpu.pc.get() + 1) as u16;
    let msb = cpu.bus.read_byte(cpu.pc.get() + 2) as u16;

    let address = (msb << 8) | lsb;

    cpu.pc.set(address);

    Some(instr)
}

pub fn jp_hl(cpu: &mut CPU) {
    cpu.pc.set(cpu.registers.get_hl());
}

pub fn jr(cpu: &mut CPU, instr: Instr) -> Option<Instr> {
    let next_step = cpu.pc.get().wrapping_add(2);

    if !should_jump(cpu, instr.lhs?) {
        // do not jump
        cpu.pc.set(next_step);
    } else {
        let offset = cpu.read_next_byte() as i8;

        let next_pc = if offset >= 0 {
            next_step.wrapping_add(offset as u16)
        } else {
            next_step.wrapping_sub(offset.abs() as u16)
        };

        cpu.pc.set(next_pc);
    }

    Some(instr)
}
