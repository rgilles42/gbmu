use crate::memory_bus::MemoryBus;
use std::{fmt::Debug, ops::Sub};

const FLAG_Z_BYTE_POS:	u8 = 7;
const FLAG_N_BYTE_POS:	u8 = 6;
const FLAG_H_BYTE_POS:	u8 = 5;
const FLAG_C_BYTE_POS:	u8 = 4;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlagsRegister {
	pub zero: bool,
	pub substract: bool,
	pub half_carry: bool,
	pub carry: bool
}

impl std::convert::From<FlagsRegister> for u8  {
	fn from(flag: FlagsRegister) -> u8 {
		(if flag.zero		{ 1 } else { 0 }) << FLAG_Z_BYTE_POS |
		(if flag.substract	{ 1 } else { 0 }) << FLAG_N_BYTE_POS |
		(if flag.half_carry	{ 1 } else { 0 }) << FLAG_H_BYTE_POS |
		(if flag.carry		{ 1 } else { 0 }) << FLAG_C_BYTE_POS
	}
}

impl std::convert::From<u8> for FlagsRegister {
	fn from(byte: u8) -> Self {
		let zero		= (byte & (1 << FLAG_Z_BYTE_POS)) != 0;
		let substract	= (byte & (1 << FLAG_N_BYTE_POS)) != 0;
		let half_carry= (byte & (1 << FLAG_H_BYTE_POS)) != 0;
		let carry		= (byte & (1 << FLAG_C_BYTE_POS)) != 0;
		FlagsRegister {
			zero,
			substract,
			half_carry,
			carry
		}
	}
}

//#[derive(Debug)]
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
	pub stack_pointer: u16,
}

/* Though the GB MCU is an 8-bit, little-endian CPU, the register pairs are assembled and expressed as big-endian... */

impl Registers {
	pub fn new() -> Self {
		Registers {a: 0, b: 0, c: 0, d: 0, e: 0, f: 0.into(), h: 0, l: 0, program_counter: 0x0000, stack_pointer : 0x0000}
	}
	// pub fn init(&mut self) {
	// 	self.program_counter = 0x100;
	// 	self.set_af_big_endian(0x01B0);			// GB/SGB - 0x01B0; GBP - 0xFFB0; GBC - 0x11B0
	// 	self.set_bc_big_endian(0x0013);
	// 	self.set_de_big_endian(0x00D8);
	// 	self.set_hl_big_endian(0x014D);
	// 	self.stack_pointer = 0xFFFE;
	// }
	pub fn get_af_big_endian(&self) -> u16 {
		(self.a as u16) << 8 | u8::from(self.f) as u16
	}
	pub fn get_bc_big_endian(&self) -> u16 {
		(self.b as u16) << 8 | self.c as u16
	}
	pub fn get_de_big_endian(&self) -> u16 {
		(self.d as u16) << 8 | self.e as u16
	}
	pub fn get_hl_big_endian(&self) -> u16 {
		(self.h as u16) << 8 | self.l as u16
	}
	pub fn set_af_big_endian(&mut self, big_endian_value: u16) {
		self.a = (big_endian_value >> 8) as u8;
		self.f = (big_endian_value as u8).into();
	}
	pub fn set_bc_big_endian(&mut self, big_endian_value: u16) {
		self.b = (big_endian_value >> 8) as u8;
		self.c = big_endian_value as u8;
	}
	pub fn set_de_big_endian(&mut self, big_endian_value: u16) {
		self.d = (big_endian_value >> 8) as u8;
		self.e = big_endian_value as u8;
	}
	pub fn set_hl_big_endian(&mut self, big_endian_value: u16) {
		self.h = (big_endian_value >> 8) as u8;
		self.l = big_endian_value as u8;
	}
	// pub fn get_af_little_endian(&self) -> u16 {
	// 	(u8::from(self.f) as u16) << 8 | self.a as u16
	// }
	// pub fn get_bc_little_endian(&self) -> u16 {
	// 	(self.c as u16) << 8 | self.b as u16
	// }
	// pub fn get_de_little_endian(&self) -> u16 {
	// 	(self.e as u16) << 8 | self.d as u16
	// }
	// pub fn get_hl_little_endian(&self) -> u16 {
	// 	(self.l as u16) << 8 | self.h as u16
	// }
	// pub fn set_af_little_endian(&mut self, little_endian_value: u16) {
	// 	self.a = little_endian_value as u8;
	// 	self.f = ((little_endian_value >> 8) as u8).into();
	// }
	// pub fn set_bc_little_endian(&mut self, little_endian_value: u16) {
	// 	self.b = little_endian_value as u8;
	// 	self.c = (little_endian_value >> 8) as u8;
	// }
	// pub fn set_de_little_endian(&mut self, little_endian_value: u16) {
	// 	self.d = little_endian_value as u8;
	// 	self.e = (little_endian_value >> 8) as u8;
	// }
	// pub fn set_hl_little_endian(&mut self, little_endian_value: u16) {
	// 	self.h = little_endian_value as u8;
	// 	self.l = (little_endian_value >> 8) as u8;
	// }
	pub fn get_bc_pointee(&self, memory_bus: &MemoryBus) -> u8 {
		memory_bus.read_byte(self.get_bc_big_endian())
	}
	pub fn get_de_pointee(&self, memory_bus: &MemoryBus) -> u8 {
		memory_bus.read_byte(self.get_de_big_endian())
	}
	pub fn get_hl_pointee(&self, memory_bus: &MemoryBus) -> u8 {
		memory_bus.read_byte(self.get_hl_big_endian())
	}
	pub fn set_bc_pointee(&self, memory_bus: &mut MemoryBus, data: u8) {
		memory_bus.write_byte(self.get_bc_big_endian(), data)
	}
	pub fn set_de_pointee(&self, memory_bus: &mut MemoryBus, data: u8) {
		memory_bus.write_byte(self.get_de_big_endian(), data)
	}
	pub fn set_hl_pointee(&self, memory_bus: &mut MemoryBus, data: u8) {
		memory_bus.write_byte(self.get_hl_big_endian(), data)
	}
}

impl Debug for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Registers").field("program_counter - 1", &self.program_counter.sub(1)).field("a", &self.a).field("bc", &self.get_bc_big_endian()).field("de", &self.get_de_big_endian()).field("hl", &self.get_hl_big_endian()).finish()
    }
}