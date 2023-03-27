const ZERO_FLAG_BYTE_POS:		u8 = 7;
const SUBSTRACT_FLAG_BYTE_POS:	u8 = 6;
const HALF_CARRY_FLAG_BYTE_POS:	u8 = 5;
const CARRY_FLAG_BYTE_POS:		u8 = 4;

struct FlagsRegister {
	zero: bool,
	substract: bool,
	half_carry: bool,
	carry: bool
}

impl std::convert::From<FlagsRegister> for u8  {
	fn from(flag: FlagsRegister) -> u8 {
		(if flag.zero       { 1 } else { 0 }) << ZERO_FLAG_BYTE_POS |
		(if flag.subtract   { 1 } else { 0 }) << SUBSTRACT_FLAG_BYTE_POS |
		(if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POS |
		(if flag.carry      { 1 } else { 0 }) << CARRY_FLAG_BYTE_POS
	}
}

impl std::convert::From<u8> for FlagsRegister {
	fn from(byte: u8) -> Self {
		let zero_flag	= (byte & (1 << ZERO_FLAG_BYTE_POS)) != 0;
		let subs_flag	= (byte & (1 << SUBSTRACT_FLAG_BYTE_POS)) != 0;
		let h_carry		= (byte & (1 << HALF_CARRY_FLAG_BYTE_POS)) != 0;
		let carry_flag	= (byte & (1 << CARRY_FLAG_BYTE_POS)) != 0;
		FlagsRegister {
			zero_flag,
			subs_flag,
			h_carry,
			carry_flag
		}
	}
}

struct Registers {
	a: u8,
	b: u8,
	c: u8,
	d: u8,
	e: u8,
	f: FlagsRegister,
	h: u8,
	l: u8,
}

impl Registers {
	fn get_bc(&self) -> u16 {
		(self.b as u16) << 8 | self.c as u16
	}
	fn set_bc(&mut self, value: u16) {
		self.b = (value >> 8) as u8;
		self.c = value as u8;
	}
}