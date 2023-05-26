use crate::memory_bus::MemoryBus;

use super::{Cpu, CpuState};

type InstrLength = u8;
type InstrCycles = u8;

#[derive(Debug, Clone, Copy)]
pub enum Regs{
	RegA, RegB, RegC, RegD, RegE, RegH, RegL, HLPointee, BCPointee, DEPointee, BytesFromPCPointee, UpperRamOffsetFromPC, UpperRamOffsetFromRegC, ByteFromPC
}
#[derive(Debug, Clone, Copy)]
pub enum RegPairs{
	RegsAF, RegsBC, RegsDE, RegsHL, RegSP, BytesFromPCPointee, BytesFromPC
}

#[derive(Debug, Clone, Copy)]
pub enum JumpCondition{
	NotZero, Zero, NotCarry, Carry
}

#[derive(Debug, Clone, Copy)]
pub enum ResetLocation{
	Hex00, Hex08, Hex10, Hex18, Hex20, Hex28, Hex30, Hex38
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
	LD(InstrLength, InstrCycles, Regs, Regs),
	LDI(InstrLength, InstrCycles, Regs, Regs),
	LDD(InstrLength, InstrCycles, Regs, Regs),
	LD16(InstrLength, InstrCycles, RegPairs, RegPairs),
	PUSH(InstrLength, InstrCycles, RegPairs),
	POP(InstrLength, InstrCycles, RegPairs),
	ADDAs(InstrLength, InstrCycles, Regs),
	ADCAs(InstrLength, InstrCycles, Regs),
	SUBs(InstrLength, InstrCycles, Regs),
	SBCAs(InstrLength, InstrCycles, Regs),
	ANDs(InstrLength, InstrCycles, Regs),
	XORs(InstrLength, InstrCycles, Regs),
	ORs(InstrLength, InstrCycles, Regs),
	CPs(InstrLength, InstrCycles, Regs),
	INCs(InstrLength, InstrCycles, Regs),
	DECs(InstrLength, InstrCycles, Regs),
	DAA(InstrLength, InstrCycles),
	CPL(InstrLength, InstrCycles),
	ADDHLss(InstrLength, InstrCycles, RegPairs),
	INCss(InstrLength, InstrCycles, RegPairs),
	DECss(InstrLength, InstrCycles, RegPairs),
	ADDSPe(InstrLength, InstrCycles),
	LDHLSPe(InstrLength, InstrCycles),
	RLCA(InstrLength, InstrCycles),
	RLA(InstrLength, InstrCycles),
	RRCA(InstrLength, InstrCycles),
	RRA(InstrLength, InstrCycles),
	RLC(InstrLength, InstrCycles, Regs),
	RL(InstrLength, InstrCycles, Regs),
	RRC(InstrLength, InstrCycles, Regs),
	RR(InstrLength, InstrCycles, Regs),
	SLA(InstrLength, InstrCycles, Regs),
	SWAP(InstrLength, InstrCycles, Regs),
	SRA(InstrLength, InstrCycles, Regs),
	SRL(InstrLength, InstrCycles, Regs),
	BIT(InstrLength, InstrCycles, u8, Regs),
	SET(InstrLength, InstrCycles, u8, Regs),
	RES(InstrLength, InstrCycles, u8, Regs),
	CCF(InstrLength, InstrCycles),
	SCF(InstrLength, InstrCycles),
	NOP(InstrLength, InstrCycles),
	HALT(InstrLength, InstrCycles),
	STOP(InstrLength, InstrCycles),
	DI(InstrLength, InstrCycles),
	EI(InstrLength, InstrCycles),
	JPnn(InstrLength, InstrCycles),
	JPHL(InstrLength, InstrCycles),
	JPfnn(InstrLength, InstrCycles, JumpCondition),
	JR(InstrLength, InstrCycles),
	JRf(InstrLength, InstrCycles, JumpCondition),
	CALL(InstrLength, InstrCycles),
	CALLf(InstrLength, InstrCycles, JumpCondition),
	ISR(InstrLength, InstrCycles),
	RET(InstrLength, InstrCycles),
	RETf(InstrLength, InstrCycles, JumpCondition),
	RETI(InstrLength, InstrCycles),
	RST(InstrLength, InstrCycles, ResetLocation),
}

