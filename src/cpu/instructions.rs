use super::Cpu;

type InstrLength = u8;
type InstrCycles = u8;

#[derive(Debug, Clone, Copy)]
pub enum ArithmeticOperand{
	RegA, RegB, RegC, RegD, RegE, RegH, RegL, HLPointee, ByteFromPC
}
#[derive(Debug, Clone, Copy)]
pub enum LargeArithmeticOperand{
	RegsBC, RegsDE, RegsHL, RegSP
}
#[derive(Debug, Clone, Copy)]
pub enum Regs{
	RegA, RegB, RegC, RegD, RegE, RegH, RegL, HLPointee, BCPointee, DEPointee, BytesFromPCPointee, UpperRamOffsetFromPC, UpperRamOffsetFromRegC, ByteFromPC
}
#[derive(Debug, Clone, Copy)]
pub enum RegPairs{
	RegsBC, RegsDE, RegsHL, RegSP
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
	NOP(InstrLength, InstrCycles),
	LD(InstrLength, InstrCycles, Regs, Regs),
	LDI(InstrLength, InstrCycles, Regs, Regs),
	LDD(InstrLength, InstrCycles, Regs, Regs),
	INCs(InstrLength, InstrCycles, Regs),
	INCss(InstrLength, InstrCycles, RegPairs),
	DECs(InstrLength, InstrCycles, Regs),
	DECss(InstrLength, InstrCycles, RegPairs),
	ADDAs(InstrLength, InstrCycles, ArithmeticOperand),
	ADDHLss(InstrLength, InstrCycles, LargeArithmeticOperand),
	ADDSPe(InstrLength, InstrCycles),
	ADCAs(InstrLength, InstrCycles, ArithmeticOperand),
	SUBs(InstrLength, InstrCycles, ArithmeticOperand),
	SBCAs(InstrLength, InstrCycles, ArithmeticOperand),
	ANDs(InstrLength, InstrCycles, ArithmeticOperand),
	XORs(InstrLength, InstrCycles, ArithmeticOperand),
	ORs(InstrLength, InstrCycles, ArithmeticOperand),
	CPs(InstrLength, InstrCycles, ArithmeticOperand),
	CCF(InstrLength, InstrCycles),
	SCF(InstrLength, InstrCycles),
	DAA(InstrLength, InstrCycles),
	CPL(InstrLength, InstrCycles),
}

