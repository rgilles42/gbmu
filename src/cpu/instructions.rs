use super::Cpu;

type InstrLength = u8;
type InstrCycles = u8;

#[derive(Debug, Clone, Copy)]
pub enum ArithmeticOperand{
	RegA, RegB, RegC, RegD, RegE, RegH, RegL, HLPointee, RawByte
}
#[derive(Debug, Clone, Copy)]
pub enum IncDecTarget{
	RegA, RegB, RegC, RegD, RegE, RegH, RegL, HLPointee
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
	NOP(InstrLength, InstrCycles),
	ADDAs(InstrLength, InstrCycles, ArithmeticOperand),
	ADCAs(InstrLength, InstrCycles, ArithmeticOperand),
	SUBs(InstrLength, InstrCycles, ArithmeticOperand),
	SBCAs(InstrLength, InstrCycles, ArithmeticOperand),
	ANDs(InstrLength, InstrCycles, ArithmeticOperand),
	XORs(InstrLength, InstrCycles, ArithmeticOperand),
	ORs(InstrLength, InstrCycles, ArithmeticOperand),
	CPs(InstrLength, InstrCycles, ArithmeticOperand),
	INCs(InstrLength, InstrCycles, IncDecTarget),
	DECs(InstrLength, InstrCycles, IncDecTarget),
}

impl Instruction {
	pub fn from_opcode(opcode: u8) -> Option<Instruction> {
		match opcode {
			0x00 => Some(Instruction::NOP(1, 4)),
			0x04 => Some(Instruction::INCs(1, 4, IncDecTarget::RegB)),
			0x05 => Some(Instruction::DECs(1, 4, IncDecTarget::RegB)),
			0x0C => Some(Instruction::INCs(1, 4, IncDecTarget::RegC)),
			0x0D => Some(Instruction::DECs(1, 4, IncDecTarget::RegC)),
			0x14 => Some(Instruction::INCs(1, 4, IncDecTarget::RegD)),
			0x15 => Some(Instruction::DECs(1, 4, IncDecTarget::RegD)),
			0x1C => Some(Instruction::INCs(1, 4, IncDecTarget::RegE)),
			0x1D => Some(Instruction::DECs(1, 4, IncDecTarget::RegE)),
			0x24 => Some(Instruction::INCs(1, 4, IncDecTarget::RegH)),
			0x25 => Some(Instruction::DECs(1, 4, IncDecTarget::RegH)),
			0x2C => Some(Instruction::INCs(1, 4, IncDecTarget::RegL)),
			0x2D => Some(Instruction::DECs(1, 4, IncDecTarget::RegL)),
			0x34 => Some(Instruction::INCs(1, 12, IncDecTarget::HLPointee)),
			0x35 => Some(Instruction::DECs(1, 12, IncDecTarget::HLPointee)),
			0x3C => Some(Instruction::INCs(1, 4, IncDecTarget::RegA)),
			0x3D => Some(Instruction::DECs(1, 4, IncDecTarget::RegA)),

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

			0xC6 => Some(Instruction::ADDAs(2, 8, ArithmeticOperand::RawByte)),
			0xCE => Some(Instruction::ADCAs(2, 8, ArithmeticOperand::RawByte)),
			0xD6 => Some(Instruction::SUBs(2, 8, ArithmeticOperand::RawByte)),
			0xDE => Some(Instruction::SBCAs(2, 8, ArithmeticOperand::RawByte)),
			0xE6 => Some(Instruction::ANDs(2, 8, ArithmeticOperand::RawByte)),
			0xEE => Some(Instruction::XORs(2, 8, ArithmeticOperand::RawByte)),
			0xF6 => Some(Instruction::ORs(2, 8, ArithmeticOperand::RawByte)),
			0xFE => Some(Instruction::CPs(2, 8, ArithmeticOperand::RawByte)),
			_ => None
		}
	}
}

