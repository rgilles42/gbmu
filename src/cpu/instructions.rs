use super::Cpu;

type InstrLength = u8;
type InstrCycles = u8;

#[derive(Debug, Clone, Copy)]
pub enum ArithmeticTarget{
	RegA, RegB, RegC, RegD, RegE, RegH, RegL, HLPointee, RawByte
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
	NOP(InstrLength, InstrCycles),
	ADDAs(InstrLength, InstrCycles, ArithmeticTarget),
	ADCAs(InstrLength, InstrCycles, ArithmeticTarget),
	SUBs(InstrLength, InstrCycles, ArithmeticTarget),
	SBCAs(InstrLength, InstrCycles, ArithmeticTarget),
	ANDs(InstrLength, InstrCycles, ArithmeticTarget),
	XORs(InstrLength, InstrCycles, ArithmeticTarget),
	ORs(InstrLength, InstrCycles, ArithmeticTarget),
	CPs(InstrLength, InstrCycles, ArithmeticTarget),
}

impl Instruction {
	pub fn from_opcode(opcode: u8) -> Option<Instruction> {
		match opcode {
			0x00 => Some(Instruction::NOP(1, 4)),
			0x80 => Some(Instruction::ADDAs(1, 4, ArithmeticTarget::RegB)),
			0x81 => Some(Instruction::ADDAs(1, 4, ArithmeticTarget::RegC)),
			0x82 => Some(Instruction::ADDAs(1, 4, ArithmeticTarget::RegD)),
			0x83 => Some(Instruction::ADDAs(1, 4, ArithmeticTarget::RegE)),
			0x84 => Some(Instruction::ADDAs(1, 4, ArithmeticTarget::RegH)),
			0x85 => Some(Instruction::ADDAs(1, 4, ArithmeticTarget::RegL)),
			0x86 => Some(Instruction::ADDAs(1, 8, ArithmeticTarget::HLPointee)),
			0x87 => Some(Instruction::ADDAs(1, 4, ArithmeticTarget::RegA)),
			0x88 => Some(Instruction::ADCAs(1, 4, ArithmeticTarget::RegB)),
			0x89 => Some(Instruction::ADCAs(1, 4, ArithmeticTarget::RegC)),
			0x8A => Some(Instruction::ADCAs(1, 4, ArithmeticTarget::RegD)),
			0x8B => Some(Instruction::ADCAs(1, 4, ArithmeticTarget::RegE)),
			0x8C => Some(Instruction::ADCAs(1, 4, ArithmeticTarget::RegH)),
			0x8D => Some(Instruction::ADCAs(1, 4, ArithmeticTarget::RegL)),
			0x8E => Some(Instruction::ADCAs(1, 8, ArithmeticTarget::HLPointee)),
			0x8F => Some(Instruction::ADCAs(1, 4, ArithmeticTarget::RegA)),
			0x90 => Some(Instruction::SUBs(1, 4, ArithmeticTarget::RegB)),
			0x91 => Some(Instruction::SUBs(1, 4, ArithmeticTarget::RegC)),
			0x92 => Some(Instruction::SUBs(1, 4, ArithmeticTarget::RegD)),
			0x93 => Some(Instruction::SUBs(1, 4, ArithmeticTarget::RegE)),
			0x94 => Some(Instruction::SUBs(1, 4, ArithmeticTarget::RegH)),
			0x95 => Some(Instruction::SUBs(1, 4, ArithmeticTarget::RegL)),
			0x96 => Some(Instruction::SUBs(1, 8, ArithmeticTarget::HLPointee)),
			0x97 => Some(Instruction::SUBs(1, 4, ArithmeticTarget::RegA)),
			0x98 => Some(Instruction::SBCAs(1, 4, ArithmeticTarget::RegB)),
			0x99 => Some(Instruction::SBCAs(1, 4, ArithmeticTarget::RegC)),
			0x9A => Some(Instruction::SBCAs(1, 4, ArithmeticTarget::RegD)),
			0x9B => Some(Instruction::SBCAs(1, 4, ArithmeticTarget::RegE)),
			0x9C => Some(Instruction::SBCAs(1, 4, ArithmeticTarget::RegH)),
			0x9D => Some(Instruction::SBCAs(1, 4, ArithmeticTarget::RegL)),
			0x9E => Some(Instruction::SBCAs(1, 8, ArithmeticTarget::HLPointee)),
			0x9F => Some(Instruction::SBCAs(1, 4, ArithmeticTarget::RegA)),
			0xA0 => Some(Instruction::ANDs(1, 4, ArithmeticTarget::RegB)),
			0xA1 => Some(Instruction::ANDs(1, 4, ArithmeticTarget::RegC)),
			0xA2 => Some(Instruction::ANDs(1, 4, ArithmeticTarget::RegD)),
			0xA3 => Some(Instruction::ANDs(1, 4, ArithmeticTarget::RegE)),
			0xA4 => Some(Instruction::ANDs(1, 4, ArithmeticTarget::RegH)),
			0xA5 => Some(Instruction::ANDs(1, 4, ArithmeticTarget::RegL)),
			0xA6 => Some(Instruction::ANDs(1, 8, ArithmeticTarget::HLPointee)),
			0xA7 => Some(Instruction::ANDs(1, 4, ArithmeticTarget::RegA)),
			0xA8 => Some(Instruction::XORs(1, 4, ArithmeticTarget::RegB)),
			0xA9 => Some(Instruction::XORs(1, 4, ArithmeticTarget::RegC)),
			0xAA => Some(Instruction::XORs(1, 4, ArithmeticTarget::RegD)),
			0xAB => Some(Instruction::XORs(1, 4, ArithmeticTarget::RegE)),
			0xAC => Some(Instruction::XORs(1, 4, ArithmeticTarget::RegH)),
			0xAD => Some(Instruction::XORs(1, 4, ArithmeticTarget::RegL)),
			0xAE => Some(Instruction::XORs(1, 8, ArithmeticTarget::HLPointee)),
			0xAF => Some(Instruction::XORs(1, 4, ArithmeticTarget::RegA)),
			0xB0 => Some(Instruction::ORs(1, 4, ArithmeticTarget::RegB)),
			0xB1 => Some(Instruction::ORs(1, 4, ArithmeticTarget::RegC)),
			0xB2 => Some(Instruction::ORs(1, 4, ArithmeticTarget::RegD)),
			0xB3 => Some(Instruction::ORs(1, 4, ArithmeticTarget::RegE)),
			0xB4 => Some(Instruction::ORs(1, 4, ArithmeticTarget::RegH)),
			0xB5 => Some(Instruction::ORs(1, 4, ArithmeticTarget::RegL)),
			0xB6 => Some(Instruction::ORs(1, 8, ArithmeticTarget::HLPointee)),
			0xB7 => Some(Instruction::ORs(1, 4, ArithmeticTarget::RegA)),
			0xB8 => Some(Instruction::CPs(1, 4, ArithmeticTarget::RegB)),
			0xB9 => Some(Instruction::CPs(1, 4, ArithmeticTarget::RegC)),
			0xBA => Some(Instruction::CPs(1, 4, ArithmeticTarget::RegD)),
			0xBB => Some(Instruction::CPs(1, 4, ArithmeticTarget::RegE)),
			0xBC => Some(Instruction::CPs(1, 4, ArithmeticTarget::RegH)),
			0xBD => Some(Instruction::CPs(1, 4, ArithmeticTarget::RegL)),
			0xBE => Some(Instruction::CPs(1, 8, ArithmeticTarget::HLPointee)),
			0xBF => Some(Instruction::CPs(1, 4, ArithmeticTarget::RegA)),

			0xC6 => Some(Instruction::ADDAs(2, 8, ArithmeticTarget::RawByte)),
			0xCE => Some(Instruction::ADCAs(2, 8, ArithmeticTarget::RawByte)),
			0xD6 => Some(Instruction::SUBs(2, 8, ArithmeticTarget::RawByte)),
			0xDE => Some(Instruction::SBCAs(2, 8, ArithmeticTarget::RawByte)),
			0xE6 => Some(Instruction::ANDs(2, 8, ArithmeticTarget::RawByte)),
			0xEE => Some(Instruction::XORs(2, 8, ArithmeticTarget::RawByte)),
			0xF6 => Some(Instruction::ORs(2, 8, ArithmeticTarget::RawByte)),
			0xFE => Some(Instruction::CPs(2, 8, ArithmeticTarget::RawByte)),
			_ => None
		}
	}
}

