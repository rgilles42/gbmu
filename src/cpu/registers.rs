use super::memory_bus::MemoryBus;

const ZERO_FLAG_BYTE_POS:		u8 = 7;
const SUBSTRACT_FLAG_BYTE_POS:	u8 = 6;
const HALF_CARRY_FLAG_BYTE_POS:	u8 = 5;
const CARRY_FLAG_BYTE_POS:		u8 = 4;

#[derive(Clone, Copy)]
pub struct FlagsRegister {
	pub zero: bool,
	pub substract: bool,
	pub half_carry: bool,
	pub carry: bool
}

impl std::convert::From<FlagsRegister> for u8  {
	fn from(flag: FlagsRegister) -> u8 {
		(if flag.zero		{ 1 } else { 0 }) << ZERO_FLAG_BYTE_POS |
		(if flag.substract	{ 1 } else { 0 }) << SUBSTRACT_FLAG_BYTE_POS |
		(if flag.half_carry	{ 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POS |
		(if flag.carry		{ 1 } else { 0 }) << CARRY_FLAG_BYTE_POS
	}
}

impl std::convert::From<u8> for FlagsRegister {
	fn from(byte: u8) -> Self {
		let zero		= (byte & (1 << ZERO_FLAG_BYTE_POS)) != 0;
		let substract	= (byte & (1 << SUBSTRACT_FLAG_BYTE_POS)) != 0;
		let half_carry= (byte & (1 << HALF_CARRY_FLAG_BYTE_POS)) != 0;
		let carry		= (byte & (1 << CARRY_FLAG_BYTE_POS)) != 0;
		FlagsRegister {
			zero,
			substract,
			half_carry,
			carry
		}
	}
}

pub struct Registers {
	pub a: u8,
	pub f: FlagsRegister,
	pub b: u8,
	pub c: u8,
	pub d: u8,
	pub e: u8,
	pub h: u8,
	pub l: u8,
	pub program_counter: u16,
//	pub stack_pointer: u16,
}

impl Registers {
	// pub fn get_af(&self) -> u16 {
	// 	(self.a as u16) << 8 | u8::from(self.f) as u16
	// }
	// pub fn get_bc(&self) -> u16 {
	// 	(self.b as u16) << 8 | self.c as u16
	// }
	// pub fn get_de(&self) -> u16 {
	// 	(self.d as u16) << 8 | self.e as u16
	// }
	pub fn get_hl(&self) -> u16 {
		(self.h as u16) << 8 | self.l as u16
	}
	pub fn get_hl_pointee(&self, memory_bus: &MemoryBus) -> u8 {
		memory_bus.read_byte(self.get_hl())
	}

	// pub fn set_af(&mut self, value: u16) {
	// 	self.a = (value >> 8) as u8;
	// 	self.f = (value as u8).into();
	// }
	// pub fn set_bc(&mut self, value: u16) {
	// 	self.b = (value >> 8) as u8;
	// 	self.c = value as u8;
	// }
	// pub fn set_de(&mut self, value: u16) {
	// 	self.d = (value >> 8) as u8;
	// 	self.e = value as u8;
	// }
	// pub fn set_hl(&mut self, value: u16) {
	// 	self.h = (value >> 8) as u8;
	// 	self.l = value as u8;
	// }
	// pub fn set_hl_poinee(data: u8) {}
}