impl Instruction {
	pub fn from_opcode(opcode: u8, cpu: &mut Cpu, memory_bus: &MemoryBus) -> Option<Instruction> {
		match opcode {
			0x00 => Some(Instruction::NOP(1, 4)),
			0x01 => Some(Instruction::LD16(3, 12, RegPairs::RegsBC, RegPairs::BytesFromPC)),
			0x02 => Some(Instruction::LD(1, 8, Regs::BCPointee, Regs::RegA)),
			0x03 => Some(Instruction::INCss(1, 8, RegPairs::RegsBC)),
			0x04 => Some(Instruction::INCs(1, 4, Regs::RegB)),
			0x05 => Some(Instruction::DECs(1, 4, Regs::RegB)),
			0x06 => Some(Instruction::LD(2, 8, Regs::RegB, Regs::ByteFromPC)),
			0x07 => Some(Instruction::RLCA(1, 4)),
			0x08 => Some(Instruction::LD16(3, 20, RegPairs::BytesFromPCPointee, RegPairs::RegSP)),
			0x09 => Some(Instruction::ADDHLss(1, 8, RegPairs::RegsBC)),
			0x0A => Some(Instruction::LD(1, 8, Regs::RegA, Regs::BCPointee)),
			0x0B => Some(Instruction::DECss(1, 8, RegPairs::RegsBC)),
			0x0C => Some(Instruction::INCs(1, 4, Regs::RegC)),
			0x0D => Some(Instruction::DECs(1, 4, Regs::RegC)),
			0x0E => Some(Instruction::LD(2, 8, Regs::RegC, Regs::ByteFromPC)),
			0x0F => Some(Instruction::RRCA(1, 4)),
			0x10 => Some(Instruction::STOP(2, 4)),													// STOP takes no argument per se but will still consume the next byte
			0x11 => Some(Instruction::LD16(3, 12, RegPairs::RegsDE, RegPairs::BytesFromPC)),
			0x12 => Some(Instruction::LD(1, 8, Regs::DEPointee, Regs::RegA)),
			0x13 => Some(Instruction::INCss(1, 8, RegPairs::RegsDE)),
			0x14 => Some(Instruction::INCs(1, 4, Regs::RegD)),
			0x15 => Some(Instruction::DECs(1, 4, Regs::RegD)),
			0x16 => Some(Instruction::LD(2, 8, Regs::RegD, Regs::ByteFromPC)),
			0x17 => Some(Instruction::RLA(1, 4)),
			0x18 => Some(Instruction::JR(2, 12)),
			0x19 => Some(Instruction::ADDHLss(1, 8, RegPairs::RegsDE)),
			0x1A => Some(Instruction::LD(1, 8, Regs::RegA, Regs::DEPointee)),
			0x1B => Some(Instruction::DECss(1, 8, RegPairs::RegsDE)),
			0x1C => Some(Instruction::INCs(1, 4, Regs::RegE)),
			0x1D => Some(Instruction::DECs(1, 4, Regs::RegE)),
			0x1E => Some(Instruction::LD(2, 8, Regs::RegE, Regs::ByteFromPC)),
			0x1F => Some(Instruction::RRA(1, 4)),
			0x20 => Some(Instruction::JRf(2, 8, JumpCondition::NotZero)),
			0x21 => Some(Instruction::LD16(3, 12, RegPairs::RegsHL, RegPairs::BytesFromPC)),
			0x22 => Some(Instruction::LDI(1, 8, Regs::HLPointee, Regs::RegA)),
			0x23 => Some(Instruction::INCss(1, 8, RegPairs::RegsHL)),
			0x24 => Some(Instruction::INCs(1, 4, Regs::RegH)),
			0x25 => Some(Instruction::DECs(1, 4, Regs::RegH)),
			0x26 => Some(Instruction::LD(2, 8, Regs::RegH, Regs::ByteFromPC)),
			0x27 => Some(Instruction::DAA(1, 4)),
			0x28 => Some(Instruction::JRf(2, 8, JumpCondition::Zero)),
			0x29 => Some(Instruction::ADDHLss(1, 8, RegPairs::RegsHL)),
			0x2A => Some(Instruction::LDI(1, 8, Regs::RegA, Regs::HLPointee)),
			0x2B => Some(Instruction::DECss(1, 8, RegPairs::RegsHL)),
			0x2C => Some(Instruction::INCs(1, 4, Regs::RegL)),
			0x2D => Some(Instruction::DECs(1, 4, Regs::RegL)),
			0x2E => Some(Instruction::LD(2, 8, Regs::RegL, Regs::ByteFromPC)),
			0x2F => Some(Instruction::CPL(1, 4)),
			0x30 => Some(Instruction::JRf(2, 8, JumpCondition::NotCarry)),
			0x31 => Some(Instruction::LD16(3, 12, RegPairs::RegSP, RegPairs::BytesFromPC)),
			0x32 => Some(Instruction::LDD(1, 8, Regs::HLPointee, Regs::RegA)),
			0x33 => Some(Instruction::INCss(1, 8, RegPairs::RegSP)),
			0x34 => Some(Instruction::INCs(1, 12, Regs::HLPointee)),
			0x35 => Some(Instruction::DECs(1, 12, Regs::HLPointee)),
			0x36 => Some(Instruction::LD(2, 12, Regs::HLPointee, Regs::ByteFromPC)),
			0x37 => Some(Instruction::SCF(1, 4)),
			0x38 => Some(Instruction::JRf(2, 8, JumpCondition::Carry)),
			0x39 => Some(Instruction::ADDHLss(1, 8, RegPairs::RegSP)),
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
			0x76 => Some(Instruction::HALT(1, 4)),
			0x77 => Some(Instruction::LD(1, 8, Regs::HLPointee, Regs::RegA)),
			0x78 => Some(Instruction::LD(1, 4, Regs::RegA, Regs::RegB)),
			0x79 => Some(Instruction::LD(1, 4, Regs::RegA, Regs::RegC)),
			0x7A => Some(Instruction::LD(1, 4, Regs::RegA, Regs::RegD)),
			0x7B => Some(Instruction::LD(1, 4, Regs::RegA, Regs::RegE)),
			0x7C => Some(Instruction::LD(1, 4, Regs::RegA, Regs::RegH)),
			0x7D => Some(Instruction::LD(1, 4, Regs::RegA, Regs::RegL)),
			0x7E => Some(Instruction::LD(1, 8, Regs::RegA, Regs::HLPointee)),
			0x7F => Some(Instruction::LD(1, 4, Regs::RegA, Regs::RegA)),
			0x80 => Some(Instruction::ADDAs(1, 4, Regs::RegB)),
			0x81 => Some(Instruction::ADDAs(1, 4, Regs::RegC)),
			0x82 => Some(Instruction::ADDAs(1, 4, Regs::RegD)),
			0x83 => Some(Instruction::ADDAs(1, 4, Regs::RegE)),
			0x84 => Some(Instruction::ADDAs(1, 4, Regs::RegH)),
			0x85 => Some(Instruction::ADDAs(1, 4, Regs::RegL)),
			0x86 => Some(Instruction::ADDAs(1, 8, Regs::HLPointee)),
			0x87 => Some(Instruction::ADDAs(1, 4, Regs::RegA)),
			0x88 => Some(Instruction::ADCAs(1, 4, Regs::RegB)),
			0x89 => Some(Instruction::ADCAs(1, 4, Regs::RegC)),
			0x8A => Some(Instruction::ADCAs(1, 4, Regs::RegD)),
			0x8B => Some(Instruction::ADCAs(1, 4, Regs::RegE)),
			0x8C => Some(Instruction::ADCAs(1, 4, Regs::RegH)),
			0x8D => Some(Instruction::ADCAs(1, 4, Regs::RegL)),
			0x8E => Some(Instruction::ADCAs(1, 8, Regs::HLPointee)),
			0x8F => Some(Instruction::ADCAs(1, 4, Regs::RegA)),
			0x90 => Some(Instruction::SUBs(1, 4, Regs::RegB)),
			0x91 => Some(Instruction::SUBs(1, 4, Regs::RegC)),
			0x92 => Some(Instruction::SUBs(1, 4, Regs::RegD)),
			0x93 => Some(Instruction::SUBs(1, 4, Regs::RegE)),
			0x94 => Some(Instruction::SUBs(1, 4, Regs::RegH)),
			0x95 => Some(Instruction::SUBs(1, 4, Regs::RegL)),
			0x96 => Some(Instruction::SUBs(1, 8, Regs::HLPointee)),
			0x97 => Some(Instruction::SUBs(1, 4, Regs::RegA)),
			0x98 => Some(Instruction::SBCAs(1, 4, Regs::RegB)),
			0x99 => Some(Instruction::SBCAs(1, 4, Regs::RegC)),
			0x9A => Some(Instruction::SBCAs(1, 4, Regs::RegD)),
			0x9B => Some(Instruction::SBCAs(1, 4, Regs::RegE)),
			0x9C => Some(Instruction::SBCAs(1, 4, Regs::RegH)),
			0x9D => Some(Instruction::SBCAs(1, 4, Regs::RegL)),
			0x9E => Some(Instruction::SBCAs(1, 8, Regs::HLPointee)),
			0x9F => Some(Instruction::SBCAs(1, 4, Regs::RegA)),
			0xA0 => Some(Instruction::ANDs(1, 4, Regs::RegB)),
			0xA1 => Some(Instruction::ANDs(1, 4, Regs::RegC)),
			0xA2 => Some(Instruction::ANDs(1, 4, Regs::RegD)),
			0xA3 => Some(Instruction::ANDs(1, 4, Regs::RegE)),
			0xA4 => Some(Instruction::ANDs(1, 4, Regs::RegH)),
			0xA5 => Some(Instruction::ANDs(1, 4, Regs::RegL)),
			0xA6 => Some(Instruction::ANDs(1, 8, Regs::HLPointee)),
			0xA7 => Some(Instruction::ANDs(1, 4, Regs::RegA)),
			0xA8 => Some(Instruction::XORs(1, 4, Regs::RegB)),
			0xA9 => Some(Instruction::XORs(1, 4, Regs::RegC)),
			0xAA => Some(Instruction::XORs(1, 4, Regs::RegD)),
			0xAB => Some(Instruction::XORs(1, 4, Regs::RegE)),
			0xAC => Some(Instruction::XORs(1, 4, Regs::RegH)),
			0xAD => Some(Instruction::XORs(1, 4, Regs::RegL)),
			0xAE => Some(Instruction::XORs(1, 8, Regs::HLPointee)),
			0xAF => Some(Instruction::XORs(1, 4, Regs::RegA)),
			0xB0 => Some(Instruction::ORs(1, 4, Regs::RegB)),
			0xB1 => Some(Instruction::ORs(1, 4, Regs::RegC)),
			0xB2 => Some(Instruction::ORs(1, 4, Regs::RegD)),
			0xB3 => Some(Instruction::ORs(1, 4, Regs::RegE)),
			0xB4 => Some(Instruction::ORs(1, 4, Regs::RegH)),
			0xB5 => Some(Instruction::ORs(1, 4, Regs::RegL)),
			0xB6 => Some(Instruction::ORs(1, 8, Regs::HLPointee)),
			0xB7 => Some(Instruction::ORs(1, 4, Regs::RegA)),
			0xB8 => Some(Instruction::CPs(1, 4, Regs::RegB)),
			0xB9 => Some(Instruction::CPs(1, 4, Regs::RegC)),
			0xBA => Some(Instruction::CPs(1, 4, Regs::RegD)),
			0xBB => Some(Instruction::CPs(1, 4, Regs::RegE)),
			0xBC => Some(Instruction::CPs(1, 4, Regs::RegH)),
			0xBD => Some(Instruction::CPs(1, 4, Regs::RegL)),
			0xBE => Some(Instruction::CPs(1, 8, Regs::HLPointee)),
			0xBF => Some(Instruction::CPs(1, 4, Regs::RegA)),
			0xC0 => Some(Instruction::RETf(1, 8, JumpCondition::NotZero)),
			0xC1 => Some(Instruction::POP(1, 12, RegPairs::RegsBC)),
			0xC2 => Some(Instruction::JPfnn(3, 12, JumpCondition::NotZero)),
			0xC3 => Some(Instruction::JPnn(3, 16)),
			0xC4 => Some(Instruction::CALLf(3, 12, JumpCondition::NotZero)),
			0xC5 => Some(Instruction::PUSH(1, 16, RegPairs::RegsBC)),
			0xC6 => Some(Instruction::ADDAs(2, 8, Regs::ByteFromPC)),
			0xC7 => Some(Instruction::RST(1, 16, ResetLocation::Hex00)),
			0xC8 => Some(Instruction::RETf(1, 8, JumpCondition::Zero)),
			0xC9 => Some(Instruction::RET(1, 16)),
			0xCA => Some(Instruction::JPfnn(3, 12, JumpCondition::Zero)),
			0xCB => Self::from_cb_opcode(cpu.fetch_pc(memory_bus)),
			0xCC => Some(Instruction::CALLf(3, 12, JumpCondition::Zero)),
			0xCD => Some(Instruction::CALL(3, 24)),
			0xCE => Some(Instruction::ADCAs(2, 8, Regs::ByteFromPC)),
			0xCF => Some(Instruction::RST(1, 16, ResetLocation::Hex08)),
			0xD0 => Some(Instruction::RETf(1, 8, JumpCondition::NotCarry)),
			0xD1 => Some(Instruction::POP(1, 12, RegPairs::RegsDE)),
			0xD2 => Some(Instruction::JPfnn(3, 12, JumpCondition::NotCarry)),
			0xD4 => Some(Instruction::CALLf(3, 12, JumpCondition::NotCarry)),
			0xD5 => Some(Instruction::PUSH(1, 16, RegPairs::RegsDE)),
			0xD6 => Some(Instruction::SUBs(2, 8, Regs::ByteFromPC)),
			0xD7 => Some(Instruction::RST(1, 16, ResetLocation::Hex10)),
			0xD8 => Some(Instruction::RETf(1, 8, JumpCondition::Carry)),
			0xD9 => Some(Instruction::RETI(1, 16)),
			0xDA => Some(Instruction::JPfnn(3, 12, JumpCondition::Carry)),
			0xDC => Some(Instruction::CALLf(3, 12, JumpCondition::Carry)),
			0xDE => Some(Instruction::SBCAs(2, 8, Regs::ByteFromPC)),
			0xDF => Some(Instruction::RST(1, 16, ResetLocation::Hex18)),
			0xE0 => Some(Instruction::LD(2, 12, Regs::UpperRamOffsetFromPC, Regs::RegA)),
			0xE1 => Some(Instruction::POP(1, 12, RegPairs::RegsHL)),
			0xE2 => Some(Instruction::LD(1, 8, Regs::UpperRamOffsetFromRegC, Regs::RegA)),
			0xE5 => Some(Instruction::PUSH(1, 16, RegPairs::RegsHL)),
			0xE6 => Some(Instruction::ANDs(2, 8, Regs::ByteFromPC)),
			0xE7 => Some(Instruction::RST(1, 16, ResetLocation::Hex20)),
			0xE8 => Some(Instruction::ADDSPe(2, 16)),
			0xE9 => Some(Instruction::JPHL(1, 4)),
			0xEA => Some(Instruction::LD(3, 16, Regs::BytesFromPCPointee, Regs::RegA)),
			0xEE => Some(Instruction::XORs(2, 8, Regs::ByteFromPC)),
			0xEF => Some(Instruction::RST(1, 16, ResetLocation::Hex28)),
			0xF0 => Some(Instruction::LD(2, 12, Regs::RegA, Regs::UpperRamOffsetFromPC)),
			0xF1 => Some(Instruction::POP(1, 12, RegPairs::RegsAF)),
			0xF2 => Some(Instruction::LD(1, 8, Regs::RegA, Regs::UpperRamOffsetFromRegC)),
			0xF3 => Some(Instruction::DI(1, 4)),
			0xF5 => Some(Instruction::PUSH(1, 16, RegPairs::RegsAF)),
			0xF6 => Some(Instruction::ORs(2, 8, Regs::ByteFromPC)),
			0xF7 => Some(Instruction::RST(1, 16, ResetLocation::Hex30)),
			0xF8 => Some(Instruction::LDHLSPe(2, 12)),
			0xF9 => Some(Instruction::LD16(1, 8, RegPairs::RegSP, RegPairs::RegsHL)),
			0xFA => Some(Instruction::LD(3, 16, Regs::RegA, Regs::BytesFromPCPointee)),
			0xFB => Some(Instruction::EI(1, 4)),
			0xFE => Some(Instruction::CPs(2, 8, Regs::ByteFromPC)),
			0xFF => Some(Instruction::RST(1, 16, ResetLocation::Hex38)),
			0xD3 | 0xDB | 0xDD | 0xE3 | 0xE4 | 0xEB | 0xEC | 0xED | 0xF4 | 0xFC | 0xFD => None
		}
	}
	pub fn from_cb_opcode(opcode: u8) -> Option<Instruction> {
		match opcode {
			0x00 => Some(Instruction::RLC(2, 8, Regs::RegB)),
			0x01 => Some(Instruction::RLC(2, 8, Regs::RegC)),
			0x02 => Some(Instruction::RLC(2, 8, Regs::RegD)),
			0x03 => Some(Instruction::RLC(2, 8, Regs::RegE)),
			0x04 => Some(Instruction::RLC(2, 8, Regs::RegH)),
			0x05 => Some(Instruction::RLC(2, 8, Regs::RegL)),
			0x06 => Some(Instruction::RLC(2, 16, Regs::HLPointee)),
			0x07 => Some(Instruction::RLC(2, 8, Regs::RegA)),
			0x08 => Some(Instruction::RRC(2, 8, Regs::RegB)),
			0x09 => Some(Instruction::RRC(2, 8, Regs::RegC)),
			0x0A => Some(Instruction::RRC(2, 8, Regs::RegD)),
			0x0B => Some(Instruction::RRC(2, 8, Regs::RegE)),
			0x0C => Some(Instruction::RRC(2, 8, Regs::RegH)),
			0x0D => Some(Instruction::RRC(2, 8, Regs::RegL)),
			0x0E => Some(Instruction::RRC(2, 16, Regs::HLPointee)),
			0x0F => Some(Instruction::RRC(2, 8, Regs::RegA)),
			0x10 => Some(Instruction::RL(2, 8, Regs::RegB)),
			0x11 => Some(Instruction::RL(2, 8, Regs::RegC)),
			0x12 => Some(Instruction::RL(2, 8, Regs::RegD)),
			0x13 => Some(Instruction::RL(2, 8, Regs::RegE)),
			0x14 => Some(Instruction::RL(2, 8, Regs::RegH)),
			0x15 => Some(Instruction::RL(2, 8, Regs::RegL)),
			0x16 => Some(Instruction::RL(2, 16, Regs::HLPointee)),
			0x17 => Some(Instruction::RL(2, 8, Regs::RegA)),
			0x18 => Some(Instruction::RR(2, 8, Regs::RegB)),
			0x19 => Some(Instruction::RR(2, 8, Regs::RegC)),
			0x1A => Some(Instruction::RR(2, 8, Regs::RegD)),
			0x1B => Some(Instruction::RR(2, 8, Regs::RegE)),
			0x1C => Some(Instruction::RR(2, 8, Regs::RegH)),
			0x1D => Some(Instruction::RR(2, 8, Regs::RegL)),
			0x1E => Some(Instruction::RR(2, 16, Regs::HLPointee)),
			0x1F => Some(Instruction::RR(2, 8, Regs::RegA)),
			0x20 => Some(Instruction::SLA(2, 8, Regs::RegB)),
			0x21 => Some(Instruction::SLA(2, 8, Regs::RegC)),
			0x22 => Some(Instruction::SLA(2, 8, Regs::RegD)),
			0x23 => Some(Instruction::SLA(2, 8, Regs::RegE)),
			0x24 => Some(Instruction::SLA(2, 8, Regs::RegH)),
			0x25 => Some(Instruction::SLA(2, 8, Regs::RegL)),
			0x26 => Some(Instruction::SLA(2, 16, Regs::HLPointee)),
			0x27 => Some(Instruction::SLA(2, 8, Regs::RegA)),
			0x28 => Some(Instruction::SRA(2, 8, Regs::RegB)),
			0x29 => Some(Instruction::SRA(2, 8, Regs::RegC)),
			0x2A => Some(Instruction::SRA(2, 8, Regs::RegD)),
			0x2B => Some(Instruction::SRA(2, 8, Regs::RegE)),
			0x2C => Some(Instruction::SRA(2, 8, Regs::RegH)),
			0x2D => Some(Instruction::SRA(2, 8, Regs::RegL)),
			0x2E => Some(Instruction::SRA(2, 16, Regs::HLPointee)),
			0x2F => Some(Instruction::SRA(2, 8, Regs::RegA)),
			0x30 => Some(Instruction::SWAP(2, 8, Regs::RegB)),
			0x31 => Some(Instruction::SWAP(2, 8, Regs::RegC)),
			0x32 => Some(Instruction::SWAP(2, 8, Regs::RegD)),
			0x33 => Some(Instruction::SWAP(2, 8, Regs::RegE)),
			0x34 => Some(Instruction::SWAP(2, 8, Regs::RegH)),
			0x35 => Some(Instruction::SWAP(2, 8, Regs::RegL)),
			0x36 => Some(Instruction::SWAP(2, 16, Regs::HLPointee)),
			0x37 => Some(Instruction::SWAP(2, 8, Regs::RegA)),
			0x38 => Some(Instruction::SRL(2, 8, Regs::RegB)),
			0x39 => Some(Instruction::SRL(2, 8, Regs::RegC)),
			0x3A => Some(Instruction::SRL(2, 8, Regs::RegD)),
			0x3B => Some(Instruction::SRL(2, 8, Regs::RegE)),
			0x3C => Some(Instruction::SRL(2, 8, Regs::RegH)),
			0x3D => Some(Instruction::SRL(2, 8, Regs::RegL)),
			0x3E => Some(Instruction::SRL(2, 16, Regs::HLPointee)),
			0x3F => Some(Instruction::SRL(2, 8, Regs::RegA)),
			0x40 | 0x48 | 0x50 | 0x58 | 0x60 | 0x68 | 0x70 | 0x78 => Some(Instruction::BIT(2, 8, (opcode - 0x40) / 8, Regs::RegB)),
			0x41 | 0x49 | 0x51 | 0x59 | 0x61 | 0x69 | 0x71 | 0x79 => Some(Instruction::BIT(2, 8, (opcode - 0x40) / 8, Regs::RegC)),
			0x42 | 0x4A | 0x52 | 0x5A | 0x62 | 0x6A | 0x72 | 0x7A => Some(Instruction::BIT(2, 8, (opcode - 0x40) / 8, Regs::RegD)),
			0x43 | 0x4B | 0x53 | 0x5B | 0x63 | 0x6B | 0x73 | 0x7B => Some(Instruction::BIT(2, 8, (opcode - 0x40) / 8, Regs::RegE)),
			0x44 | 0x4C | 0x54 | 0x5C | 0x64 | 0x6C | 0x74 | 0x7C => Some(Instruction::BIT(2, 8, (opcode - 0x40) / 8, Regs::RegH)),
			0x45 | 0x4D | 0x55 | 0x5D | 0x65 | 0x6D | 0x75 | 0x7D => Some(Instruction::BIT(2, 8, (opcode - 0x40) / 8, Regs::RegL)),
			0x46 | 0x4E | 0x56 | 0x5E | 0x66 | 0x6E | 0x76 | 0x7E => Some(Instruction::BIT(2, 12, (opcode - 0x40) / 8, Regs::HLPointee)),
			0x47 | 0x4F | 0x57 | 0x5F | 0x67 | 0x6F | 0x77 | 0x7F => Some(Instruction::BIT(2, 8, (opcode - 0x40) / 8, Regs::RegA)),
			0x80 | 0x88 | 0x90 | 0x98 | 0xA0 | 0xA8 | 0xB0 | 0xB8 => Some(Instruction::RES(2, 8, (opcode - 0x80) / 8, Regs::RegB)),
			0x81 | 0x89 | 0x91 | 0x99 | 0xA1 | 0xA9 | 0xB1 | 0xB9 => Some(Instruction::RES(2, 8, (opcode - 0x80) / 8, Regs::RegC)),
			0x82 | 0x8A | 0x92 | 0x9A | 0xA2 | 0xAA | 0xB2 | 0xBA => Some(Instruction::RES(2, 8, (opcode - 0x80) / 8, Regs::RegD)),
			0x83 | 0x8B | 0x93 | 0x9B | 0xA3 | 0xAB | 0xB3 | 0xBB => Some(Instruction::RES(2, 8, (opcode - 0x80) / 8, Regs::RegE)),
			0x84 | 0x8C | 0x94 | 0x9C | 0xA4 | 0xAC | 0xB4 | 0xBC => Some(Instruction::RES(2, 8, (opcode - 0x80) / 8, Regs::RegH)),
			0x85 | 0x8D | 0x95 | 0x9D | 0xA5 | 0xAD | 0xB5 | 0xBD => Some(Instruction::RES(2, 8, (opcode - 0x80) / 8, Regs::RegL)),
			0x86 | 0x8E | 0x96 | 0x9E | 0xA6 | 0xAE | 0xB6 | 0xBE => Some(Instruction::RES(2, 16, (opcode - 0x80) / 8, Regs::HLPointee)),
			0x87 | 0x8F | 0x97 | 0x9F | 0xA7 | 0xAF | 0xB7 | 0xBF => Some(Instruction::RES(2, 8, (opcode - 0x80) / 8, Regs::RegA)),
			0xC0 | 0xC8 | 0xD0 | 0xD8 | 0xE0 | 0xE8 | 0xF0 | 0xF8 => Some(Instruction::SET(2, 8, (opcode - 0xC0) / 8, Regs::RegB)),
			0xC1 | 0xC9 | 0xD1 | 0xD9 | 0xE1 | 0xE9 | 0xF1 | 0xF9 => Some(Instruction::SET(2, 8, (opcode - 0xC0) / 8, Regs::RegC)),
			0xC2 | 0xCA | 0xD2 | 0xDA | 0xE2 | 0xEA | 0xF2 | 0xFA => Some(Instruction::SET(2, 8, (opcode - 0xC0) / 8, Regs::RegD)),
			0xC3 | 0xCB | 0xD3 | 0xDB | 0xE3 | 0xEB | 0xF3 | 0xFB => Some(Instruction::SET(2, 8, (opcode - 0xC0) / 8, Regs::RegE)),
			0xC4 | 0xCC | 0xD4 | 0xDC | 0xE4 | 0xEC | 0xF4 | 0xFC => Some(Instruction::SET(2, 8, (opcode - 0xC0) / 8, Regs::RegH)),
			0xC5 | 0xCD | 0xD5 | 0xDD | 0xE5 | 0xED | 0xF5 | 0xFD => Some(Instruction::SET(2, 8, (opcode - 0xC0) / 8, Regs::RegL)),
			0xC6 | 0xCE | 0xD6 | 0xDE | 0xE6 | 0xEE | 0xF6 | 0xFE => Some(Instruction::SET(2, 16, (opcode - 0xC0) / 8, Regs::HLPointee)),
			0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF => Some(Instruction::SET(2, 8, (opcode - 0xC0) / 8, Regs::RegA))
		}
	}
}