impl Instruction {
	pub fn from_opcode(opcode: u8) -> Option<Instruction> {
		match opcode {
			0x00 => Some(Instruction::NOP(1, 4)),
			0x02 => Some(Instruction::LD(1, 8, Regs::BCPointee, Regs::RegA)),
			0x03 => Some(Instruction::INCss(1, 8, RegPairs::RegsBC)),
			0x04 => Some(Instruction::INCs(1, 4, Regs::RegB)),
			0x05 => Some(Instruction::DECs(1, 4, Regs::RegB)),
			0x06 => Some(Instruction::LD(2, 8, Regs::RegB, Regs::ByteFromPC)),
			0x09 => Some(Instruction::ADDHLss(1, 8, LargeArithmeticOperand::RegsBC)),
			0x0A => Some(Instruction::LD(1, 8, Regs::RegA, Regs::BCPointee)),
			0x0B => Some(Instruction::DECss(1, 8, RegPairs::RegsBC)),
			0x0C => Some(Instruction::INCs(1, 4, Regs::RegC)),
			0x0D => Some(Instruction::DECs(1, 4, Regs::RegC)),
			0x0E => Some(Instruction::LD(2, 8, Regs::RegC, Regs::ByteFromPC)),
			0x12 => Some(Instruction::LD(1, 8, Regs::DEPointee, Regs::RegA)),
			0x13 => Some(Instruction::INCss(1, 8, RegPairs::RegsDE)),
			0x14 => Some(Instruction::INCs(1, 4, Regs::RegD)),
			0x15 => Some(Instruction::DECs(1, 4, Regs::RegD)),
			0x16 => Some(Instruction::LD(2, 8, Regs::RegD, Regs::ByteFromPC)),
			0x19 => Some(Instruction::ADDHLss(1, 8, LargeArithmeticOperand::RegsDE)),
			0x1A => Some(Instruction::LD(1, 8, Regs::RegA, Regs::DEPointee)),
			0x1B => Some(Instruction::DECss(1, 8, RegPairs::RegsDE)),
			0x1C => Some(Instruction::INCs(1, 4, Regs::RegE)),
			0x1D => Some(Instruction::DECs(1, 4, Regs::RegE)),
			0x1E => Some(Instruction::LD(2, 8, Regs::RegE, Regs::ByteFromPC)),
			0x22 => Some(Instruction::LDI(1, 8, Regs::HLPointee, Regs::RegA)),
			0x23 => Some(Instruction::INCss(1, 8, RegPairs::RegsHL)),
			0x24 => Some(Instruction::INCs(1, 4, Regs::RegH)),
			0x25 => Some(Instruction::DECs(1, 4, Regs::RegH)),
			0x26 => Some(Instruction::LD(2, 8, Regs::RegH, Regs::ByteFromPC)),
			0x27 => Some(Instruction::DAA(1, 4)),
			0x29 => Some(Instruction::ADDHLss(1, 8, LargeArithmeticOperand::RegsHL)),
			0x2A => Some(Instruction::LDI(1, 8, Regs::RegA, Regs::HLPointee)),
			0x2B => Some(Instruction::DECss(1, 8, RegPairs::RegsHL)),
			0x2C => Some(Instruction::INCs(1, 4, Regs::RegL)),
			0x2D => Some(Instruction::DECs(1, 4, Regs::RegL)),
			0x2E => Some(Instruction::LD(2, 8, Regs::RegL, Regs::ByteFromPC)),
			0x2F => Some(Instruction::CPL(1, 4)),
			0x32 => Some(Instruction::LDD(1, 8, Regs::HLPointee, Regs::RegA)),
			0x33 => Some(Instruction::INCss(1, 8, RegPairs::RegSP)),
			0x34 => Some(Instruction::INCs(1, 12, Regs::HLPointee)),
			0x35 => Some(Instruction::DECs(1, 12, Regs::HLPointee)),
			0x36 => Some(Instruction::LD(2, 12, Regs::HLPointee, Regs::ByteFromPC)),
			0x37 => Some(Instruction::SCF(1, 4)),
			0x39 => Some(Instruction::ADDHLss(1, 8, LargeArithmeticOperand::RegSP)),
			0x3A => Some(Instruction::LDD(1, 8, Regs::RegA, Regs::HLPointee)),
			0x3B => Some(Instruction::DECss(1, 8, RegPairs::RegSP)),
			0x3C => Some(Instruction::INCs(1, 4, Regs::RegA)),
			0x3D => Some(Instruction::DECs(1, 4, Regs::RegA)),
			0x3E => Some(Instruction::LD(2, 8, Regs::RegA, Regs::ByteFromPC)),
			0x3F => Some(Instruction::CCF(1, 4)),
			0x40 => Some(Instruction::LD(1, 4, Regs::RegB, Regs::RegB)),
			0x41 => Some(Instruction::LD(1, 4, Regs::RegB, Regs::RegC)),
			0x42 => Some(Instruction::LD(1, 4, Regs::RegB, Regs::RegD)),
			0x43 => Some(Instruction::LD(1, 4, Regs::RegB, Regs::RegE)),
			0x44 => Some(Instruction::LD(1, 4, Regs::RegB, Regs::RegH)),
			0x45 => Some(Instruction::LD(1, 4, Regs::RegB, Regs::RegL)),
			0x46 => Some(Instruction::LD(1, 8, Regs::RegB, Regs::HLPointee)),
			0x47 => Some(Instruction::LD(1, 4, Regs::RegB, Regs::RegA)),
			0x48 => Some(Instruction::LD(1, 4, Regs::RegC, Regs::RegB)),
			0x49 => Some(Instruction::LD(1, 4, Regs::RegC, Regs::RegC)),
			0x4A => Some(Instruction::LD(1, 4, Regs::RegC, Regs::RegD)),
			0x4B => Some(Instruction::LD(1, 4, Regs::RegC, Regs::RegE)),
			0x4C => Some(Instruction::LD(1, 4, Regs::RegC, Regs::RegH)),
			0x4D => Some(Instruction::LD(1, 4, Regs::RegC, Regs::RegL)),
			0x4E => Some(Instruction::LD(1, 8, Regs::RegC, Regs::HLPointee)),
			0x4F => Some(Instruction::LD(1, 4, Regs::RegC, Regs::RegA)),
			0x50 => Some(Instruction::LD(1, 4, Regs::RegD, Regs::RegB)),
			0x51 => Some(Instruction::LD(1, 4, Regs::RegD, Regs::RegC)),
			0x52 => Some(Instruction::LD(1, 4, Regs::RegD, Regs::RegD)),
			0x53 => Some(Instruction::LD(1, 4, Regs::RegD, Regs::RegE)),
			0x54 => Some(Instruction::LD(1, 4, Regs::RegD, Regs::RegH)),
			0x55 => Some(Instruction::LD(1, 4, Regs::RegD, Regs::RegL)),
			0x56 => Some(Instruction::LD(1, 8, Regs::RegD, Regs::HLPointee)),
			0x57 => Some(Instruction::LD(1, 4, Regs::RegD, Regs::RegA)),
			0x58 => Some(Instruction::LD(1, 4, Regs::RegE, Regs::RegB)),
			0x59 => Some(Instruction::LD(1, 4, Regs::RegE, Regs::RegC)),
			0x5A => Some(Instruction::LD(1, 4, Regs::RegE, Regs::RegD)),
			0x5B => Some(Instruction::LD(1, 4, Regs::RegE, Regs::RegE)),
			0x5C => Some(Instruction::LD(1, 4, Regs::RegE, Regs::RegH)),
			0x5D => Some(Instruction::LD(1, 4, Regs::RegE, Regs::RegL)),
			0x5E => Some(Instruction::LD(1, 8, Regs::RegE, Regs::HLPointee)),
			0x5F => Some(Instruction::LD(1, 4, Regs::RegE, Regs::RegA)),
			0x60 => Some(Instruction::LD(1, 4, Regs::RegH, Regs::RegB)),
			0x61 => Some(Instruction::LD(1, 4, Regs::RegH, Regs::RegC)),
			0x62 => Some(Instruction::LD(1, 4, Regs::RegH, Regs::RegD)),
			0x63 => Some(Instruction::LD(1, 4, Regs::RegH, Regs::RegE)),
			0x64 => Some(Instruction::LD(1, 4, Regs::RegH, Regs::RegH)),
			0x65 => Some(Instruction::LD(1, 4, Regs::RegH, Regs::RegL)),
			0x66 => Some(Instruction::LD(1, 8, Regs::RegH, Regs::HLPointee)),
			0x67 => Some(Instruction::LD(1, 4, Regs::RegH, Regs::RegA)),
			0x68 => Some(Instruction::LD(1, 4, Regs::RegL, Regs::RegB)),
			0x69 => Some(Instruction::LD(1, 4, Regs::RegL, Regs::RegC)),
			0x6A => Some(Instruction::LD(1, 4, Regs::RegL, Regs::RegD)),
			0x6B => Some(Instruction::LD(1, 4, Regs::RegL, Regs::RegE)),
			0x6C => Some(Instruction::LD(1, 4, Regs::RegL, Regs::RegH)),
			0x6D => Some(Instruction::LD(1, 4, Regs::RegL, Regs::RegL)),
			0x6E => Some(Instruction::LD(1, 8, Regs::RegL, Regs::HLPointee)),
			0x6F => Some(Instruction::LD(1, 4, Regs::RegL, Regs::RegA)),
			0x70 => Some(Instruction::LD(1, 8, Regs::HLPointee, Regs::RegB)),
			0x71 => Some(Instruction::LD(1, 8, Regs::HLPointee, Regs::RegC)),
			0x72 => Some(Instruction::LD(1, 8, Regs::HLPointee, Regs::RegD)),
			0x73 => Some(Instruction::LD(1, 8, Regs::HLPointee, Regs::RegE)),
			0x74 => Some(Instruction::LD(1, 8, Regs::HLPointee, Regs::RegH)),
			0x75 => Some(Instruction::LD(1, 8, Regs::HLPointee, Regs::RegL)),
			0x77 => Some(Instruction::LD(1, 8, Regs::HLPointee, Regs::RegA)),
			0x78 => Some(Instruction::LD(1, 4, Regs::RegA, Regs::RegB)),
			0x79 => Some(Instruction::LD(1, 4, Regs::RegA, Regs::RegC)),
			0x7A => Some(Instruction::LD(1, 4, Regs::RegA, Regs::RegD)),
			0x7B => Some(Instruction::LD(1, 4, Regs::RegA, Regs::RegE)),
			0x7C => Some(Instruction::LD(1, 4, Regs::RegA, Regs::RegH)),
			0x7D => Some(Instruction::LD(1, 4, Regs::RegA, Regs::RegL)),
			0x7E => Some(Instruction::LD(1, 8, Regs::RegA, Regs::HLPointee)),
			0x7F => Some(Instruction::LD(1, 4, Regs::RegA, Regs::RegA)),
			0x80 => Some(Instruction::ADDAs(1, 4, ArithmeticOperand::RegB)),
			0x81 => Some(Instruction::ADDAs(1, 4, ArithmeticOperand::RegC)),
			0x82 => Some(Instruction::ADDAs(1, 4, ArithmeticOperand::RegD)),
			0x83 => Some(Instruction::ADDAs(1, 4, ArithmeticOperand::RegE)),
			0x84 => Some(Instruction::ADDAs(1, 4, ArithmeticOperand::RegH)),
			0x85 => Some(Instruction::ADDAs(1, 4, ArithmeticOperand::RegL)),
			0x86 => Some(Instruction::ADDAs(1, 8, ArithmeticOperand::HLPointee)),
			0x87 => Some(Instruction::ADDAs(1, 4, ArithmeticOperand::RegA)),
			0x88 => Some(Instruction::ADCAs(1, 4, ArithmeticOperand::RegB)),
			0x89 => Some(Instruction::ADCAs(1, 4, ArithmeticOperand::RegC)),
			0x8A => Some(Instruction::ADCAs(1, 4, ArithmeticOperand::RegD)),
			0x8B => Some(Instruction::ADCAs(1, 4, ArithmeticOperand::RegE)),
			0x8C => Some(Instruction::ADCAs(1, 4, ArithmeticOperand::RegH)),
			0x8D => Some(Instruction::ADCAs(1, 4, ArithmeticOperand::RegL)),
			0x8E => Some(Instruction::ADCAs(1, 8, ArithmeticOperand::HLPointee)),
			0x8F => Some(Instruction::ADCAs(1, 4, ArithmeticOperand::RegA)),
			0x90 => Some(Instruction::SUBs(1, 4, ArithmeticOperand::RegB)),
			0x91 => Some(Instruction::SUBs(1, 4, ArithmeticOperand::RegC)),
			0x92 => Some(Instruction::SUBs(1, 4, ArithmeticOperand::RegD)),
			0x93 => Some(Instruction::SUBs(1, 4, ArithmeticOperand::RegE)),
			0x94 => Some(Instruction::SUBs(1, 4, ArithmeticOperand::RegH)),
			0x95 => Some(Instruction::SUBs(1, 4, ArithmeticOperand::RegL)),
			0x96 => Some(Instruction::SUBs(1, 8, ArithmeticOperand::HLPointee)),
			0x97 => Some(Instruction::SUBs(1, 4, ArithmeticOperand::RegA)),
			0x98 => Some(Instruction::SBCAs(1, 4, ArithmeticOperand::RegB)),
			0x99 => Some(Instruction::SBCAs(1, 4, ArithmeticOperand::RegC)),
			0x9A => Some(Instruction::SBCAs(1, 4, ArithmeticOperand::RegD)),
			0x9B => Some(Instruction::SBCAs(1, 4, ArithmeticOperand::RegE)),
			0x9C => Some(Instruction::SBCAs(1, 4, ArithmeticOperand::RegH)),
			0x9D => Some(Instruction::SBCAs(1, 4, ArithmeticOperand::RegL)),
			0x9E => Some(Instruction::SBCAs(1, 8, ArithmeticOperand::HLPointee)),
			0x9F => Some(Instruction::SBCAs(1, 4, ArithmeticOperand::RegA)),
			0xA0 => Some(Instruction::ANDs(1, 4, ArithmeticOperand::RegB)),
			0xA1 => Some(Instruction::ANDs(1, 4, ArithmeticOperand::RegC)),
			0xA2 => Some(Instruction::ANDs(1, 4, ArithmeticOperand::RegD)),
			0xA3 => Some(Instruction::ANDs(1, 4, ArithmeticOperand::RegE)),
			0xA4 => Some(Instruction::ANDs(1, 4, ArithmeticOperand::RegH)),
			0xA5 => Some(Instruction::ANDs(1, 4, ArithmeticOperand::RegL)),
			0xA6 => Some(Instruction::ANDs(1, 8, ArithmeticOperand::HLPointee)),
			0xA7 => Some(Instruction::ANDs(1, 4, ArithmeticOperand::RegA)),
			0xA8 => Some(Instruction::XORs(1, 4, ArithmeticOperand::RegB)),
			0xA9 => Some(Instruction::XORs(1, 4, ArithmeticOperand::RegC)),
			0xAA => Some(Instruction::XORs(1, 4, ArithmeticOperand::RegD)),
			0xAB => Some(Instruction::XORs(1, 4, ArithmeticOperand::RegE)),
			0xAC => Some(Instruction::XORs(1, 4, ArithmeticOperand::RegH)),
			0xAD => Some(Instruction::XORs(1, 4, ArithmeticOperand::RegL)),
			0xAE => Some(Instruction::XORs(1, 8, ArithmeticOperand::HLPointee)),
			0xAF => Some(Instruction::XORs(1, 4, ArithmeticOperand::RegA)),
			0xB0 => Some(Instruction::ORs(1, 4, ArithmeticOperand::RegB)),
			0xB1 => Some(Instruction::ORs(1, 4, ArithmeticOperand::RegC)),
			0xB2 => Some(Instruction::ORs(1, 4, ArithmeticOperand::RegD)),
			0xB3 => Some(Instruction::ORs(1, 4, ArithmeticOperand::RegE)),
			0xB4 => Some(Instruction::ORs(1, 4, ArithmeticOperand::RegH)),
			0xB5 => Some(Instruction::ORs(1, 4, ArithmeticOperand::RegL)),
			0xB6 => Some(Instruction::ORs(1, 8, ArithmeticOperand::HLPointee)),
			0xB7 => Some(Instruction::ORs(1, 4, ArithmeticOperand::RegA)),
			0xB8 => Some(Instruction::CPs(1, 4, ArithmeticOperand::RegB)),
			0xB9 => Some(Instruction::CPs(1, 4, ArithmeticOperand::RegC)),
			0xBA => Some(Instruction::CPs(1, 4, ArithmeticOperand::RegD)),
			0xBB => Some(Instruction::CPs(1, 4, ArithmeticOperand::RegE)),
			0xBC => Some(Instruction::CPs(1, 4, ArithmeticOperand::RegH)),
			0xBD => Some(Instruction::CPs(1, 4, ArithmeticOperand::RegL)),
			0xBE => Some(Instruction::CPs(1, 8, ArithmeticOperand::HLPointee)),
			0xBF => Some(Instruction::CPs(1, 4, ArithmeticOperand::RegA)),

			0xC6 => Some(Instruction::ADDAs(2, 8, ArithmeticOperand::ByteFromPC)),
			0xCE => Some(Instruction::ADCAs(2, 8, ArithmeticOperand::ByteFromPC)),
			0xD6 => Some(Instruction::SUBs(2, 8, ArithmeticOperand::ByteFromPC)),
			0xDE => Some(Instruction::SBCAs(2, 8, ArithmeticOperand::ByteFromPC)),
			0xE0 => Some(Instruction::LD(2, 12, Regs::UpperRamOffsetFromPC, Regs::RegA)),
			0xE2 => Some(Instruction::LD(1, 8, Regs::UpperRamOffsetFromRegC, Regs::RegA)),
			0xE6 => Some(Instruction::ANDs(2, 8, ArithmeticOperand::ByteFromPC)),
			0xE8 => Some(Instruction::ADDSPe(2, 16)),
			0xEA => Some(Instruction::LD(3, 16, Regs::BytesFromPCPointee, Regs::RegA)),
			0xEE => Some(Instruction::XORs(2, 8, ArithmeticOperand::ByteFromPC)),
			0xF0 => Some(Instruction::LD(2, 12, Regs::RegA, Regs::UpperRamOffsetFromPC)),
			0xF2 => Some(Instruction::LD(1, 8, Regs::RegA, Regs::UpperRamOffsetFromRegC)),
			0xF6 => Some(Instruction::ORs(2, 8, ArithmeticOperand::ByteFromPC)),
			0xFA => Some(Instruction::LD(3, 16, Regs::RegA, Regs::BytesFromPCPointee)),
			0xFE => Some(Instruction::CPs(2, 8, ArithmeticOperand::ByteFromPC)),
			_ => None
		}
	}
}

