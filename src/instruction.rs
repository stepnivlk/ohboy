use crate::registers::{Reg16Kind, Reg8Kind};
use std::{convert, fmt};

#[derive(Debug, Clone, Copy)]
pub enum Operand {
    Reg8(Reg8Kind),
    Reg16(Reg16Kind),
    U8,
    U8Indir(u16),
    U16,
    Reg8Indir(Reg8Kind, u16),
    Reg16Indir(Reg16Kind),
    Cond(CondKind),
    RotLeft,
    RotRight,
    BitPos(u8),
}

#[derive(Debug, Clone, Copy)]
pub enum CondKind {
    NotZero,
    NotCarry,
    Zero,
    Carry,
    Always,
}

#[derive(Debug)]
pub enum PostOp {
    Dec(Reg16Kind),
    Inc(Reg16Kind),
    CarryToB7,
    B7ToCarryAndB0,
    CarryToB0,
    B0ToCarryAndB7,
}

#[derive(Debug)]
pub enum InstrKind {
    Unimpl,
    Nop,
    Halt,
    Add,
    Adc,
    AddHl,
    Sub,
    Sbc,
    And,
    Or,
    Xor,
    Pop,
    Push,
    Jp,
    Call,
    Ret,
    Ld,
    LdWord,
    Jr,
    Inc,
    Dec,
    Rot,
    Bit,
}

type Tag = &'static str;

#[derive(Debug)]
pub struct Instr {
    pub tag: Tag,
    pub pos: u16,
    pub id: InstrKind,
    pub lhs: Option<Operand>,
    pub rhs: Option<Operand>,
    pub post_op: Option<PostOp>,
    pub trace: Option<(u16, u16)>,
}

impl Instr {
    fn new(pos: u16, tag: Tag) -> Self {
        Self {
            pos,
            tag,
            id: InstrKind::Unimpl,
            lhs: None,
            rhs: None,
            post_op: None,
            trace: None,
        }
    }

    fn id(mut self, kind: InstrKind) -> Self {
        self.id = kind;

        self
    }

    fn lhs(mut self, operand: Operand) -> Self {
        self.lhs = Some(operand);

        self
    }

    pub fn rhs(mut self, operand: Operand) -> Self {
        self.rhs = Some(operand);

        self
    }

    fn post_op(mut self, op: PostOp) -> Self {
        self.post_op = Some(op);

        self
    }

    pub fn trace(&mut self, data: (u16, u16)) {
        self.trace = Some(data);
    }
}