impl Cpu {
	pub fn get_arith_target_value(&mut self, target: ArithmeticTarget) -> u8 {
		match target {
			ArithmeticTarget::RegA => self.registers.a,
			ArithmeticTarget::RegB => self.registers.b,
			ArithmeticTarget::RegC => self.registers.c,
			ArithmeticTarget::RegD => self.registers.d,
			ArithmeticTarget::RegE => self.registers.e,
			ArithmeticTarget::RegH => self.registers.h,
			ArithmeticTarget::RegL => self.registers.l,
			ArithmeticTarget::HLPointee => self.registers.get_hl_pointee(&self.memory_bus),
			ArithmeticTarget::RawByte => self.fetch_pc()
		}
	}
	pub fn execute_op(&mut self, instruction: Instruction) {
		match instruction {
			Instruction::NOP(_, _) => {}
			Instruction::ADDAs(_, _, target) | Instruction::ADCAs(_, _, target) => {
				let reg_a_content = self.registers.a as u16;
				let target_content = self.get_arith_target_value(target) as u16;
				// if instr is ADC and carry flag is true, let carry = 1;
				let carry_val = if let Instruction::ADCAs(_, _, _) = instruction {self.registers.f.carry as u16} else {0};
				let r = reg_a_content + target_content + carry_val;
				self.registers.f.zero = r == 0;
				self.registers.f.substract = false;
				self.registers.f.half_carry = (reg_a_content & 0xF) + (target_content & 0xF) + carry_val >= 0x10;
				self.registers.f.carry = r & 0x100 != 0;					// reg_a_content + target_content + carry >= 0x100
				self.registers.a = r as u8;
			}
			Instruction::SUBs(_, _, target) | Instruction::SBCAs(_, _, target) => {
				let reg_a_content = self.registers.a as u16;
				let target_content = self.get_arith_target_value(target) as u16;
				// if instr is ADC and carry flag is true, let carry = 1;
				let carry_val = if let Instruction::SBCAs(_, _, _) = instruction {self.registers.f.carry as u16} else {0};
				let r = reg_a_content - target_content - carry_val;
				self.registers.f.zero = r == 0;
				self.registers.f.substract = true;
				self.registers.f.half_carry = (reg_a_content & 0xF) - carry_val < (target_content & 0xF);	// carry val is subs befor comp, from what I got of the nintendo manual
				self.registers.f.carry = r & 0x100 != 0; 					// reg_a_content < target_content + carry; In unsigned logic, a borrow from the next unset bit sets it
				self.registers.a = r as u8;
			}
			Instruction::ANDs(_, _, target) => {
				let target_content = self.get_arith_target_value(target);
				self.registers.a &= target_content;
				self.registers.f.zero = self.registers.a == 0;
				self.registers.f.substract = false;
				self.registers.f.half_carry = true;
				self.registers.f.carry = false
			}
			Instruction::XORs(_, _, target) => {
				let target_content = self.get_arith_target_value(target);
				self.registers.a ^= target_content;
				self.registers.f.zero = self.registers.a == 0;
				self.registers.f.substract = false;
				self.registers.f.half_carry = false;
				self.registers.f.carry = false
			}
			Instruction::ORs(_, _, target) => {
				let target_content = self.get_arith_target_value(target);
				self.registers.a |= target_content;
				self.registers.f.zero = self.registers.a == 0;
				self.registers.f.substract = false;
				self.registers.f.half_carry = false;
				self.registers.f.carry = false
			}
			Instruction::CPs(_, _, target) => {
				let reg_a_content = self.registers.a as u16;
				let target_content = self.get_arith_target_value(target) as u16;
				let r = reg_a_content - target_content;
				self.registers.f.zero = r == 0;
				self.registers.f.substract = true;
				self.registers.f.half_carry = (reg_a_content & 0xF) < (target_content & 0xF);
				self.registers.f.carry = r & 0x100 != 0;
			}
		}
	}
}