impl Cpu {
	pub fn get_reg_value(&mut self, memory_bus: &MemoryBus, reg: Regs) -> u8 {
		match reg {
			Regs::RegA => self.registers.a,
			Regs::RegB => self.registers.b,
			Regs::RegC => self.registers.c,
			Regs::RegD => self.registers.d,
			Regs::RegE => self.registers.e,
			Regs::RegH => self.registers.h,
			Regs::RegL => self.registers.l,
			Regs::HLPointee => self.registers.get_hl_pointee(memory_bus),
			Regs::BCPointee => self.registers.get_bc_pointee(memory_bus),
			Regs::DEPointee => self.registers.get_de_pointee(memory_bus),
			Regs::BytesFromPCPointee => {
				let address = self.fetch_pc(memory_bus) as u16 | ((self.fetch_pc(memory_bus) as u16) << 8);
				memory_bus.read_byte(address)
			}
			Regs::UpperRamOffsetFromPC => {
				let address = 0xFF00 + self.fetch_pc(memory_bus) as u16;
				memory_bus.read_byte(address)
			}
			Regs::UpperRamOffsetFromRegC => memory_bus.read_byte(0xFF00 + self.registers.c as u16),
			Regs::ByteFromPC => self.fetch_pc(memory_bus)
		}
	}
	pub fn set_reg_value(&mut self, memory_bus: &mut MemoryBus, reg: Regs, data: u8) {
		match reg {
			Regs::RegA => {self.registers.a = data}
			Regs::RegB => {self.registers.b = data}
			Regs::RegC => {self.registers.c = data}
			Regs::RegD => {self.registers.d = data}
			Regs::RegE => {self.registers.e = data}
			Regs::RegH => {self.registers.h = data}
			Regs::RegL => {self.registers.l = data}
			Regs::HLPointee => {self.registers.set_hl_pointee(memory_bus, data)}
			Regs::BCPointee => {self.registers.set_bc_pointee(memory_bus, data)},
			Regs::DEPointee => {self.registers.set_de_pointee(memory_bus, data)},
			Regs::BytesFromPCPointee => {
				let address = self.fetch_pc(memory_bus) as u16 | ((self.fetch_pc(memory_bus) as u16) << 8);
				memory_bus.write_byte(address, data)
			}
			Regs::UpperRamOffsetFromPC => {
				let address = 0xFF00 + self.fetch_pc(memory_bus) as u16;
				memory_bus.write_byte(address, data)
			}
			Regs::UpperRamOffsetFromRegC => {memory_bus.write_byte(0xFF00 + self.registers.c as u16, data)},
			Regs::ByteFromPC => {println!("SHOULD NEVER HAPPEN")}
		}
	}
	pub fn get_reg_pair_big_endian_value(&mut self, memory_bus: &MemoryBus, reg_pair: RegPairs) -> u16 {
		match reg_pair {
			RegPairs::RegsAF => self.registers.get_af_big_endian(),
			RegPairs::RegsBC => self.registers.get_bc_big_endian(),
			RegPairs::RegsDE => self.registers.get_de_big_endian(),
			RegPairs::RegsHL => self.registers.get_hl_big_endian(),
			RegPairs::RegSP => self.registers.stack_pointer,
			RegPairs::BytesFromPC => { self.fetch_pc(memory_bus) as u16 | ((self.fetch_pc(memory_bus) as u16) << 8) },
			RegPairs::BytesFromPCPointee => {
				println!("SHOULD NEVER HAPPEN");
				let address = self.fetch_pc(memory_bus) as u16 | ((self.fetch_pc(memory_bus) as u16) << 8);
				memory_bus.read_byte(address) as u16 | ((memory_bus.read_byte(address.overflowing_add(1).0) as u16) << 8)
			},
		}
	}
	pub fn set_reg_pair_big_endian_value(&mut self, memory_bus: &mut MemoryBus, reg_pair: RegPairs, big_endian_value: u16) {
		match reg_pair {
			RegPairs::RegsAF => {self.registers.set_af_big_endian(big_endian_value)}
			RegPairs::RegsBC => {self.registers.set_bc_big_endian(big_endian_value)}
			RegPairs::RegsDE => {self.registers.set_de_big_endian(big_endian_value)}
			RegPairs::RegsHL => {self.registers.set_hl_big_endian(big_endian_value)}
			RegPairs::RegSP => {self.registers.stack_pointer = big_endian_value}
			RegPairs::BytesFromPCPointee => {
				let address = self.fetch_pc(memory_bus) as u16 | ((self.fetch_pc(memory_bus) as u16) << 8);
				memory_bus.write_byte(address, big_endian_value as u8);
				memory_bus.write_byte(address.overflowing_add(1).0, (big_endian_value >> 8) as u8);
			},
			RegPairs::BytesFromPC => {println!("SHOULD NEVER HAPPEN")},
		}
	}
	pub fn execute_op(&mut self, memory_bus: &mut MemoryBus, instruction: Instruction) {
		match instruction {
			Instruction::NOP(_, _) => {}
			Instruction::LD(_, _, target, src) => {
				let data = self.get_reg_value(memory_bus, src);
				self.set_reg_value(memory_bus, target, data);
			}
			Instruction::LDI(_, _, target, src) => {
				let data = self.get_reg_value(memory_bus, src);
				self.set_reg_value(memory_bus, target, data);
				self.registers.set_hl_big_endian(self.registers.get_hl_big_endian().overflowing_add(1).0);
			}
			Instruction::LDD(_, _, target, src) => {
				let data = self.get_reg_value(memory_bus, src);
				self.set_reg_value(memory_bus, target, data);
				self.registers.set_hl_big_endian(self.registers.get_hl_big_endian().overflowing_sub(1).0);
			}
			Instruction::LD16(_, _, target, src) => {
				let data = self.get_reg_pair_big_endian_value(memory_bus, src);
				self.set_reg_pair_big_endian_value(memory_bus, target, data);
			}
			Instruction::LDHLSPe(_, _) => {
				let sp_value = self.registers.stack_pointer;
				let operand_value = self.fetch_pc(memory_bus) as i8 as u16;
				let res = sp_value.overflowing_add(operand_value).0;
				self.registers.f.zero = false;
				self.registers.f.substract = false;
				self.registers.f.half_carry = (sp_value & 0xF) + (operand_value & 0xF) >= 0x10;
				self.registers.f.carry = (sp_value & 0xFF) + (operand_value & 0xFF) >= 0x100;
				self.registers.set_hl_big_endian(res);
			}
			Instruction::PUSH(_, _, target) => {
				let reg_content = self.get_reg_pair_big_endian_value(memory_bus, target);
				memory_bus.write_byte(self.registers.stack_pointer.overflowing_sub(1).0, (reg_content >> 8) as u8);
				memory_bus.write_byte(self.registers.stack_pointer.overflowing_sub(2).0, reg_content as u8);
				self.registers.stack_pointer = self.registers.stack_pointer.overflowing_sub(2).0;
			}
			Instruction::POP(_, _, target) => {
				let mut reg_content = 0x0000 as u16;
				reg_content |= memory_bus.read_byte(self.registers.stack_pointer) as u16;
				reg_content |= (memory_bus.read_byte(self.registers.stack_pointer.overflowing_add(1).0) as u16) << 8;
				self.registers.stack_pointer = self.registers.stack_pointer.overflowing_add(2).0;
				self.set_reg_pair_big_endian_value(memory_bus, target, reg_content);
			}
			Instruction::ADDAs(_, _, _operand) | Instruction::ADCAs(_, _, _operand) => {
				let reg_a_content = self.registers.a as u16;
				let operand_val = self.get_reg_value(memory_bus, _operand) as u16;
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
				let operand_val = self.get_reg_value(memory_bus, _operand) as u16;
				let carry_val = if let Instruction::SBCAs(_, _, _) = instruction {self.registers.f.carry as u16} else {0};
				let r = reg_a_content.overflowing_sub(operand_val + carry_val).0;
				self.registers.f.zero = r as u8 == 0;
				self.registers.f.substract = true;
				self.registers.f.half_carry = reg_a_content & 0xF < (operand_val & 0xF) + carry_val;
				self.registers.f.carry = r & 0x100 != 0;
				self.registers.a = r as u8;
			}
			Instruction::ANDs(_, _, _operand) => {
				let operand_val = self.get_reg_value(memory_bus, _operand);
				self.registers.a &= operand_val;
				self.registers.f.zero = self.registers.a == 0;
				self.registers.f.substract = false;
				self.registers.f.half_carry = true;
				self.registers.f.carry = false;
			}
			Instruction::XORs(_, _, _operand) => {
				let operand_val = self.get_reg_value(memory_bus, _operand);
				self.registers.a ^= operand_val;
				self.registers.f.zero = self.registers.a == 0;
				self.registers.f.substract = false;
				self.registers.f.half_carry = false;
				self.registers.f.carry = false;
			}
			Instruction::ORs(_, _, _operand) => {
				let operand_val = self.get_reg_value(memory_bus, _operand);
				self.registers.a |= operand_val;
				self.registers.f.zero = self.registers.a == 0;
				self.registers.f.substract = false;
				self.registers.f.half_carry = false;
				self.registers.f.carry = false;
			}
			Instruction::CPs(_, _, _operand) => {
				let reg_a_content = self.registers.a as u16;
				let operand_val = self.get_reg_value(memory_bus, _operand) as u16;
				let r = reg_a_content.overflowing_sub(operand_val).0;
				self.registers.f.zero = r as u8 == 0;
				self.registers.f.substract = true;
				self.registers.f.half_carry = (reg_a_content & 0xF) < (operand_val & 0xF);
				self.registers.f.carry = r & 0x100 != 0;
			}
			Instruction::INCs(_, _, _target) => {
				let target_value = self.get_reg_value(memory_bus, _target);
				self.registers.f.zero = target_value == 0xFF;
				self.registers.f.substract = false;
				self.registers.f.half_carry = (target_value & 0xF) + 1 >= 0x10;
				self.set_reg_value(memory_bus, _target, target_value.overflowing_add(1).0);
			}
			Instruction::DECs(_, _, _target) => {
				let target_value = self.get_reg_value(memory_bus, _target);
				self.registers.f.zero = target_value == 0x01;
				self.registers.f.substract = true;
				self.registers.f.half_carry = target_value & 0xF < 1;
				self.set_reg_value(memory_bus, _target, target_value.overflowing_sub(1).0);
			}
			Instruction::INCss(_, _, _target) => {
				let data = self.get_reg_pair_big_endian_value(memory_bus, _target).overflowing_add(1).0;
				self.set_reg_pair_big_endian_value(memory_bus, _target, data);
			}
			Instruction::DECss(_, _, _target) => {
				let data = self.get_reg_pair_big_endian_value(memory_bus, _target).overflowing_sub(1).0;
				self.set_reg_pair_big_endian_value(memory_bus, _target, data);
			}
			Instruction::ADDHLss(_, _, _operand) => {
				let hl_value = self.registers.get_hl_big_endian();
				let operand_value = self.get_reg_pair_big_endian_value(memory_bus, _operand);
				let r = hl_value.overflowing_add(operand_value).0;
				self.registers.f.substract = false;
				self.registers.f.half_carry = (hl_value & 0x0FFF) + (operand_value & 0x0FFF) >= 0x1000;
				self.registers.f.carry = r < hl_value;
				self.registers.set_hl_big_endian(r);
			}
			Instruction::ADDSPe(_, _) => {
				let sp_value = self.registers.stack_pointer;
				let operand_value = self.fetch_pc(memory_bus) as i8 as u16;
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
			Instruction::RLCA(_, _) => {
				self.registers.f.zero = false;
				self.registers.f.substract = false;
				self.registers.f.half_carry = false;
				self.registers.f.carry = false;
				let mut reg_content = (self.registers.a as u16) << 1;
				if (reg_content & 0x0100) != 0 {
					self.registers.f.carry = true;
					reg_content += 1;
				}
				self.registers.a = reg_content as u8;
			}
			Instruction::RLA(_, _) => {
				let mut reg_content = (self.registers.a as u16) << 1;
				reg_content += if self.registers.f.carry {1} else {0};
				self.registers.f.carry = (reg_content & 0x0100) != 0;
				self.registers.f.zero = false;
				self.registers.f.substract = false;
				self.registers.f.half_carry = false;
				self.registers.a = reg_content as u8;
			}
			Instruction::RRCA(_, _) => {
				self.registers.f.zero = false;
				self.registers.f.substract = false;
				self.registers.f.half_carry = false;
				self.registers.f.carry = false;
				let mut reg_content = (self.registers.a as u16) << 7;
				if (reg_content & 0x0080) != 0 {
					self.registers.f.carry = true;
					reg_content += 0x8000;
				}
				self.registers.a = (reg_content >> 8) as u8;
			}
			Instruction::RRA(_, _) => {
				let mut reg_content = (self.registers.a as u16) << 7;
				reg_content += if self.registers.f.carry {0x8000} else {0};
				self.registers.f.carry = (reg_content & 0x0080) != 0;
				self.registers.f.zero = false;
				self.registers.f.substract = false;
				self.registers.f.half_carry = false;
				self.registers.a = (reg_content >> 8) as u8;
			}
			Instruction::RLC(_, _, target) => {
				self.registers.f.substract = false;
				self.registers.f.half_carry = false;
				self.registers.f.carry = false;
				let mut reg_content = (self.get_reg_value(memory_bus, target) as u16) << 1;
				if (reg_content & 0x0100) != 0 {
					self.registers.f.carry = true;
					reg_content += 1;
				}
				self.registers.f.zero = reg_content == 0;
				self.set_reg_value(memory_bus, target, reg_content as u8);
			}
			Instruction::RL(_, _, target) => {
				let mut reg_content = self.get_reg_value(memory_bus, target);
				let had_carry = self.registers.f.carry as u8;
				self.registers.f.carry = (reg_content & 0x80) != 0;
				reg_content = had_carry | (reg_content << 1);
				self.registers.f.zero = reg_content == 0;
				self.registers.f.substract = false;
				self.registers.f.half_carry = false;
				self.set_reg_value(memory_bus, target, reg_content as u8);
			}
			Instruction::RRC(_, _, target) => {
				self.registers.f.substract = false;
				self.registers.f.half_carry = false;
				self.registers.f.carry = false;
				let mut reg_content = (self.get_reg_value(memory_bus, target) as u16) << 7;
				if (reg_content & 0x0080) != 0 {
					self.registers.f.carry = true;
					reg_content += 0x8000;
				}
				self.registers.f.zero = reg_content == 0;
				self.set_reg_value(memory_bus, target, (reg_content >> 8) as u8);
			}
			Instruction::RR(_, _, target) => {
				let mut reg_content = self.get_reg_value(memory_bus, target);
				let had_carry = (self.registers.f.carry as u8) << 7;
				self.registers.f.carry = (reg_content & 0x01) != 0;
				reg_content = had_carry | (reg_content >> 1);
				self.registers.f.zero = reg_content == 0;
				self.registers.f.substract = false;
				self.registers.f.half_carry = false;
				self.set_reg_value(memory_bus, target, reg_content as u8);
			}
			Instruction::SLA(_, _, target) => {
				let mut reg_content = self.get_reg_value(memory_bus, target);
				self.registers.f.carry = (reg_content & 0x80) != 0;
				reg_content = reg_content << 1;
				self.registers.f.zero = reg_content == 0;
				self.registers.f.substract = false;
				self.registers.f.half_carry = false;
				self.set_reg_value(memory_bus, target, reg_content as u8);
			}
			Instruction::SWAP(_, _, target) => {
				let reg_content = self.get_reg_value(memory_bus, target);
				let reg_content = reg_content << 4 | reg_content >> 4;
				self.registers.f.zero = reg_content == 0;
				self.registers.f.substract = false;
				self.registers.f.half_carry = false;
				self.registers.f.carry = false;
				self.set_reg_value(memory_bus, target, reg_content);
			}
			Instruction::SRA(_, _, target) => {
				let mut reg_content = self.get_reg_value(memory_bus, target);
				self.registers.f.carry = (reg_content & 0x01) != 0;
				reg_content = (reg_content & 0x80) | (reg_content >> 1);
				self.registers.f.zero = reg_content == 0;
				self.registers.f.substract = false;
				self.registers.f.half_carry = false;
				self.set_reg_value(memory_bus, target, reg_content as u8);
			}
			Instruction::SRL(_, _, target) => {
				let mut reg_content = self.get_reg_value(memory_bus, target);
				self.registers.f.carry = (reg_content & 0x01) != 0;
				reg_content = reg_content >> 1;
				self.registers.f.zero = reg_content == 0;
				self.registers.f.substract = false;
				self.registers.f.half_carry = false;
				self.set_reg_value(memory_bus, target, reg_content as u8);
			}
			Instruction::BIT(_, _, pos, target) => {
				self.registers.f.substract = false;
				self.registers.f.half_carry = true;
				self.registers.f.zero = (self.get_reg_value(memory_bus, target) & (1 << pos)) == 0;
			}
			Instruction::SET(_, _, pos, target) => {
				let reg_content = self.get_reg_value(memory_bus, target) | (1 << pos);
				self.set_reg_value(memory_bus, target, reg_content);
			}
			Instruction::RES(_, _, pos, target) => {
				let reg_content = self.get_reg_value(memory_bus, target) & !(1 << pos);
				self.set_reg_value(memory_bus, target, reg_content);
			}
			Instruction::JPnn(_, _) => self.registers.program_counter = self.get_reg_pair_big_endian_value(memory_bus, RegPairs::BytesFromPC),
			Instruction::JPHL(_, _) => self.registers.program_counter = self.registers.get_hl_big_endian(),
			Instruction::JPfnn(len, _, condition) => {
				let destination = self.get_reg_pair_big_endian_value(memory_bus, RegPairs::BytesFromPC);
				let mut do_jump = false;
				match condition {
					JumpCondition::NotZero => {if !self.registers.f.zero {do_jump = true};}
					JumpCondition::Zero => {if self.registers.f.zero {do_jump = true};}
					JumpCondition::NotCarry => {if !self.registers.f.carry {do_jump = true};}
					JumpCondition::Carry => {if self.registers.f.carry {do_jump = true};}
				}
				if do_jump {
					self.current_op = Some(Instruction::JPfnn(len, 16, condition));
					self.registers.program_counter = destination;
				}
			}
			Instruction::JR(_, _) => {
				let operand_value = self.fetch_pc(memory_bus) as i8 as u16;
				self.registers.program_counter = self.registers.program_counter.overflowing_add(operand_value).0;
			}
			Instruction::JRf(len, _, condition) => {
				let operand_value = self.fetch_pc(memory_bus) as i8 as u16;
				let mut do_jump = false;
				match condition {
					JumpCondition::NotZero => {if !self.registers.f.zero {do_jump = true};}
					JumpCondition::Zero => {if self.registers.f.zero {do_jump = true};}
					JumpCondition::NotCarry => {if !self.registers.f.carry {do_jump = true};}
					JumpCondition::Carry => {if self.registers.f.carry {do_jump = true};}
				}
				if do_jump {
					self.current_op = Some(Instruction::JRf(len, 12, condition));
					self.registers.program_counter = self.registers.program_counter.overflowing_add(operand_value).0;
				}
			}
			Instruction::CALL(_, _) => {
				let address = self.get_reg_pair_big_endian_value(memory_bus, RegPairs::BytesFromPC);
				memory_bus.write_byte(self.registers.stack_pointer.overflowing_sub(1).0, (self.registers.program_counter >> 8) as u8);
				memory_bus.write_byte(self.registers.stack_pointer.overflowing_sub(2).0, self.registers.program_counter as u8);
				self.registers.stack_pointer = self.registers.stack_pointer.overflowing_sub(2).0;
				self.registers.program_counter = address;
			}
			Instruction::CALLf(len, _, condition) => {
				let address = self.get_reg_pair_big_endian_value(memory_bus, RegPairs::BytesFromPC);
				let mut do_call = false;
				match condition {
					JumpCondition::NotZero => {if !self.registers.f.zero {do_call = true};}
					JumpCondition::Zero => {if self.registers.f.zero {do_call = true};}
					JumpCondition::NotCarry => {if !self.registers.f.carry {do_call = true};}
					JumpCondition::Carry => {if self.registers.f.carry {do_call = true};}
				}
				if do_call {
					self.current_op = Some(Instruction::CALLf(len, 24, condition));
					memory_bus.write_byte(self.registers.stack_pointer.overflowing_sub(1).0, (self.registers.program_counter >> 8) as u8);
					memory_bus.write_byte(self.registers.stack_pointer.overflowing_sub(2).0, self.registers.program_counter as u8);
					self.registers.stack_pointer = self.registers.stack_pointer.overflowing_sub(2).0;
					self.registers.program_counter = address;
				}
			}
			Instruction::ISR(_, _) => {
				let interrupt_enable = memory_bus.read_byte(0xFFFF);
				let interrupt_flag = memory_bus.read_byte(0xFF0F);
				let effective_interrupts = interrupt_enable & interrupt_flag;
				let address =
					if effective_interrupts & (1 << 0) != 0			{memory_bus.write_byte(0xFF0F, interrupt_flag & !(1 << 0)); 0x0040}
					else if effective_interrupts & (1 << 1) != 0	{memory_bus.write_byte(0xFF0F, interrupt_flag & !(1 << 1)); 0x0048}
					else if effective_interrupts & (1 << 2) != 0	{memory_bus.write_byte(0xFF0F, interrupt_flag & !(1 << 2)); 0x0050}
					else if effective_interrupts & (1 << 3) != 0	{memory_bus.write_byte(0xFF0F, interrupt_flag & !(1 << 3)); 0x0058}
					else 											{memory_bus.write_byte(0xFF0F, interrupt_flag & !(1 << 4)); 0x0060};
				memory_bus.write_byte(self.registers.stack_pointer.overflowing_sub(1).0, (self.registers.program_counter >> 8) as u8);
				memory_bus.write_byte(self.registers.stack_pointer.overflowing_sub(2).0, self.registers.program_counter as u8);
				self.registers.stack_pointer = self.registers.stack_pointer.overflowing_sub(2).0;
				self.registers.program_counter = address;
				self.ime_set = false;
			}
			Instruction::RET(_, _) => {
				let mut ret_pc = 0x0000 as u16;
				ret_pc |= memory_bus.read_byte(self.registers.stack_pointer) as u16;
				ret_pc |= (memory_bus.read_byte(self.registers.stack_pointer.overflowing_add(1).0) as u16) << 8;
				self.registers.stack_pointer = self.registers.stack_pointer.overflowing_add(2).0;
				self.registers.program_counter = ret_pc;
			}
			Instruction::RETf(len, _, condition) => {
				let mut do_call = false;
				match condition {
					JumpCondition::NotZero => {if !self.registers.f.zero {do_call = true};}
					JumpCondition::Zero => {if self.registers.f.zero {do_call = true};}
					JumpCondition::NotCarry => {if !self.registers.f.carry {do_call = true};}
					JumpCondition::Carry => {if self.registers.f.carry {do_call = true};}
				}
				if do_call {
					self.current_op = Some(Instruction::RETf(len, 20, condition));
					let mut ret_pc = 0x0000 as u16;
					ret_pc |= memory_bus.read_byte(self.registers.stack_pointer) as u16;
					ret_pc |= (memory_bus.read_byte(self.registers.stack_pointer.overflowing_add(1).0) as u16) << 8;
					self.registers.stack_pointer = self.registers.stack_pointer.overflowing_add(2).0;
					self.registers.program_counter = ret_pc;
				}
			}
			Instruction::RETI(_, _) => {
				let mut ret_pc = 0x0000 as u16;
				ret_pc |= memory_bus.read_byte(self.registers.stack_pointer) as u16;
				ret_pc |= (memory_bus.read_byte(self.registers.stack_pointer.overflowing_add(1).0) as u16) << 8;
				self.registers.stack_pointer = self.registers.stack_pointer.overflowing_add(2).0;
				self.registers.program_counter = ret_pc;
				self.ime_set = true;
			},
			Instruction::RST(_, _, location) => {
				let location = match location {
					ResetLocation::Hex00 => 0x00 as u16,
					ResetLocation::Hex08 => 0x08 as u16,
					ResetLocation::Hex10 => 0x10 as u16,
					ResetLocation::Hex18 => 0x18 as u16,
					ResetLocation::Hex20 => 0x20 as u16,
					ResetLocation::Hex28 => 0x28 as u16,
					ResetLocation::Hex30 => 0x30 as u16,
					ResetLocation::Hex38 => 0x38 as u16,
				};
				memory_bus.write_byte(self.registers.stack_pointer.overflowing_sub(1).0, (self.registers.program_counter >> 8) as u8);
				memory_bus.write_byte(self.registers.stack_pointer.overflowing_sub(2).0, self.registers.program_counter as u8);
				self.registers.stack_pointer = self.registers.stack_pointer.overflowing_sub(2).0;
				self.registers.program_counter = location;
			}
			Instruction::HALT(_, _) => {self.state = CpuState::Halted}
			Instruction::STOP(_, _) => {
				if memory_bus.is_cgb && memory_bus.speed_chg_scheduled {
					memory_bus.is_double_speed = !memory_bus.is_double_speed;
					memory_bus.speed_chg_scheduled = false;
				}
				self.fetch_pc(memory_bus);
			}
			Instruction::DI(_, _) => {self.ime_set = false}
			Instruction::EI(_, _) => {self.ime_scheduled = true}
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::{memory_bus::MemoryBus, cpu::{Cpu, {registers::FlagsRegister, instructions::RegPairs}}};
	use super::{Instruction, Regs};

	fn test_adds(cpu: &mut Cpu, memory_bus: &mut MemoryBus, init_a_value: u8, expected_res: u8, expected_flag_reg: FlagsRegister) {
		cpu.current_op = Some(Instruction::ADDAs(1, 4, Regs::RegA));
		cpu.registers.a = init_a_value;
		cpu.exec_current_op(memory_bus);
		assert_eq!(cpu.registers.a, expected_res);
		assert_eq!(cpu.registers.f, expected_flag_reg);
	}
	fn test_sub(cpu: &mut Cpu, memory_bus: &mut MemoryBus, init_a_value: u8, operand: u8, expected_res: u8, expected_flag_reg: FlagsRegister) {
		cpu.current_op = Some(Instruction::SUBs(1, 4, Regs::RegB));
		cpu.registers.a = init_a_value;
		cpu.registers.b = operand;
		cpu.exec_current_op(memory_bus);
		assert_eq!(cpu.registers.a, expected_res);
		assert_eq!(cpu.registers.f, expected_flag_reg);
	}
	fn test_cps(cpu: &mut Cpu, memory_bus: &mut MemoryBus, init_a_value: u8, operand: u8, expected_flag_reg: FlagsRegister) {
		cpu.current_op = Some(Instruction::CPs(1, 4, Regs::RegB));
		cpu.registers.a = init_a_value;
		cpu.registers.b = operand;
		cpu.exec_current_op(memory_bus);
		assert_eq!(cpu.registers.f, expected_flag_reg);
	}
	fn test_addhlss(cpu: &mut Cpu, memory_bus: &mut MemoryBus, init_hl_value: u16, expected_res: u16, expected_flag_reg: FlagsRegister) {
		cpu.current_op = Some(Instruction::ADDHLss(1, 8, RegPairs::RegsHL));
		cpu.registers.set_hl_big_endian(init_hl_value);
		cpu.exec_current_op(memory_bus);
		assert_eq!(cpu.registers.get_hl_big_endian(), expected_res);
		assert_eq!(cpu.registers.f, expected_flag_reg);
	}
	fn test_addspe(cpu: &mut Cpu, memory_bus: &mut MemoryBus, init_sp_value: u16, operand: i8, expected_res: u16, expected_flag_reg: FlagsRegister) {
		cpu.current_op = Some(Instruction::ADDSPe(2, 16));
		cpu.registers.stack_pointer = init_sp_value;
		memory_bus.write_byte(cpu.registers.program_counter, operand as u8);
		cpu.exec_current_op(memory_bus);
		assert_eq!(cpu.registers.stack_pointer, expected_res);
		assert_eq!(cpu.registers.f, expected_flag_reg);
	}
	fn test_daa(cpu: &mut Cpu, memory_bus: &mut MemoryBus, expected_res: u8, expected_flag_reg: FlagsRegister) {
		cpu.current_op = Some(Instruction::DAA(1, 4));
		cpu.exec_current_op(memory_bus);
		assert_eq!(cpu.registers.a, expected_res);
		assert_eq!(cpu.registers.f, expected_flag_reg);
	}
	#[test]
	fn test_arith() {
		let mut memory_bus = MemoryBus::new(None, false);
		let mut my_cpu = Cpu::new();
		my_cpu.registers.program_counter = 0xC000;
		test_adds(&mut my_cpu, &mut memory_bus, 0x12, 0x24, 0x00.into());
		test_adds(&mut my_cpu, &mut memory_bus, 0x80, 0x00, FlagsRegister{ zero: true, substract: false, half_carry: false, carry: true });
		test_adds(&mut my_cpu, &mut memory_bus, 0xF1, 0xE2, FlagsRegister{ zero: false, substract: false, half_carry: false, carry: true });
		test_adds(&mut my_cpu, &mut memory_bus, 0xFF, 0xFE, FlagsRegister{ zero: false, substract: false, half_carry: true, carry: true });
		test_sub(&mut my_cpu, &mut memory_bus, 0xFF, 0x10, 0xEF, FlagsRegister{ zero: false, substract: true, half_carry: false, carry: false });
		test_sub(&mut my_cpu, &mut memory_bus, 0xFF, 0xFF, 0x00, FlagsRegister{ zero: true, substract: true, half_carry: false, carry: false });
		test_sub(&mut my_cpu, &mut memory_bus, 0xF1, 0x0F, 0xE2, FlagsRegister{ zero: false, substract: true, half_carry: true, carry: false });
		test_sub(&mut my_cpu, &mut memory_bus, 0x10, 0x20, 0xF0, FlagsRegister{ zero: false, substract: true, half_carry: false, carry: true });
		test_sub(&mut my_cpu, &mut memory_bus, 0x10, 0x21, 0xEF, FlagsRegister{ zero: false, substract: true, half_carry: true, carry: true });
		test_cps(&mut my_cpu, &mut memory_bus, 0xFF, 0x10, FlagsRegister{ zero: false, substract: true, half_carry: false, carry: false });
		test_cps(&mut my_cpu, &mut memory_bus, 0xFF, 0xFF, FlagsRegister{ zero: true, substract: true, half_carry: false, carry: false });
		test_cps(&mut my_cpu, &mut memory_bus, 0xF1, 0x0F, FlagsRegister{ zero: false, substract: true, half_carry: true, carry: false });
		test_cps(&mut my_cpu, &mut memory_bus, 0x10, 0x20, FlagsRegister{ zero: false, substract: true, half_carry: false, carry: true });
		test_cps(&mut my_cpu, &mut memory_bus, 0x10, 0x21, FlagsRegister{ zero: false, substract: true, half_carry: true, carry: true });

		test_addhlss(&mut my_cpu, &mut memory_bus, 0x8A23, 0x1446, FlagsRegister{ zero: false, substract: false, half_carry: true, carry: true });
		test_addhlss(&mut my_cpu, &mut memory_bus, 0x0000, 0x0000, FlagsRegister{ zero: false, substract: false, half_carry: false, carry: false });
		test_addspe(&mut my_cpu, &mut memory_bus, 0xFFF8, 0x02, 0xFFFA, FlagsRegister{ zero: false, substract: false, half_carry: false, carry: false });
		test_addspe(&mut my_cpu, &mut memory_bus, 0xFF88, 0x0F, 0xFF97, FlagsRegister{ zero: false, substract: false, half_carry: true, carry: false });
		test_addspe(&mut my_cpu, &mut memory_bus, 0xF8D8, 0x2F, 0xF907, FlagsRegister{ zero: false, substract: false, half_carry: true, carry: true });
		test_addspe(&mut my_cpu, &mut memory_bus, 0xF8D8, -0x24, 0xF8B4, FlagsRegister{ zero: false, substract: false, half_carry: true, carry: true });

		test_adds(&mut my_cpu, &mut memory_bus, 0x45, 0x8A, FlagsRegister{ zero: false, substract: false, half_carry: false, carry: false });
		test_daa(&mut my_cpu, &mut memory_bus, 0x90, FlagsRegister{ zero: false, substract: false, half_carry: false, carry: false });
		test_adds(&mut my_cpu, &mut memory_bus, 0x91, 0x22, FlagsRegister{ zero: false, substract: false, half_carry: false, carry: true });
		test_daa(&mut my_cpu, &mut memory_bus, 0x82, FlagsRegister{ zero: false, substract: false, half_carry: false, carry: true });
		test_sub(&mut my_cpu, &mut memory_bus, 0x83, 0x38, 0x4B, FlagsRegister{ zero: false, substract: true, half_carry: true, carry: false });
		test_daa(&mut my_cpu, &mut memory_bus, 0x45, FlagsRegister{ zero: false, substract: true, half_carry: false, carry: false });
	}
}