impl Cpu {
	pub fn get_operand_value(&mut self, operand: ArithmeticOperand) -> u8 {
		match operand {
			ArithmeticOperand::RegA => self.registers.a,
			ArithmeticOperand::RegB => self.registers.b,
			ArithmeticOperand::RegC => self.registers.c,
			ArithmeticOperand::RegD => self.registers.d,
			ArithmeticOperand::RegE => self.registers.e,
			ArithmeticOperand::RegH => self.registers.h,
			ArithmeticOperand::RegL => self.registers.l,
			ArithmeticOperand::HLPointee => self.registers.get_hl_pointee(&self.memory_bus),
			ArithmeticOperand::ByteFromPC => self.fetch_pc()
		}
	}
	pub fn get_large_operand_value(&mut self, operand: LargeArithmeticOperand) -> u16 {
		match operand {
			LargeArithmeticOperand::RegsBC => self.registers.get_bc(),
			LargeArithmeticOperand::RegsDE => self.registers.get_de(),
			LargeArithmeticOperand::RegsHL => self.registers.get_hl(),
			LargeArithmeticOperand::RegSP => self.registers.stack_pointer,
		}
	}
	pub fn get_reg_value(&mut self, reg: Regs) -> u8 {
		match reg {
			Regs::RegA => self.registers.a,
			Regs::RegB => self.registers.b,
			Regs::RegC => self.registers.c,
			Regs::RegD => self.registers.d,
			Regs::RegE => self.registers.e,
			Regs::RegH => self.registers.h,
			Regs::RegL => self.registers.l,
			Regs::HLPointee => self.registers.get_hl_pointee(&self.memory_bus),
			Regs::BCPointee => self.registers.get_bc_pointee(&self.memory_bus),
			Regs::DEPointee => self.registers.get_de_pointee(&self.memory_bus),
			Regs::BytesFromPCPointee => {
				let address = self.fetch_pc() as u16 | ((self.fetch_pc() as u16) << 8);
				self.memory_bus.read_byte(address)
			}
			Regs::UpperRamOffsetFromPC => {
				let address = 0xFF00 + self.fetch_pc() as u16;
				self.memory_bus.read_byte(address)
			}
			Regs::UpperRamOffsetFromRegC => self.memory_bus.read_byte(0xFF00 + self.registers.c as u16),
			Regs::ByteFromPC => self.fetch_pc()
		}
	}
	pub fn set_reg_value(&mut self, reg: Regs, data: u8) {
		match reg {
			Regs::RegA => {self.registers.a = data}
			Regs::RegB => {self.registers.b = data}
			Regs::RegC => {self.registers.c = data}
			Regs::RegD => {self.registers.d = data}
			Regs::RegE => {self.registers.e = data}
			Regs::RegH => {self.registers.h = data}
			Regs::RegL => {self.registers.l = data}
			Regs::HLPointee => {self.registers.set_hl_pointee(&mut self.memory_bus, data)}
			Regs::BCPointee => {self.registers.set_bc_pointee(&mut self.memory_bus, data)},
			Regs::DEPointee => {self.registers.set_de_pointee(&mut self.memory_bus, data)},
			Regs::BytesFromPCPointee => {
				let address = self.fetch_pc() as u16 | ((self.fetch_pc() as u16) << 8);
				self.memory_bus.write_byte(address, data)
			}
			Regs::UpperRamOffsetFromPC => {
				let address = 0xFF00 + self.fetch_pc() as u16;
				self.memory_bus.write_byte(address, data)
			}
			Regs::UpperRamOffsetFromRegC => {self.memory_bus.write_byte(0xFF00 + self.registers.c as u16, data)},
			Regs::ByteFromPC => {println!("SHOULD NEVER HAPPEN")}
		}
	}
	pub fn set_reg_pair_value(&mut self, reg_pair: RegPairs, data: u16) {
		match reg_pair {
			RegPairs::RegsBC => {self.registers.set_bc(data)}
			RegPairs::RegsDE => {self.registers.set_de(data)}
			RegPairs::RegsHL => {self.registers.set_hl(data)}
			RegPairs::RegSP => {self.registers.stack_pointer = data}
		}
	}
	pub fn get_reg_pair_value(&self, reg_pair: RegPairs) -> u16 {
		match reg_pair {
			RegPairs::RegsBC => self.registers.get_bc(),
			RegPairs::RegsDE => self.registers.get_de(),
			RegPairs::RegsHL => self.registers.get_hl(),
			RegPairs::RegSP => self.registers.stack_pointer
		}
	}
	pub fn execute_op(&mut self, instruction: Instruction) {
		match instruction {
			Instruction::NOP(_, _) => {}
			Instruction::LD(_, _, target, src) => {
				let data = self.get_reg_value(src);
				self.set_reg_value(target, data);
			}
			Instruction::LDI(_, _, target, src) => {
				let data = self.get_reg_value(src);
				self.set_reg_value(target, data);
				self.registers.set_hl(self.registers.get_hl().overflowing_add(1).0);
			}
			Instruction::LDD(_, _, target, src) => {
				let data = self.get_reg_value(src);
				self.set_reg_value(target, data);
				self.registers.set_hl(self.registers.get_hl().overflowing_sub(1).0);
			}
			Instruction::ADDAs(_, _, _operand) | Instruction::ADCAs(_, _, _operand) => {
				let reg_a_content = self.registers.a as u16;
				let operand_val = self.get_operand_value(_operand) as u16;
				// if instr is ADC and carry flag is true, let carry = 1;
				let carry_val = if let Instruction::ADCAs(_, _, _) = instruction {self.registers.f.carry as u16} else {0};
				let r = reg_a_content + operand_val + carry_val;
				self.registers.f.zero = r as u8 == 0;
				self.registers.f.substract = false;
				self.registers.f.half_carry = (reg_a_content & 0xF) + (operand_val & 0xF) + carry_val >= 0x10;
				self.registers.f.carry = r & 0x100 != 0;					// reg_a_content + operand + carry >= 0x100
				self.registers.a = r as u8;
			}
			Instruction::SUBs(_, _, _operand) | Instruction::SBCAs(_, _, _operand) => {
				let reg_a_content = self.registers.a as u16;
				let operand_val = self.get_operand_value(_operand) as u16;
				// if instr is ADC and carry flag is true, let carry = 1;
				let carry_val = if let Instruction::SBCAs(_, _, _) = instruction {self.registers.f.carry as u16} else {0};
				let r = reg_a_content.overflowing_sub(operand_val + carry_val).0;
				self.registers.f.zero = r as u8 == 0;
				self.registers.f.substract = true;
				self.registers.f.half_carry = (reg_a_content & 0xF) - carry_val < (operand_val & 0xF);	// carry val is subs befor comp, from what I got of the nintendo manual
				self.registers.f.carry = r & 0x100 != 0; 					// reg_a_content < operand + carry; In unsigned logic, a borrow from the next unset bit sets it
				self.registers.a = r as u8;
			}
			Instruction::ANDs(_, _, _operand) => {
				let operand_val = self.get_operand_value(_operand);
				self.registers.a &= operand_val;
				self.registers.f.zero = self.registers.a == 0;
				self.registers.f.substract = false;
				self.registers.f.half_carry = true;
				self.registers.f.carry = false;
			}
			Instruction::XORs(_, _, _operand) => {
				let operand_val = self.get_operand_value(_operand);
				self.registers.a ^= operand_val;
				self.registers.f.zero = self.registers.a == 0;
				self.registers.f.substract = false;
				self.registers.f.half_carry = false;
				self.registers.f.carry = false;
			}
			Instruction::ORs(_, _, _operand) => {
				let operand_val = self.get_operand_value(_operand);
				self.registers.a |= operand_val;
				self.registers.f.zero = self.registers.a == 0;
				self.registers.f.substract = false;
				self.registers.f.half_carry = false;
				self.registers.f.carry = false;
			}
			Instruction::CPs(_, _, _operand) => {
				let reg_a_content = self.registers.a as u16;
				let operand_val = self.get_operand_value(_operand) as u16;
				let r = reg_a_content.overflowing_sub(operand_val).0;
				self.registers.f.zero = r as u8 == 0;
				self.registers.f.substract = true;
				self.registers.f.half_carry = (reg_a_content & 0xF) < (operand_val & 0xF);
				self.registers.f.carry = r & 0x100 != 0;
			}
			Instruction::INCs(_, _, _target) => {
				let target_value = self.get_reg_value(_target);
				self.registers.f.zero = target_value == 0xFF;
				self.registers.f.substract = false;
				self.registers.f.half_carry = (target_value & 0xF) + 1 >= 0x10;
				self.set_reg_value(_target, target_value.overflowing_add(1).0);
			}
			Instruction::DECs(_, _, _target) => {
				let target_value = self.get_reg_value(_target);
				self.registers.f.zero = target_value == 0x01;
				self.registers.f.substract = true;
				self.registers.f.half_carry = target_value & 0xF < 1;
				self.set_reg_value(_target, target_value.overflowing_sub(1).0);
			}
			Instruction::INCss(_, _, _target) => {
				self.set_reg_pair_value(_target, self.get_reg_pair_value(_target).overflowing_add(1).0);
			}
			Instruction::DECss(_, _, _target) => {
				self.set_reg_pair_value(_target, self.get_reg_pair_value(_target).overflowing_sub(1).0);
			}
			Instruction::ADDHLss(_, _, _operand) => {
				let hl_value = self.registers.get_hl();
				let operand_value = self.get_large_operand_value(_operand);
				let r = hl_value.overflowing_add(operand_value).0;
				self.registers.f.substract = false;
				self.registers.f.half_carry = (hl_value & 0x0FFF) + (operand_value & 0x0FFF) >= 0x1000;
				self.registers.f.carry = r < hl_value;
				self.registers.set_hl(r);
			}
			Instruction::ADDSPe(_, _) => {
				let sp_value = self.registers.stack_pointer;
				let operand_value = self.fetch_pc() as i8 as u16;
				let res = sp_value.overflowing_add(operand_value).0;
				self.registers.f.zero = false;
				self.registers.f.substract = false;
				self.registers.f.half_carry = (sp_value & 0xF) + (operand_value & 0xF) >= 0x10;
				self.registers.f.carry = (sp_value & 0xFF) + (operand_value & 0xFF) >= 0x100;
				self.registers.stack_pointer = res;
			}
			Instruction::CCF(_, _) => {
				self.registers.f.substract = false;
				self.registers.f.half_carry = false;
				self.registers.f.carry = !self.registers.f.carry;
			}
			Instruction::SCF(_, _) => {
				self.registers.f.substract = false;
				self.registers.f.half_carry = false;
				self.registers.f.carry = true;
			}
			Instruction::DAA(_, _) => {
				let reg_a_content = self.registers.a as u16;
				let mut daa_term = 0 as i8;
				if self.registers.f.substract {
					if self.registers.f.half_carry {
						daa_term -= 0x06
					}
					if self.registers.f.carry {
						daa_term -= 0x60;
					}
				} else {
					if self.registers.f.carry || reg_a_content > 0x99 {
						self.registers.f.carry = true;
						daa_term += 0x60;
					}
					if self.registers.f.half_carry || (reg_a_content & 0x0F) > 0x09 {
						daa_term += 0x06;
					}
				}
				let daa_term = daa_term as u8 as u16;
				let reg_a_content = (reg_a_content + daa_term) as u8;
				self.registers.f.zero = reg_a_content == 0;
				self.registers.f.half_carry = false;
				self.registers.a = reg_a_content;
			}
			Instruction::CPL(_, _) => {
				self.registers.a = !self.registers.a;
				self.registers.f.substract = true;
				self.registers.f.half_carry = true;
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::{*, cpu::{registers::FlagsRegister, instructions::LargeArithmeticOperand}};

use super::{Instruction, ArithmeticOperand};
	fn test_adds(cpu: &mut Cpu, init_a_value: u8, expected_res: u8, expected_flag_reg: FlagsRegister) {
		cpu.current_op = Some(Instruction::ADDAs(1, 4, ArithmeticOperand::RegA));
		cpu.registers.a = init_a_value;
		cpu.exec_current_op();
		assert_eq!(cpu.registers.a, expected_res);
		assert_eq!(cpu.registers.f, expected_flag_reg);
	}
	fn test_sub(cpu: &mut Cpu, init_a_value: u8, operand: u8, expected_res: u8, expected_flag_reg: FlagsRegister) {
		cpu.current_op = Some(Instruction::SUBs(1, 4, ArithmeticOperand::RegB));
		cpu.registers.a = init_a_value;
		cpu.registers.b = operand;
		cpu.exec_current_op();
		assert_eq!(cpu.registers.a, expected_res);
		assert_eq!(cpu.registers.f, expected_flag_reg);
	}
	fn test_cps(cpu: &mut Cpu, init_a_value: u8, operand: u8, expected_flag_reg: FlagsRegister) {
		cpu.current_op = Some(Instruction::CPs(1, 4, ArithmeticOperand::RegB));
		cpu.registers.a = init_a_value;
		cpu.registers.b = operand;
		cpu.exec_current_op();
		assert_eq!(cpu.registers.f, expected_flag_reg);
	}
	fn test_addhlss(cpu: &mut Cpu, init_hl_value: u16, expected_res: u16, expected_flag_reg: FlagsRegister) {
		cpu.current_op = Some(Instruction::ADDHLss(1, 8, LargeArithmeticOperand::RegsHL));
		cpu.registers.set_hl(init_hl_value);
		cpu.exec_current_op();
		assert_eq!(cpu.registers.get_hl(), expected_res);
		assert_eq!(cpu.registers.f, expected_flag_reg);
	}
	fn test_addspe(cpu: &mut Cpu, init_sp_value: u16, operand: i8, expected_res: u16, expected_flag_reg: FlagsRegister) {
		cpu.current_op = Some(Instruction::ADDSPe(2, 16));
		cpu.registers.stack_pointer = init_sp_value;
		cpu.memory_bus.write_byte(cpu.registers.program_counter, operand as u8);
		cpu.exec_current_op();
		assert_eq!(cpu.registers.stack_pointer, expected_res);
		assert_eq!(cpu.registers.f, expected_flag_reg);
	}
	fn test_daa(cpu: &mut Cpu, expected_res: u8, expected_flag_reg: FlagsRegister) {
		cpu.current_op = Some(Instruction::DAA(1, 4));
		cpu.exec_current_op();
		assert_eq!(cpu.registers.a, expected_res);
		assert_eq!(cpu.registers.f, expected_flag_reg);
	}
	#[test]
	fn test_arith() {
		let mut my_cpu = Cpu::new();
		test_adds(&mut my_cpu, 0x12, 0x24, 0x00.into());
		test_adds(&mut my_cpu, 0x80, 0x00, FlagsRegister{ zero: true, substract: false, half_carry: false, carry: true });
		test_adds(&mut my_cpu, 0xF1, 0xE2, FlagsRegister{ zero: false, substract: false, half_carry: false, carry: true });
		test_adds(&mut my_cpu, 0xFF, 0xFE, FlagsRegister{ zero: false, substract: false, half_carry: true, carry: true });
		test_sub(&mut my_cpu, 0xFF, 0x10, 0xEF, FlagsRegister{ zero: false, substract: true, half_carry: false, carry: false });
		test_sub(&mut my_cpu, 0xFF, 0xFF, 0x00, FlagsRegister{ zero: true, substract: true, half_carry: false, carry: false });
		test_sub(&mut my_cpu, 0xF1, 0x0F, 0xE2, FlagsRegister{ zero: false, substract: true, half_carry: true, carry: false });
		test_sub(&mut my_cpu, 0x10, 0x20, 0xF0, FlagsRegister{ zero: false, substract: true, half_carry: false, carry: true });
		test_sub(&mut my_cpu, 0x10, 0x21, 0xEF, FlagsRegister{ zero: false, substract: true, half_carry: true, carry: true });
		test_cps(&mut my_cpu, 0xFF, 0x10, FlagsRegister{ zero: false, substract: true, half_carry: false, carry: false });
		test_cps(&mut my_cpu, 0xFF, 0xFF, FlagsRegister{ zero: true, substract: true, half_carry: false, carry: false });
		test_cps(&mut my_cpu, 0xF1, 0x0F, FlagsRegister{ zero: false, substract: true, half_carry: true, carry: false });
		test_cps(&mut my_cpu, 0x10, 0x20, FlagsRegister{ zero: false, substract: true, half_carry: false, carry: true });
		test_cps(&mut my_cpu, 0x10, 0x21, FlagsRegister{ zero: false, substract: true, half_carry: true, carry: true });

		test_addhlss(&mut my_cpu, 0x8A23, 0x1446, FlagsRegister{ zero: false, substract: false, half_carry: true, carry: true });
		test_addhlss(&mut my_cpu, 0x0000, 0x0000, FlagsRegister{ zero: false, substract: false, half_carry: false, carry: false });
		test_addspe(&mut my_cpu, 0xFFF8, 0x02, 0xFFFA, FlagsRegister{ zero: false, substract: false, half_carry: false, carry: false });
		test_addspe(&mut my_cpu, 0xFF88, 0x0F, 0xFF97, FlagsRegister{ zero: false, substract: false, half_carry: true, carry: false });
		test_addspe(&mut my_cpu, 0xF8D8, 0x2F, 0xF907, FlagsRegister{ zero: false, substract: false, half_carry: true, carry: true });
		test_addspe(&mut my_cpu, 0xF8D8, -0x24, 0xF8B4, FlagsRegister{ zero: false, substract: false, half_carry: true, carry: true });

		test_adds(&mut my_cpu, 0x45, 0x8A, FlagsRegister{ zero: false, substract: false, half_carry: false, carry: false });
		test_daa(&mut my_cpu, 0x90, FlagsRegister{ zero: false, substract: false, half_carry: false, carry: false });
		test_adds(&mut my_cpu, 0x91, 0x22, FlagsRegister{ zero: false, substract: false, half_carry: false, carry: true });
		test_daa(&mut my_cpu, 0x82, FlagsRegister{ zero: false, substract: false, half_carry: false, carry: true });
		test_sub(&mut my_cpu, 0x83, 0x38, 0x4B, FlagsRegister{ zero: false, substract: true, half_carry: true, carry: false });
		test_daa(&mut my_cpu, 0x45, FlagsRegister{ zero: false, substract: true, half_carry: false, carry: false });
	}
}