use crate::registers::{Reg8Kind, Reg16Kind};
use std::{convert, fmt};

pub enum Ar8From {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub enum Ar8To {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub enum ArHlFrom {
    BC,
    DE,
    HL,
    SP,
}

pub enum ArHlTo {
    BC,
    DE,
    HL,
    SP,
}

#[derive(Debug, Copy, Clone)]
pub enum Ld8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    D8,
}

pub enum Ld16 {
    BC,
    DE,
    HL,
    SP,
}

#[derive(Copy, Clone)]
pub enum StackTo {
    BC,
    DE,
    HL,
}

#[derive(Debug)]
pub enum Cond {
    NotZero,
    NotCarry,
    Zero,
    Carry,
    Always,
}

type Meta = (u8, &'static str);

// pub enum Instruction {
    // Nop(Meta),
    // Halt(Meta),
    // Add(Meta, Ar8To, Ar8From),
    // Adc(Meta, Ar8To, Ar8From),
    // AddHl(Meta, ArHlFrom),
    // Sub(Meta, Ar8To, Ar8From),
    // Sbc(Meta, Ar8To, Ar8From),
    // And(Meta, Ar8To, Ar8From),
    // Or(Meta, Ar8To, Ar8From),
    // Xor(Meta, Ar8To, Ar8From),
    // Pop(Meta, StackTo),
    // Push(Meta, StackTo),
    // Jp(Meta, Cond),
    // Call(Meta, Cond),
    // Ret(Meta, Cond),
    // Ld(Meta, Ld8To, Ld8From),
//
    // Unknown(Meta),
// }

// #[derive(Debug)]
// pub enum Reg8Kind {
// A,
// B,
// C,
// D,
// E,
// H,
// L,
// }

#[derive(Debug)]
pub enum InstrKind {
    Unknown,
    Nop,
    Halt,
    Add(Reg8Kind),
    Adc(Reg8Kind),
    AddHl(Reg16Kind),
    Sub(Reg8Kind),
    Sbc(Reg8Kind),
    And(Reg8Kind),
    Or(Reg8Kind),
    Xor(Reg8Kind),
    Pop(Reg16Kind),
    Push(Reg16Kind),
    Jp(Cond),
    Call(Cond),
    Ret(Cond),
    Ld { from: Ld8, to: Ld8 },
}

type Tag = &'static str;

#[derive(Debug)]
pub struct Instr {
    tag: Tag,
    pos: u8,
    pub kind: InstrKind,
}

impl Instr {
    fn new(pos: u8, tag: Tag, kind: InstrKind) -> Self {
        Self { pos, tag, kind }
    }
}

impl convert::From<u8> for Instr {
    fn from(pos: u8) -> Instr {
        use InstrKind::*;
        use Reg8Kind::*;
        use Reg16Kind::*;

        let i = |tag, kind| {
            Instr::new(pos, tag, kind)
        };

        match pos {
            0x00 => i("NOP", Nop),
            0x76 => i("HALT", Halt),

            0x80 => i("ADD A, B", Add(B)),
            0x81 => i("ADD A, C", Add(C)),
            0x82 => i("ADD A, D", Add(D)),
            0x83 => i("ADD A, E", Add(E)),
            0x84 => i("ADD A, H", Add(H)),
            0x85 => i("ADD A, L", Add(L)),
            0x87 => i("ADD A, A", Add(A)),

            0x88 => i("ADC A, B", Adc(B)),
            0x89 => i("ADC A, C", Adc(C)),
            0x8A => i("ADC A, D", Adc(D)),
            0x8B => i("ADC A, E", Adc(E)),
            0x8C => i("ADC A, H", Adc(H)),
            0x8D => i("ADC A, L", Adc(L)),
            0x8F => i("ADC A, A", Adc(A)),

            0x09 => i("ADD HL, BC", AddHl(BC)),
            0x19 => i("ADD HL, DE", AddHl(DE)),
            0x29 => i("ADD HL, HL", AddHl(HL)),
            0x39 => i("ADD HL, SP", AddHl(SP)),

            0x90 => i("SUB A, B", Sub(B)),
            0x91 => i("SUB A, C", Sub(C)),
            0x92 => i("SUB A, D", Sub(D)),
            0x93 => i("SUB A, E", Sub(E)),
            0x94 => i("SUB A, H", Sub(H)),
            0x95 => i("SUB A, L", Sub(L)),
            0x97 => i("SUB A, A", Sub(A)),

            0x98 => i("SBC A, B", Sbc(B)),
            0x99 => i("SBC A, C", Sbc(C)),
            0x9A => i("SBC A, D", Sbc(D)),
            0x9B => i("SBC A, E", Sbc(E)),
            0x9C => i("SBC A, H", Sbc(H)),
            0x9D => i("SBC A, L", Sbc(L)),
            0x9F => i("SBC A, A", Sbc(A)),

            0xA0 => i("AND A, B", And(B)),
            0xA1 => i("AND A, C", And(C)),
            0xA2 => i("AND A, D", And(D)),
            0xA3 => i("AND A, E", And(E)),
            0xA4 => i("AND A, H", And(H)),
            0xA5 => i("AND A, L", And(L)),
            0xA7 => i("AND A, A", And(A)),

            0xB0 => i("OR A, B", Or(B)),
            0xB1 => i("OR A, C", Or(C)),
            0xB2 => i("OR A, D", Or(D)),
            0xB3 => i("OR A, E", Or(E)),
            0xB4 => i("OR A, H", Or(H)),
            0xB5 => i("OR A, L", Or(L)),
            0xB7 => i("OR A, A", Or(A)),

            0xA8 => i("XOR A, B", Xor(B)),
            0xA9 => i("XOR A, C", Xor(C)),
            0xAA => i("XOR A, D", Xor(D)),
            0xAB => i("XOR A, E", Xor(E)),
            0xAC => i("XOR A, H", Xor(H)),
            0xAD => i("XOR A, L", Xor(L)),
            0xAF => i("XOR A, A", Xor(A)),

            0xC1 => i("POP BC", Pop(Reg16Kind::BC)),
            0xD1 => i("POP DE", Pop(Reg16Kind::DE)),
            0xE1 => i("POP HL", Pop(Reg16Kind::HL)),

            0xC5 => i("PUSH BC", Push(Reg16Kind::BC)),
            0xD5 => i("PUSH DE", Push(Reg16Kind::DE)),
            0xE5 => i("PUSH HL", Push(Reg16Kind::HL)),

            0xC3 => i("JP a16", Jp(Cond::Always)),
            0xC2 => i("JP NZ, a16", Jp(Cond::NotZero)),
            0xD2 => i("JP NC, a16", Jp(Cond::NotCarry)),
            0xCA => i("JP Z, a16", Jp(Cond::Zero)),
            0xDA => i("JP C, a16", Jp(Cond::Carry)),

            0xC4 => i("CALL NZ, a16", Call(Cond::NotZero)),
            0xD4 => i("CALL NC, a16", Call(Cond::NotCarry)),
            0xCC => i("CALL Z, a16", Call(Cond::Zero)),
            0xDC => i("CALL C, a16", Call(Cond::Carry)),
            0xCD => i("CALL a16", Call(Cond::Always)),

            0xC0 => i("RET NZ", Ret(Cond::NotZero)),
            0xD0 => i("RET NZ", Ret(Cond::NotCarry)),
            0xC8 => i("RET NC", Ret(Cond::Zero)),
            0xD8 => i("RET Z", Ret(Cond::Carry)),
            0xC9 => i("RET", Ret(Cond::Always)),

            0x40 => i("LD B, B", Ld { from: Ld8::B, to: Ld8::B }),
            0x41 => i("LD B, C", Ld { from: Ld8::B, to: Ld8::C }),
            0x42 => i("LD B, D", Ld { from: Ld8::B, to: Ld8::D }),
            0x43 => i("LD B, E", Ld { from: Ld8::B, to: Ld8::E }),
            0x44 => i("LD B, H", Ld { from: Ld8::B, to: Ld8::H }),
            0x45 => i("LD B, L", Ld { from: Ld8::B, to: Ld8::L }),
            0x46 => i("LD B, (HL)", Ld { from: Ld8::B, to: Ld8::HL }),
            0x47 => i("LD B, A", Ld { from: Ld8::B, to: Ld8::A }),

            // 0x48 => Ld((b, "LD C, B"), Ld8To::C, Ld8From::B),
            // 0x49 => Ld((b, "LD C, C"), Ld8To::C, Ld8From::C),
            // 0x4A => Ld((b, "LD C, D"), Ld8To::C, Ld8From::D),
            // 0x4B => Ld((b, "LD C, E"), Ld8To::C, Ld8From::E),
            // 0x4D => Ld((b, "LD C, L"), Ld8To::C, Ld8From::L),
            // 0x4E => Ld((b, "LD C, (HL)"), Ld8To::C, Ld8From::HL),
            // 0x4F => Ld((b, "LD C, A"), Ld8To::C, Ld8From::A),
            //
            // 0x50 => Ld((b, "LD D, B"), Ld8To::D, Ld8From::B),
            // 0x51 => Ld((b, "LD D, C"), Ld8To::D, Ld8From::C),
            // 0x52 => Ld((b, "LD D, D"), Ld8To::D, Ld8From::D),
            // 0x53 => Ld((b, "LD D, E"), Ld8To::D, Ld8From::E),
            // 0x54 => Ld((b, "LD D, H"), Ld8To::D, Ld8From::H),
            // 0x55 => Ld((b, "LD D, L"), Ld8To::D, Ld8From::L),
            // 0x56 => Ld((b, "LD D, (HL)"), Ld8To::D, Ld8From::HL),
            // 0x57 => Ld((b, "LD D, A"), Ld8To::D, Ld8From::A),
            //
            // 0x58 => Ld((b, "LD E, B"), Ld8To::E, Ld8From::B),
            // 0x59 => Ld((b, "LD E, C"), Ld8To::E, Ld8From::C),
            // 0x5A => Ld((b, "LD E, D"), Ld8To::E, Ld8From::D),
            // 0x5B => Ld((b, "LD E, E"), Ld8To::E, Ld8From::E),
            // 0x5D => Ld((b, "LD E, L"), Ld8To::E, Ld8From::L),
            // 0x5E => Ld((b, "LD E, (HL)"), Ld8To::E, Ld8From::HL),
            // 0x5F => Ld((b, "LD E, A"), Ld8To::E, Ld8From::A),
            //
            // 0x60 => Ld((b, "LD H, B"), Ld8To::H, Ld8From::B),
            // 0x61 => Ld((b, "LD H, C"), Ld8To::H, Ld8From::C),
            // 0x62 => Ld((b, "LD H, D"), Ld8To::H, Ld8From::D),
            // 0x63 => Ld((b, "LD H, E"), Ld8To::H, Ld8From::E),
            // 0x64 => Ld((b, "LD H, H"), Ld8To::H, Ld8From::H),
            // 0x65 => Ld((b, "LD H, L"), Ld8To::H, Ld8From::L),
            // 0x66 => Ld((b, "LD H, (HL)"), Ld8To::H, Ld8From::HL),
            // 0x67 => Ld((b, "LD H, A"), Ld8To::H, Ld8From::A),
            //
            // 0x68 => Ld((b, "LD L, B"), Ld8To::L, Ld8From::B),
            // 0x69 => Ld((b, "LD L, C"), Ld8To::L, Ld8From::C),
            // 0x6A => Ld((b, "LD L, D"), Ld8To::L, Ld8From::D),
            // 0x6B => Ld((b, "LD L, E"), Ld8To::L, Ld8From::E),
            // 0x6D => Ld((b, "LD L, L"), Ld8To::L, Ld8From::L),
            // 0x6E => Ld((b, "LD L, (HL)"), Ld8To::L, Ld8From::HL),
            // 0x6F => Ld((b, "LD L, A"), Ld8To::L, Ld8From::A),
            //
            // 0x70 => Ld((b, "LD (HL), B"), Ld8To::HL, Ld8From::B),
            // 0x71 => Ld((b, "LD (HL), B"), Ld8To::HL, Ld8From::C),
            // 0x72 => Ld((b, "LD (HL), B"), Ld8To::HL, Ld8From::D),
            // 0x73 => Ld((b, "LD (HL), B"), Ld8To::HL, Ld8From::E),
            // 0x74 => Ld((b, "LD (HL), B"), Ld8To::HL, Ld8From::H),
            // 0x75 => Ld((b, "LD (HL), B"), Ld8To::HL, Ld8From::L),
            // 0x77 => Ld((b, "LD (HL), B"), Ld8To::HL, Ld8From::A),
            //
            // 0x78 => Ld((b, "LD A, B"), Ld8To::A, Ld8From::B),
            // 0x79 => Ld((b, "LD A, C"), Ld8To::A, Ld8From::C),
            // 0x7A => Ld((b, "LD A, D"), Ld8To::A, Ld8From::D),
            // 0x7B => Ld((b, "LD A, E"), Ld8To::A, Ld8From::E),
            // 0x7D => Ld((b, "LD A, L"), Ld8To::A, Ld8From::L),
            // 0x7E => Ld((b, "LD A, (HL)"), Ld8To::A, Ld8From::HL),
            // 0x7F => Ld((b, "LD A, A"), Ld8To::A, Ld8From::A),
            //
            // 0x3E => Ld((b, "LD A, d8"), Ld8To::A, Ld8From::D8),
            // 0x06 => Ld((b, "LD B, d8"), Ld8To::B, Ld8From::D8),
            // 0x0E => Ld((b, "LD C, d8"), Ld8To::C, Ld8From::D8),
            // 0x16 => Ld((b, "LD D, d8"), Ld8To::D, Ld8From::D8),
            // 0x1E => Ld((b, "LD E, d8"), Ld8To::E, Ld8From::D8),
            // 0x26 => Ld((b, "LD H, d8"), Ld8To::H, Ld8From::D8),
            // 0x2E => Ld((b, "LD L, d8"), Ld8To::L, Ld8From::D8),
            // 0x36 => Ld((b, "LD (HL), d8"), Ld8To::HL, Ld8From::D8),
            _ => Instr::new(pos, "UNKNOWN", Unknown),
        }
    }
}

// fn meta_from_instruction(instruction: &Instruction) -> &Meta {
    // use Instruction::*;
//
    // match instruction {
        // Nop(meta) => meta,
        // Halt(meta) => meta,
        // Add(meta, _, _) => meta,
        // Adc(meta, _, _) => meta,
        // AddHl(meta, _) => meta,
        // Sub(meta, _, _) => meta,
        // Sbc(meta, _, _) => meta,
        // And(meta, _, _) => meta,
        // Or(meta, _, _) => meta,
        // Xor(meta, _, _) => meta,
        // Ld(meta, _, _) => meta,
        // Pop(meta, _) => meta,
        // Push(meta, _) => meta,
        // Jp(meta, _) => meta,
        // Call(meta, _) => meta,
        // Ret(meta, _) => meta,
//
        // Unknown(meta) => meta,
    // }
// }

// impl fmt::Debug for Instruction {
    // fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let (pos, tag) = meta_from_instruction(self);
        // let pos = format!("0x{:0>2X}", pos);
//
        // f.debug_struct("Instruction")
            // .field("tag", &tag)
            // .field("pos", &pos)
            // .finish()
    // }
// }

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pos = format!("0x{:0>2X}", self.pos);

        write!(f, "{} at {}", self.tag, pos)
    }
}
