use crate::{
    instruction::{Instr, Operand, PostOp},
    registers::{Reg16Kind, Reg8Kind},
    CPU,
};

pub fn ld(cpu: &mut CPU, mut instr: Instr) -> Option<Instr> {
    let rhs = match &instr.rhs {
        Some(Operand::Reg8(reg)) => match reg {
            Reg8Kind::A => cpu.registers.a,
            Reg8Kind::B => cpu.registers.b,
            Reg8Kind::C => cpu.registers.c,
            Reg8Kind::D => cpu.registers.d,
            Reg8Kind::E => cpu.registers.e,
            Reg8Kind::H => cpu.registers.h,
            Reg8Kind::L => cpu.registers.l,
        },
        Some(Operand::U8Indir(offset)) => {
            let addr = offset + (cpu.read_next_byte() as u16);

            cpu.bus.read_byte(addr)
        },
        Some(Operand::Reg16Indir(reg)) => cpu.read_at_reg_16(reg),
        Some(Operand::U8) => cpu.read_next_byte(),
        _ => panic!("[{:X} | {}] unsupported operand {:?}", instr.pos, instr.tag, instr.rhs),
    };

    let lhs = match &instr.lhs {
        Some(Operand::Reg8(reg)) => {
            match reg {
                Reg8Kind::A => cpu.registers.a = rhs,
                Reg8Kind::B => cpu.registers.b = rhs,
                Reg8Kind::C => cpu.registers.c = rhs,
                Reg8Kind::D => cpu.registers.d = rhs,
                Reg8Kind::E => cpu.registers.e = rhs,
                Reg8Kind::H => cpu.registers.h = rhs,
                Reg8Kind::L => cpu.registers.l = rhs,
            };

            rhs as u16
        },
        Some(Operand::Reg16Indir(Reg16Kind::HL)) => {
            let addr = cpu.registers.get_hl();

            cpu.bus.write_byte(addr, rhs);

            addr
        },
        Some(Operand::Reg8Indir(Reg8Kind::C, offset)) => {
            let addr = offset + (cpu.registers.c as u16);

            cpu.bus.write_byte(addr, rhs);

            addr
        },
        Some(Operand::U8Indir(offset)) => {
            let addr = offset + (cpu.read_next_byte() as u16);

            cpu.bus.write_byte(addr, rhs);

            addr
        },
        Some(Operand::U16Indir) => {
            let addr = cpu.read_next_word();

            cpu.bus.write_byte(addr, rhs);

            addr
        },
        _ => panic!("{}: unsupported operand {:?}", instr, instr.lhs),
    };

    instr.trace((lhs, rhs as u16));

    match &instr.rhs {
        Some(Operand::U8Indir(_)) => cpu.pc.add(2),
        Some(Operand::U8) => cpu.pc.add(2),
        _ => {
            match &instr.lhs {
                Some(Operand::U8Indir(_)) => {
                    cpu.pc.add(2);
                },
                Some(Operand::U16Indir) => {
                    cpu.pc.add(3);
                },
                _ => cpu.pc.add(1),
            }
        },
    };

    match &instr.post_op {
        Some(PostOp::Dec(Reg16Kind::HL)) => {
            cpu.registers.set_hl(cpu.registers.get_hl().wrapping_sub(1))
        },
        Some(PostOp::Inc(Reg16Kind::HL)) => {
            // println!("{}", instr);
            // panic!();
            cpu.registers.set_hl(cpu.registers.get_hl().wrapping_add(1))
        },
        None => {},
        _ => panic!("{}: unsupported post_op {:?}", instr, instr.post_op),
    };

    Some(instr)
}