impl Cpu {
	pub fn get_arith_operand_value(&mut self, operand: ArithmeticOperand) -> u8 {
		match operand {
			ArithmeticOperand::RegA => self.registers.a,
			ArithmeticOperand::RegB => self.registers.b,
			ArithmeticOperand::RegC => self.registers.c,
			ArithmeticOperand::RegD => self.registers.d,
			ArithmeticOperand::RegE => self.registers.e,
			ArithmeticOperand::RegH => self.registers.h,
			ArithmeticOperand::RegL => self.registers.l,
			ArithmeticOperand::HLPointee => self.registers.get_hl_pointee(&self.memory_bus),
			ArithmeticOperand::RawByte => self.fetch_pc()
		}
	}
	pub fn get_incdec_target_value(&self, target: IncDecTarget) -> u8 {
		match target {
			IncDecTarget::RegA => self.registers.a,
			IncDecTarget::RegB => self.registers.b,
			IncDecTarget::RegC => self.registers.c,
			IncDecTarget::RegD => self.registers.d,
			IncDecTarget::RegE => self.registers.e,
			IncDecTarget::RegH => self.registers.h,
			IncDecTarget::RegL => self.registers.l,
			IncDecTarget::HLPointee => self.registers.get_hl_pointee(&self.memory_bus),
		}
	}
	pub fn set_incdec_target_value(&mut self, target: IncDecTarget, data: u8) {
		match target {
			IncDecTarget::RegA => {self.registers.a = data}
			IncDecTarget::RegB => {self.registers.b = data}
			IncDecTarget::RegC => {self.registers.c = data}
			IncDecTarget::RegD => {self.registers.d = data}
			IncDecTarget::RegE => {self.registers.e = data}
			IncDecTarget::RegH => {self.registers.h = data}
			IncDecTarget::RegL => {self.registers.l = data}
			IncDecTarget::HLPointee => {self.registers.set_hl_pointee(&mut self.memory_bus, data)}
		}
	}
	pub fn execute_op(&mut self, instruction: Instruction) {
		match instruction {
			Instruction::NOP(_, _) => {}
			Instruction::ADDAs(_, _, _operand) | Instruction::ADCAs(_, _, _operand) => {
				let reg_a_content = self.registers.a as u16;
				let operand_val = self.get_arith_operand_value(_operand) as u16;
				// if instr is ADC and carry flag is true, let carry = 1;
				let carry_val = if let Instruction::ADCAs(_, _, _) = instruction {self.registers.f.carry as u16} else {0};
				let r = reg_a_content + operand_val + carry_val;
				self.registers.f.zero = r == 0;
				self.registers.f.substract = false;
				self.registers.f.half_carry = (reg_a_content & 0xF) + (operand_val & 0xF) + carry_val >= 0x10;
				self.registers.f.carry = r & 0x100 != 0;					// reg_a_content + operand + carry >= 0x100
				self.registers.a = r as u8;
			}
			Instruction::SUBs(_, _, _operand) | Instruction::SBCAs(_, _, _operand) => {
				let reg_a_content = self.registers.a as u16;
				let operand_val = self.get_arith_operand_value(_operand) as u16;
				// if instr is ADC and carry flag is true, let carry = 1;
				let carry_val = if let Instruction::SBCAs(_, _, _) = instruction {self.registers.f.carry as u16} else {0};
				let r = reg_a_content - operand_val - carry_val;
				self.registers.f.zero = r == 0;
				self.registers.f.substract = true;
				self.registers.f.half_carry = (reg_a_content & 0xF) - carry_val < (operand_val & 0xF);	// carry val is subs befor comp, from what I got of the nintendo manual
				self.registers.f.carry = r & 0x100 != 0; 					// reg_a_content < operand + carry; In unsigned logic, a borrow from the next unset bit sets it
				self.registers.a = r as u8;
			}
			Instruction::ANDs(_, _, _operand) => {
				let operand_val = self.get_arith_operand_value(_operand);
				self.registers.a &= operand_val;
				self.registers.f.zero = self.registers.a == 0;
				self.registers.f.substract = false;
				self.registers.f.half_carry = true;
				self.registers.f.carry = false;
			}
			Instruction::XORs(_, _, _operand) => {
				let operand_val = self.get_arith_operand_value(_operand);
				self.registers.a ^= operand_val;
				self.registers.f.zero = self.registers.a == 0;
				self.registers.f.substract = false;
				self.registers.f.half_carry = false;
				self.registers.f.carry = false;
			}
			Instruction::ORs(_, _, _operand) => {
				let operand_val = self.get_arith_operand_value(_operand);
				self.registers.a |= operand_val;
				self.registers.f.zero = self.registers.a == 0;
				self.registers.f.substract = false;
				self.registers.f.half_carry = false;
				self.registers.f.carry = false;
			}
			Instruction::CPs(_, _, _operand) => {
				let reg_a_content = self.registers.a as u16;
				let operand_val = self.get_arith_operand_value(_operand) as u16;
				let r = reg_a_content - operand_val;
				self.registers.f.zero = r == 0;
				self.registers.f.substract = true;
				self.registers.f.half_carry = (reg_a_content & 0xF) < (operand_val & 0xF);
				self.registers.f.carry = r & 0x100 != 0;
			}
			Instruction::INCs(_, _, _target) => {
				let target_value = self.get_incdec_target_value(_target);
				self.registers.f.zero = target_value == 0xFF;
				self.registers.f.substract = false;
				self.registers.f.half_carry = (target_value & 0xF) + 1 >= 0x10;
				self.set_incdec_target_value(_target, target_value + 1);
			}
			Instruction::DECs(_, _, _target) => {
				let target_value = self.get_incdec_target_value(_target);
				self.registers.f.zero = target_value == 0x01;
				self.registers.f.substract = true;
				self.registers.f.half_carry = target_value & 0xF < 1;
				self.set_incdec_target_value(_target, target_value - 1);
			}
		}
	}
}