impl convert::From<u16> for Instr {
    fn from(pos: u16) -> Instr {
        use InstrKind::*;
        use Operand::*;
        use Reg16Kind::*;
        use Reg8Kind::*;
        let i = |tag| Instr::new(pos, tag);

        match pos {
            0xCB00 => i("RLC B").id(Rot).lhs(Reg8(B)).rhs(RotLeft).post_op(PostOp::B7ToCarryAndB0),
            0xCB01 => i("RLC C"),
            0xCB02 => i("RLC D"),
            0xCB03 => i("RLC E"),
            0xCB04 => i("RLC H"),
            0xCB05 => i("RLC L"),
            0xCB06 => i("RLC (HL)"),
            0xCB07 => i("RLC A"),
            0xCB08 => i("RRC B"),
            0xCB09 => i("RRC C"),
            0xCB0A => i("RRC D"),
            0xCB0B => i("RRC E"),
            0xCB0C => i("RRC H"),
            0xCB0D => i("RRC L"),
            0xCB0E => i("RRC (HL)"),
            0xCB0F => i("RRC A"),

            0xCB10 => i("RL B"),
            0xCB11 => i("RL C").id(Rot).lhs(Reg8(C)).rhs(RotLeft).post_op(PostOp::CarryToB0),
            0xCB12 => i("RL D"),
            0xCB13 => i("RL E"),
            0xCB14 => i("RL H"),
            0xCB15 => i("RL L"),
            0xCB16 => i("RL (HL)"),
            0xCB17 => i("RL A"),
            0xCB18 => i("RR B"),
            0xCB19 => i("RR C"),
            0xCB1A => i("RR D"),
            0xCB1B => i("RR E"),
            0xCB1C => i("RR H"),
            0xCB1D => i("RR L"),
            0xCB1E => i("RR (HL)"),
            0xCB1F => i("RR A"),

            0xCB40 => i("BIT 0, B").id(Bit).lhs(BitPos(0)).rhs(Reg8(B)),
            0xCB41 => i("BIT 0, C").id(Bit).lhs(BitPos(0)).rhs(Reg8(C)),
            0xCB42 => i("BIT 0, D").id(Bit).lhs(BitPos(0)).rhs(Reg8(D)),
            0xCB43 => i("BIT 0, E").id(Bit).lhs(BitPos(0)).rhs(Reg8(E)),
            0xCB44 => i("BIT 0, H").id(Bit).lhs(BitPos(0)).rhs(Reg8(H)),
            0xCB45 => i("BIT 0, L").id(Bit).lhs(BitPos(0)).rhs(Reg8(L)),
            0xCB46 => i("BIT 0, (HL)").id(Bit).lhs(BitPos(0)).rhs(Reg16Indir(HL)),
            0xCB47 => i("BIT 0, A").id(Bit).lhs(BitPos(0)).rhs(Reg8(A)),
            0xCB48 => i("BIT 1, B").id(Bit).lhs(BitPos(1)).rhs(Reg8(B)),
            0xCB49 => i("BIT 1, C").id(Bit).lhs(BitPos(1)).rhs(Reg8(C)),
            0xCB4A => i("BIT 1, D").id(Bit).lhs(BitPos(1)).rhs(Reg8(D)),
            0xCB4B => i("BIT 1, E").id(Bit).lhs(BitPos(1)).rhs(Reg8(E)),
            0xCB4C => i("BIT 1, H").id(Bit).lhs(BitPos(1)).rhs(Reg8(H)),
            0xCB4D => i("BIT 1, L").id(Bit).lhs(BitPos(1)).rhs(Reg8(L)),
            0xCB4E => i("BIT 1, (HL)").id(Bit).lhs(BitPos(0)).rhs(Reg16Indir(HL)),
            0xCB4F => i("BIT 1, A").id(Bit).lhs(BitPos(1)).rhs(Reg8(A)),

            0xCB50 => i("BIT 2, B").id(Bit).lhs(BitPos(2)).rhs(Reg8(B)),
            0xCB51 => i("BIT 2, C").id(Bit).lhs(BitPos(2)).rhs(Reg8(C)),
            0xCB52 => i("BIT 2, D").id(Bit).lhs(BitPos(2)).rhs(Reg8(D)),
            0xCB53 => i("BIT 2, E").id(Bit).lhs(BitPos(2)).rhs(Reg8(E)),
            0xCB54 => i("BIT 2, H").id(Bit).lhs(BitPos(2)).rhs(Reg8(H)),
            0xCB55 => i("BIT 2, L").id(Bit).lhs(BitPos(2)).rhs(Reg8(L)),
            0xCB56 => i("BIT 2, (HL)").id(Bit).lhs(BitPos(0)).rhs(Reg16Indir(HL)),
            0xCB57 => i("BIT 2, A").id(Bit).lhs(BitPos(2)).rhs(Reg8(A)),
            0xCB58 => i("BIT 3, B").id(Bit).lhs(BitPos(3)).rhs(Reg8(B)),
            0xCB59 => i("BIT 3, C").id(Bit).lhs(BitPos(3)).rhs(Reg8(C)),
            0xCB5A => i("BIT 3, D").id(Bit).lhs(BitPos(3)).rhs(Reg8(D)),
            0xCB5B => i("BIT 3, E").id(Bit).lhs(BitPos(3)).rhs(Reg8(E)),
            0xCB5C => i("BIT 3, H").id(Bit).lhs(BitPos(3)).rhs(Reg8(H)),
            0xCB5D => i("BIT 3, L").id(Bit).lhs(BitPos(3)).rhs(Reg8(L)),
            0xCB5E => i("BIT 3, (HL)").id(Bit).lhs(BitPos(0)).rhs(Reg16Indir(HL)),
            0xCB5F => i("BIT 3, A").id(Bit).lhs(BitPos(3)).rhs(Reg8(A)),

            0xCB60 => i("BIT 4, B").id(Bit).lhs(BitPos(4)).rhs(Reg8(B)),
            0xCB61 => i("BIT 4, C").id(Bit).lhs(BitPos(4)).rhs(Reg8(C)),
            0xCB62 => i("BIT 4, D").id(Bit).lhs(BitPos(4)).rhs(Reg8(D)),
            0xCB63 => i("BIT 4, E").id(Bit).lhs(BitPos(4)).rhs(Reg8(E)),
            0xCB64 => i("BIT 4, H").id(Bit).lhs(BitPos(4)).rhs(Reg8(H)),
            0xCB65 => i("BIT 4, L").id(Bit).lhs(BitPos(4)).rhs(Reg8(L)),
            0xCB66 => i("BIT 4, (HL)").id(Bit).lhs(BitPos(0)).rhs(Reg16Indir(HL)),
            0xCB67 => i("BIT 4, A").id(Bit).lhs(BitPos(4)).rhs(Reg8(A)),
            0xCB68 => i("BIT 5, B").id(Bit).lhs(BitPos(5)).rhs(Reg8(B)),
            0xCB69 => i("BIT 5, C").id(Bit).lhs(BitPos(5)).rhs(Reg8(C)),
            0xCB6A => i("BIT 5, D").id(Bit).lhs(BitPos(5)).rhs(Reg8(D)),
            0xCB6B => i("BIT 5, E").id(Bit).lhs(BitPos(5)).rhs(Reg8(E)),
            0xCB6C => i("BIT 5, H").id(Bit).lhs(BitPos(5)).rhs(Reg8(H)),
            0xCB6D => i("BIT 5, L").id(Bit).lhs(BitPos(5)).rhs(Reg8(L)),
            0xCB6E => i("BIT 5, (HL)").id(Bit).lhs(BitPos(0)).rhs(Reg16Indir(HL)),
            0xCB6F => i("BIT 5, A").id(Bit).lhs(BitPos(5)).rhs(Reg8(A)),
                                                                        
            0xCB70 => i("BIT 6, B").id(Bit).lhs(BitPos(6)).rhs(Reg8(B)),
            0xCB71 => i("BIT 6, C").id(Bit).lhs(BitPos(6)).rhs(Reg8(C)),
            0xCB72 => i("BIT 6, D").id(Bit).lhs(BitPos(6)).rhs(Reg8(D)),
            0xCB73 => i("BIT 6, E").id(Bit).lhs(BitPos(6)).rhs(Reg8(E)),
            0xCB74 => i("BIT 6, H").id(Bit).lhs(BitPos(6)).rhs(Reg8(H)),
            0xCB75 => i("BIT 6, L").id(Bit).lhs(BitPos(6)).rhs(Reg8(L)),
            0xCB76 => i("BIT 6, (HL)").id(Bit).lhs(BitPos(0)).rhs(Reg16Indir(HL)),
            0xCB77 => i("BIT 6, A").id(Bit).lhs(BitPos(6)).rhs(Reg8(A)),
            0xCB78 => i("BIT 7, B").id(Bit).lhs(BitPos(7)).rhs(Reg8(B)),
            0xCB79 => i("BIT 7, C").id(Bit).lhs(BitPos(7)).rhs(Reg8(C)),
            0xCB7A => i("BIT 7, D").id(Bit).lhs(BitPos(7)).rhs(Reg8(D)),
            0xCB7B => i("BIT 7, E").id(Bit).lhs(BitPos(7)).rhs(Reg8(E)),
            0xCB7C => i("BIT 7, H").id(Bit).lhs(BitPos(7)).rhs(Reg8(H)),
            0xCB7D => i("BIT 7, L").id(Bit).lhs(BitPos(7)).rhs(Reg8(L)),
            0xCB7E => i("BIT 7, (HL)").id(Bit).lhs(BitPos(0)).rhs(Reg16Indir(HL)),
            0xCB7F => i("BIT 7, A").id(Bit).lhs(BitPos(7)).rhs(Reg8(A)),

            _ => Instr::new(pos, "UNKNOWN"),
        }
    }
}

