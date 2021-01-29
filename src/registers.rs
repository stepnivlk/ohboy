#[derive(Debug)]
pub struct FlagsRegister {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

impl FlagsRegister {
    fn new() -> Self {
        Self {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: false,
        }
    }
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION
            | (if flag.subtract { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION
            | (if flag.half_carry { 1 } else { 0 })
                << HALF_CARRY_FLAG_BYTE_POSITION
            | (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> FlagsRegister {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}

#[derive(Debug)]
pub enum Reg8Kind {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug)]
pub enum Reg16Kind {
    BC,
    DE,
    HL,
    SP,
}

#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagsRegister,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagsRegister::new(),
            h: 0,
            l: 0,
        }
    }

    pub fn get(&self, reg: Reg8Kind) -> u8 {
        match reg {
            Reg8Kind::A => self.a,
            Reg8Kind::B => self.b,
            Reg8Kind::C => self.c,
            Reg8Kind::D => self.d,
            Reg8Kind::E => self.e,
            Reg8Kind::H => self.h,
            Reg8Kind::L => self.l,
        }
    }

    pub fn get_word(&self, reg: Reg16Kind) -> u16 {
        match reg {
            Reg16Kind::SP => 0xFFFF,
            Reg16Kind::BC => self.get_bc(),
            Reg16Kind::DE => self.get_de(),
            Reg16Kind::HL => self.get_hl(),
        }
    }

    pub fn add(&self, reg: Reg8Kind, val: u8) -> (u8, bool) {
        let reg_val = match reg {
            Reg8Kind::A => self.a,
            Reg8Kind::B => self.b,
            Reg8Kind::C => self.c,
            Reg8Kind::D => self.d,
            Reg8Kind::E => self.e,
            Reg8Kind::H => self.h,
            Reg8Kind::L => self.l,
        };

        reg_val.overflowing_add(val)
    }

    pub fn set(&self, reg: Reg8Kind, val: u8) {
        match reg {
            Reg8Kind::A => self.a = val,
            Reg8Kind::B => self.b = val,
            Reg8Kind::C => self.c = val,
            Reg8Kind::D => self.d = val,
            Reg8Kind::E => self.e = val,
            Reg8Kind::H => self.h = val,
            Reg8Kind::L => self.l = val,
        }
    }

    pub fn get_bc(&self) -> u16 {
        self.merge(self.b, self.c)
    }

    pub fn set_bc(&mut self, value: u16) {
        let (h, l) = self.split(value);

        self.b = h;
        self.c = l;
    }

    pub fn get_de(&self) -> u16 {
        self.merge(self.d, self.e)
    }

    pub fn set_de(&mut self, value: u16) {
        let (h, l) = self.split(value);

        self.d = h;
        self.e = l;
    }

    pub fn get_hl(&self) -> u16 {
        self.merge(self.h, self.l)
    }

    pub fn set_hl(&mut self, value: u16) {
        let (h, l) = self.split(value);

        self.h = h;
        self.l = l;
    }

    fn split(&self, value: u16) -> (u8, u8) {
        (((value & 0xFF00) >> 8) as u8, (value & 0xFF) as u8)
    }

    fn merge(&self, h: u8, l: u8) -> u16 {
        (h as u16) << 8 | l as u16
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const VAL: u16 = 0x12ab;

    #[test]
    fn it_sets_and_gets_bc() {
        let mut registers = Registers::new();

        registers.set_bc(VAL);

        assert_eq!(registers.get_bc(), VAL);
    }

    #[test]
    fn it_sets_and_gets_de() {
        let mut registers = Registers::new();

        registers.set_de(VAL);

        assert_eq!(registers.get_de(), VAL);
    }

    #[test]
    fn it_sets_and_gets_hl() {
        let mut registers = Registers::new();

        registers.set_hl(VAL);

        assert_eq!(registers.get_hl(), VAL);
    }
}