impl convert::From<u8> for Instr {
    fn from(pos: u8) -> Instr {
        use InstrKind::*;
        use Operand::*;
        use Reg16Kind::*;
        use Reg8Kind::*;

        let i = |tag| Instr::new(pos as u16, tag);

        match pos {
            0x00 => i("NOP").id(Nop),
            0x01 => i("LD BC, u16").id(LdWord).lhs(Reg16(BC)).rhs(U16),
            0x02 => i("LD (BC), A").id(Ld).lhs(Reg16Indir(BC)).rhs(Reg8(A)),
            0x03 => i("INC BC").id(Inc).rhs(Reg16(Reg16Kind::BC)),
            0x04 => i("INC B").id(Inc).rhs(Reg8(Reg8Kind::B)),
            0x05 => i("DEC B").id(Dec).rhs(Reg8(Reg8Kind::B)),
            0x06 => i("LD B, u8").id(Ld).lhs(Reg8(B)).rhs(U8),
            0x07 => i("RLCA"),
            0x08 => i("LD (u16), SP"),
            0x09 => i("ADD HL, BC").id(AddHl).rhs(Reg16(BC)),
            0x0A => i("LD A,(BC)"),
            0x0B => i("DEC BC").id(Dec).rhs(Reg16(Reg16Kind::BC)),
            0x0C => i("INC C").id(Inc).rhs(Reg8(Reg8Kind::C)),
            0x0D => i("DEC C").id(Dec).rhs(Reg8(Reg8Kind::C)),
            0x0E => i("LD C, u8").id(Ld).lhs(Reg8(C)).rhs(U8),
            0x0F => i("RRCA"),

            0x10 => i("STOP 0"),
            0x11 => i("LD DE, u16").id(LdWord).lhs(Reg16(DE)).rhs(U16),
            0x12 => i("LD (DE), A").id(Ld).lhs(Reg16Indir(DE)).rhs(Reg8(A)),
            0x13 => i("INC DE").id(Inc).rhs(Reg16(Reg16Kind::DE)),
            0x14 => i("INC D").id(Inc).rhs(Reg8(Reg8Kind::D)),
            0x15 => i("DEC D"),
            0x16 => i("LD D, u8").id(Ld).lhs(Reg8(D)).rhs(U8),
            0x17 => i("RLA").id(Rot).lhs(Reg8(A)).rhs(RotLeft).post_op(PostOp::CarryToB0),
            0x18 => i("JR i8").id(Jr).lhs(Cond(CondKind::Always)),
            0x19 => i("ADD HL, DE").id(AddHl).rhs(Reg16(DE)),
            0x1A => i("LD A, (DE)").id(Ld).lhs(Reg8(A)).rhs(Reg16Indir(DE)),
            0x1B => i("DEC DE"),
            0x1C => i("INC E").id(Inc).rhs(Reg8(Reg8Kind::E)),
            0x1D => i("DEC E"),
            0x1E => i("LD E, u8").id(Ld).lhs(Reg8(E)).rhs(U8),
            0x1F => i("RRA"),

            0x20 => i("JR NZ, i8").id(Jr).lhs(Cond(CondKind::NotZero)),
            0x21 => i("LD HL, u16").id(LdWord).lhs(Reg16(HL)).rhs(U16),
            0x22 => i("LD (HL+), A")
                .id(Ld)
                .lhs(Reg16Indir(HL))
                .rhs(Reg8(A))
                .post_op(PostOp::Inc(HL)),
            0x23 => i("INC HL").id(Inc).rhs(Reg16(Reg16Kind::HL)),
            0x24 => i("INC H").id(Inc).rhs(Reg8(Reg8Kind::H)),
            0x25 => i("DEC H"),
            0x26 => i("LD H, u8").id(Ld).lhs(Reg8(H)).rhs(U8),
            0x27 => i("DAA"),
            0x28 => i("JR Z, i8").id(Jr).lhs(Cond(CondKind::Zero)),
            0x29 => i("ADD HL, HL").id(AddHl).rhs(Reg16(HL)),
            0x2A => i("LD A, (HL+)"),
            0x2B => i("DEC HL"),
            0x2C => i("INC L").id(Inc).rhs(Reg8(Reg8Kind::L)),
            0x2D => i("DEC L"),
            0x2E => i("LD L, u8").id(Ld).lhs(Reg8(L)).rhs(U8),
            0x2F => i("CPL"),

            0x30 => i("JR NC, i8").id(Jr).lhs(Cond(CondKind::NotCarry)),
            0x31 => i("LD SP, u16").id(LdWord).lhs(Reg16(SP)).rhs(U16),
            0x32 => i("LD (HL-), A")
                .id(Ld)
                .lhs(Reg16Indir(HL))
                .rhs(Reg8(A))
                .post_op(PostOp::Dec(HL)),
            0x33 => i("INC SP").id(Inc).rhs(Reg16(Reg16Kind::SP)),
            0x34 => i("INC (HL)").id(Inc).rhs(Reg16Indir(Reg16Kind::HL)),
            0x35 => i("DEC (HL)"),
            0x36 => i("LD (HL), u8").id(Ld).lhs(Reg16Indir(HL)).rhs(U8),
            0x37 => i("SCF"),
            0x38 => i("JR C, i8").id(Jr).lhs(Cond(CondKind::Carry)),
            0x39 => i("ADD HL, SP").id(AddHl).rhs(Reg16(SP)),
            0x3A => i("LD A, (HL-)"),
            0x3B => i("DEC SP"),
            0x3C => i("INC A").id(Inc).rhs(Reg8(Reg8Kind::A)),
            0x3D => i("DEC A"),
            0x3E => i("LD A, u8").id(Ld).lhs(Reg8(A)).rhs(U8),
            0x3F => i("CCF"),

            0x40 => i("LD B, B").id(Ld).lhs(Reg8(B)).rhs(Reg8(B)),
            0x41 => i("LD B, C").id(Ld).lhs(Reg8(B)).rhs(Reg8(C)),
            0x42 => i("LD B, D").id(Ld).lhs(Reg8(B)).rhs(Reg8(D)),
            0x43 => i("LD B, E").id(Ld).lhs(Reg8(B)).rhs(Reg8(E)),
            0x44 => i("LD B, H").id(Ld).lhs(Reg8(B)).rhs(Reg8(H)),
            0x45 => i("LD B, L").id(Ld).lhs(Reg8(B)).rhs(Reg8(L)),
            0x46 => i("LD B, (HL)").id(Ld).lhs(Reg8(B)).rhs(Reg16Indir(HL)),
            0x47 => i("LD B, A").id(Ld).lhs(Reg8(B)).rhs(Reg8(A)),
            0x48 => i("LD C, B").id(Ld).lhs(Reg8(C)).rhs(Reg8(B)),
            0x49 => i("LD C, C").id(Ld).lhs(Reg8(C)).rhs(Reg8(C)),
            0x4A => i("LD C, D").id(Ld).lhs(Reg8(C)).rhs(Reg8(D)),
            0x4B => i("LD C, E").id(Ld).lhs(Reg8(C)).rhs(Reg8(E)),
            0x4C => i("LD C, H").id(Ld).lhs(Reg8(C)).rhs(Reg8(H)),
            0x4D => i("LD C, L").id(Ld).lhs(Reg8(C)).rhs(Reg8(L)),
            0x4E => i("LD C, (HL)").id(Ld).lhs(Reg8(C)).rhs(Reg16Indir(HL)),
            0x4F => i("LD C, A").id(Ld).lhs(Reg8(C)).rhs(Reg8(A)),

            0x50 => i("LD D, B").id(Ld).lhs(Reg8(D)).rhs(Reg8(B)),
            0x51 => i("LD D, C").id(Ld).lhs(Reg8(D)).rhs(Reg8(C)),
            0x52 => i("LD D, D").id(Ld).lhs(Reg8(D)).rhs(Reg8(D)),
            0x53 => i("LD D, E").id(Ld).lhs(Reg8(D)).rhs(Reg8(E)),
            0x54 => i("LD D, H").id(Ld).lhs(Reg8(D)).rhs(Reg8(H)),
            0x55 => i("LD D, L").id(Ld).lhs(Reg8(D)).rhs(Reg8(L)),
            0x56 => i("LD D, (HL)").id(Ld).lhs(Reg8(D)).rhs(Reg16Indir(HL)),
            0x57 => i("LD D, A").id(Ld).lhs(Reg8(D)).rhs(Reg8(A)),
            0x58 => i("LD E, B").id(Ld).lhs(Reg8(E)).rhs(Reg8(B)),
            0x59 => i("LD E, C").id(Ld).lhs(Reg8(E)).rhs(Reg8(C)),
            0x5A => i("LD E, D").id(Ld).lhs(Reg8(E)).rhs(Reg8(D)),
            0x5B => i("LD E, E").id(Ld).lhs(Reg8(E)).rhs(Reg8(E)),
            0x5C => i("LD E, H").id(Ld).lhs(Reg8(E)).rhs(Reg8(H)),
            0x5D => i("LD E, L").id(Ld).lhs(Reg8(E)).rhs(Reg8(L)),
            0x5E => i("LD E, (HL)").id(Ld).lhs(Reg8(E)).rhs(Reg16Indir(HL)),
            0x5F => i("LD E, A").id(Ld).lhs(Reg8(E)).rhs(Reg8(A)),

            0x60 => i("LD H, B").id(Ld).lhs(Reg8(H)).rhs(Reg8(B)),
            0x61 => i("LD H, C").id(Ld).lhs(Reg8(H)).rhs(Reg8(C)),
            0x62 => i("LD H, D").id(Ld).lhs(Reg8(H)).rhs(Reg8(D)),
            0x63 => i("LD H, E").id(Ld).lhs(Reg8(H)).rhs(Reg8(E)),
            0x64 => i("LD H, H").id(Ld).lhs(Reg8(H)).rhs(Reg8(H)),
            0x65 => i("LD H, L").id(Ld).lhs(Reg8(H)).rhs(Reg8(L)),
            0x66 => i("LD H, (HL)").id(Ld).lhs(Reg8(H)).rhs(Reg16Indir(HL)),
            0x67 => i("LD H, A").id(Ld).lhs(Reg8(H)).rhs(Reg8(A)),
            0x68 => i("LD L, B").id(Ld).lhs(Reg8(L)).rhs(Reg8(B)),
            0x69 => i("LD L, C").id(Ld).lhs(Reg8(L)).rhs(Reg8(C)),
            0x6A => i("LD L, D").id(Ld).lhs(Reg8(L)).rhs(Reg8(D)),
            0x6B => i("LD L, E").id(Ld).lhs(Reg8(L)).rhs(Reg8(E)),
            0x6C => i("LD L, H").id(Ld).lhs(Reg8(L)).rhs(Reg8(H)),
            0x6D => i("LD L, L").id(Ld).lhs(Reg8(L)).rhs(Reg8(L)),
            0x6E => i("LD L, (HL)").id(Ld).lhs(Reg8(L)).rhs(Reg16Indir(HL)),
            0x6F => i("LD L, A").id(Ld).lhs(Reg8(L)).rhs(Reg8(A)),

            0x70 => i("LD (HL), B").id(Ld).lhs(Reg16Indir(HL)).rhs(Reg8(B)),
            0x71 => i("LD (HL), B").id(Ld).lhs(Reg16Indir(HL)).rhs(Reg8(C)),
            0x72 => i("LD (HL), B").id(Ld).lhs(Reg16Indir(HL)).rhs(Reg8(D)),
            0x73 => i("LD (HL), B").id(Ld).lhs(Reg16Indir(HL)).rhs(Reg8(E)),
            0x74 => i("LD (HL), B").id(Ld).lhs(Reg16Indir(HL)).rhs(Reg8(H)),
            0x75 => i("LD (HL), B").id(Ld).lhs(Reg16Indir(HL)).rhs(Reg8(L)),
            0x76 => i("HALT").id(Halt),
            0x77 => i("LD (HL), A").id(Ld).lhs(Reg16Indir(HL)).rhs(Reg8(A)),
            0x78 => i("LD A, B").id(Ld).lhs(Reg8(A)).rhs(Reg8(B)),
            0x79 => i("LD A, C").id(Ld).lhs(Reg8(A)).rhs(Reg8(C)),
            0x7A => i("LD A, D").id(Ld).lhs(Reg8(A)).rhs(Reg8(D)),
            0x7B => i("LD A, E").id(Ld).lhs(Reg8(A)).rhs(Reg8(E)),
            0x7C => i("LD A, H").id(Ld).lhs(Reg8(A)).rhs(Reg8(H)),
            0x7D => i("LD A, L").id(Ld).lhs(Reg8(A)).rhs(Reg8(L)),
            0x7E => i("LD A, (HL)").id(Ld).lhs(Reg8(A)).rhs(Reg16Indir(HL)),
            0x7F => i("LD A, A").id(Ld).lhs(Reg8(A)).rhs(Reg8(A)),

            0x80 => i("ADD A, B").id(Add).rhs(Reg8(B)),
            0x81 => i("ADD A, C").id(Add).rhs(Reg8(C)),
            0x82 => i("ADD A, D").id(Add).rhs(Reg8(D)),
            0x83 => i("ADD A, E").id(Add).rhs(Reg8(E)),
            0x84 => i("ADD A, H").id(Add).rhs(Reg8(H)),
            0x85 => i("ADD A, L").id(Add).rhs(Reg8(L)),
            0x86 => i("ADD A, (HL)"),
            0x87 => i("ADD A, A").id(Add).rhs(Reg8(A)),
            0x88 => i("ADC A, B").id(Adc).rhs(Reg8(B)),
            0x89 => i("ADC A, C").id(Adc).rhs(Reg8(C)),
            0x8A => i("ADC A, D").id(Adc).rhs(Reg8(D)),
            0x8B => i("ADC A, E").id(Adc).rhs(Reg8(E)),
            0x8C => i("ADC A, H").id(Adc).rhs(Reg8(H)),
            0x8D => i("ADC A, L").id(Adc).rhs(Reg8(L)),
            0x8E => i("ADC A, (HL)"),
            0x8F => i("ADC A, A").id(Adc).rhs(Reg8(A)),

            0x90 => i("SUB A, B").id(Sub).rhs(Reg8(B)),
            0x91 => i("SUB A, C").id(Sub).rhs(Reg8(C)),
            0x92 => i("SUB A, D").id(Sub).rhs(Reg8(D)),
            0x93 => i("SUB A, E").id(Sub).rhs(Reg8(E)),
            0x94 => i("SUB A, H").id(Sub).rhs(Reg8(H)),
            0x95 => i("SUB A, L").id(Sub).rhs(Reg8(L)),
            0x96 => i("SUB A, (HL)"),
            0x97 => i("SUB A, A").id(Sub).rhs(Reg8(A)),
            0x98 => i("SBC A, B").id(Sbc).rhs(Reg8(B)),
            0x99 => i("SBC A, C").id(Sbc).rhs(Reg8(C)),
            0x9A => i("SBC A, D").id(Sbc).rhs(Reg8(D)),
            0x9B => i("SBC A, E").id(Sbc).rhs(Reg8(E)),
            0x9C => i("SBC A, H").id(Sbc).rhs(Reg8(H)),
            0x9D => i("SBC A, L").id(Sbc).rhs(Reg8(L)),
            0x9E => i("SBC A, (HL)"),
            0x9F => i("SBC A, A").id(Sbc).rhs(Reg8(A)),

            0xA0 => i("AND A, B").id(And).rhs(Reg8(B)),
            0xA1 => i("AND A, C").id(And).rhs(Reg8(C)),
            0xA2 => i("AND A, D").id(And).rhs(Reg8(D)),
            0xA3 => i("AND A, E").id(And).rhs(Reg8(E)),
            0xA4 => i("AND A, H").id(And).rhs(Reg8(H)),
            0xA5 => i("AND A, L").id(And).rhs(Reg8(L)),
            0xA6 => i("AND A, (HL)"),
            0xA7 => i("AND A, A").id(And).rhs(Reg8(A)),
            0xA8 => i("XOR A, B").id(Xor).rhs(Reg8(B)),
            0xA9 => i("XOR A, C").id(Xor).rhs(Reg8(C)),
            0xAA => i("XOR A, D").id(Xor).rhs(Reg8(D)),
            0xAB => i("XOR A, E").id(Xor).rhs(Reg8(E)),
            0xAC => i("XOR A, H").id(Xor).rhs(Reg8(H)),
            0xAD => i("XOR A, L").id(Xor).rhs(Reg8(L)),
            0xAE => i("XOR A, (HL)"),
            0xAF => i("XOR A, A").id(Xor).rhs(Reg8(A)),

            0xB0 => i("OR A, B").id(Or).rhs(Reg8(B)),
            0xB1 => i("OR A, C").id(Or).rhs(Reg8(C)),
            0xB2 => i("OR A, D").id(Or).rhs(Reg8(D)),
            0xB3 => i("OR A, E").id(Or).rhs(Reg8(E)),
            0xB4 => i("OR A, H").id(Or).rhs(Reg8(H)),
            0xB5 => i("OR A, L").id(Or).rhs(Reg8(L)),
            0xB6 => i("OR A, (H)"),
            0xB7 => i("OR A, A").id(Or).rhs(Reg8(A)),
            0xB8 => i("CP A, B"),
            0xB9 => i("CP A, C"),
            0xBA => i("CP A, D"),
            0xBB => i("CP A, E"),
            0xBC => i("CP A, H"),
            0xBD => i("CP A, L"),
            0xBE => i("CP A, (HL)"),
            0xBF => i("CP A, A"),

            0xC0 => i("RET NZ").id(Ret).lhs(Cond(CondKind::NotZero)),
            0xC1 => i("POP BC").id(Pop).rhs(Reg16(BC)),
            0xC2 => i("JP NZ, u16").id(Jp).lhs(Cond(CondKind::NotZero)),
            0xC3 => i("JP u16").id(Jp).lhs(Cond(CondKind::Always)).rhs(U16),
            0xC4 => i("CALL NZ, u16")
                .id(Call)
                .lhs(Cond(CondKind::NotZero))
                .rhs(U16),
            0xC5 => i("PUSH BC").id(Push).rhs(Reg16(BC)),
            0xC6 => i("ADD A, u8"),
            0xC7 => i("RST 00h"),
            0xC8 => i("RET Z").id(Ret).lhs(Cond(CondKind::Zero)),
            0xC9 => i("RET").id(Ret).lhs(Cond(CondKind::Always)),
            0xCA => i("JP Z, u16").id(Jp).lhs(Cond(CondKind::Zero)),
            0xCB => i("PREFIX CB"),
            0xCC => i("CALL Z, u16").id(Call).lhs(Cond(CondKind::Zero)),
            0xCD => i("CALL u16").id(Call).lhs(Cond(CondKind::Always)),
            0xCE => i("ADC A, u8"),
            0xCF => i("RST 08h"),

            0xD0 => i("RET NZ").id(Ret).lhs(Cond(CondKind::NotCarry)),
            0xD1 => i("POP DE").id(Pop).rhs(Reg16(DE)),
            0xD2 => i("JP NC, u16").id(Jp).lhs(Cond(CondKind::NotCarry)),
            0xD4 => i("CALL NC, u16").id(Call).lhs(Cond(CondKind::NotCarry)),
            0xD5 => i("PUSH DE").id(Push).rhs(Reg16(DE)),
            0xD6 => i("SUB A, u8"),
            0xD7 => i("RST 10h"),
            0xD8 => i("RET C").id(Ret).lhs(Cond(CondKind::Carry)),
            0xD9 => i("RETI"),
            0xDA => i("JP C, u16").id(Jp).lhs(Cond(CondKind::Carry)),
            0xDC => i("CALL C, u16").id(Call).lhs(Cond(CondKind::Carry)),
            0xDE => i("SBC A, u8"),
            0xDF => i("RST 18h"),

            0xE0 => i("LD (FF00+u8), A").id(Ld).lhs(U8Indir(0xFF00)).rhs(Reg8(A)),
            0xE1 => i("POP HL").id(Pop).rhs(Reg16(HL)),
            0xE2 => i("LD (FF00+C), A")
                .id(Ld)
                .lhs(Reg8Indir(Reg8Kind::C, 0xFF00))
                .rhs(Reg8(A)),
            0xE5 => i("PUSH HL").id(Push).rhs(Reg16(HL)),
            0xE6 => i("AND A, u8"),
            0xE7 => i("RST 20h"),
            0xE8 => i("ADD SP, i8"),
            0xE9 => i("JP HL"),
            0xEA => i("LD (u16), A"),
            0xEE => i("XOR A, u8"),
            0xEF => i("RST 28h"),

            0xF0 => i("LD A, (FF00+u8)"),
            0xF1 => i("POP AF"),
            0xF2 => i("LD A, (FF00+C)"),
            0xF3 => i("DI"),
            0xF5 => i("PUSH AF"),
            0xF6 => i("OR A, u8"),
            0xF7 => i("RST 30h"),
            0xF8 => i("LD HL, SP+i8"),
            0xF9 => i("LD SP, HL").id(LdWord).lhs(Reg16(SP)).rhs(Reg16(HL)),
            0xFA => i("LD A, (u16)"),
            0xFB => i("EI"),
            0xFE => i("CP A, u8"),
            0xFF => i("RST 38h"),

            _ => Instr::new(pos as u16, "UNKNOWN"),
        }
    }
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pos = format!("0x{:0>4X}", self.pos);

        let (lhs, rhs) = self.trace.unwrap_or((0, 0));

        if lhs == 0 && rhs == 0 {
            write!(f, "[{} | {:^15}]", pos, self.tag)
        } else {
            write!(f, "[{} | {:^15}] lhs: 0x{:0>4X}, rhs: 0x{:0>4X}", pos, self.tag, lhs, rhs)
        }
    }
}
