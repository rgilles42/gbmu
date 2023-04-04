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
pub enum TargetReg{
	RegA, RegB, RegC, RegD, RegE, RegH, RegL, HLPointee
}
#[derive(Debug, Clone, Copy)]
pub enum TargetRegPair{
	RegsBC, RegsDE, RegsHL, RegSP
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
	NOP(InstrLength, InstrCycles),
	INCs(InstrLength, InstrCycles, TargetReg),
	INCss(InstrLength, InstrCycles, TargetRegPair),
	DECs(InstrLength, InstrCycles, TargetReg),
	DECss(InstrLength, InstrCycles, TargetRegPair),
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
			0x03 => Some(Instruction::INCss(1, 8, TargetRegPair::RegsBC)),
			0x04 => Some(Instruction::INCs(1, 4, TargetReg::RegB)),
			0x05 => Some(Instruction::DECs(1, 4, TargetReg::RegB)),
			0x09 => Some(Instruction::ADDHLss(1, 8, LargeArithmeticOperand::RegsBC)),
			0x0B => Some(Instruction::DECss(1, 8, TargetRegPair::RegsBC)),
			0x0C => Some(Instruction::INCs(1, 4, TargetReg::RegC)),
			0x0D => Some(Instruction::DECs(1, 4, TargetReg::RegC)),
			0x13 => Some(Instruction::INCss(1, 8, TargetRegPair::RegsDE)),
			0x14 => Some(Instruction::INCs(1, 4, TargetReg::RegD)),
			0x15 => Some(Instruction::DECs(1, 4, TargetReg::RegD)),
			0x19 => Some(Instruction::ADDHLss(1, 8, LargeArithmeticOperand::RegsDE)),
			0x1B => Some(Instruction::DECss(1, 8, TargetRegPair::RegsDE)),
			0x1C => Some(Instruction::INCs(1, 4, TargetReg::RegE)),
			0x1D => Some(Instruction::DECs(1, 4, TargetReg::RegE)),
			0x23 => Some(Instruction::INCss(1, 8, TargetRegPair::RegsHL)),
			0x24 => Some(Instruction::INCs(1, 4, TargetReg::RegH)),
			0x25 => Some(Instruction::DECs(1, 4, TargetReg::RegH)),
			0x27 => Some(Instruction::DAA(1, 4)),
			0x29 => Some(Instruction::ADDHLss(1, 8, LargeArithmeticOperand::RegsHL)),
			0x2B => Some(Instruction::DECss(1, 8, TargetRegPair::RegsHL)),
			0x2C => Some(Instruction::INCs(1, 4, TargetReg::RegL)),
			0x2D => Some(Instruction::DECs(1, 4, TargetReg::RegL)),
			0x2F => Some(Instruction::CPL(1, 4)),
			0x33 => Some(Instruction::INCss(1, 8, TargetRegPair::RegSP)),
			0x34 => Some(Instruction::INCs(1, 12, TargetReg::HLPointee)),
			0x35 => Some(Instruction::DECs(1, 12, TargetReg::HLPointee)),
			0x37 => Some(Instruction::SCF(1, 4)),
			0x39 => Some(Instruction::ADDHLss(1, 8, LargeArithmeticOperand::RegSP)),
			0x3B => Some(Instruction::DECss(1, 8, TargetRegPair::RegSP)),
			0x3C => Some(Instruction::INCs(1, 4, TargetReg::RegA)),
			0x3D => Some(Instruction::DECs(1, 4, TargetReg::RegA)),
			0x3F => Some(Instruction::CCF(1, 4)),
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
			0xE6 => Some(Instruction::ANDs(2, 8, ArithmeticOperand::ByteFromPC)),
			0xE8 => Some(Instruction::ADDSPe(2, 16)),
			0xEE => Some(Instruction::XORs(2, 8, ArithmeticOperand::ByteFromPC)),
			0xF6 => Some(Instruction::ORs(2, 8, ArithmeticOperand::ByteFromPC)),
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
	pub fn get_target_reg_value(&self, target: TargetReg) -> u8 {
		match target {
			TargetReg::RegA => self.registers.a,
			TargetReg::RegB => self.registers.b,
			TargetReg::RegC => self.registers.c,
			TargetReg::RegD => self.registers.d,
			TargetReg::RegE => self.registers.e,
			TargetReg::RegH => self.registers.h,
			TargetReg::RegL => self.registers.l,
			TargetReg::HLPointee => self.registers.get_hl_pointee(&self.memory_bus),
		}
	}
	pub fn set_target_reg_value(&mut self, target: TargetReg, data: u8) {
		match target {
			TargetReg::RegA => {self.registers.a = data}
			TargetReg::RegB => {self.registers.b = data}
			TargetReg::RegC => {self.registers.c = data}
			TargetReg::RegD => {self.registers.d = data}
			TargetReg::RegE => {self.registers.e = data}
			TargetReg::RegH => {self.registers.h = data}
			TargetReg::RegL => {self.registers.l = data}
			TargetReg::HLPointee => {self.registers.set_hl_pointee(&mut self.memory_bus, data)}
		}
	}
	pub fn set_target_pair_value(&mut self, operand: TargetRegPair, data: u16) {
		match operand {
			TargetRegPair::RegsBC => {self.registers.set_bc(data)}
			TargetRegPair::RegsDE => {self.registers.set_de(data)}
			TargetRegPair::RegsHL => {self.registers.set_hl(data)}
			TargetRegPair::RegSP => {self.registers.stack_pointer = data}
		}
	}
	pub fn get_target_pair_value(&self, operand: TargetRegPair) -> u16 {
		match operand {
			TargetRegPair::RegsBC => self.registers.get_bc(),
			TargetRegPair::RegsDE => self.registers.get_de(),
			TargetRegPair::RegsHL => self.registers.get_hl(),
			TargetRegPair::RegSP => self.registers.stack_pointer
		}
	}
	pub fn execute_op(&mut self, instruction: Instruction) {
		match instruction {
			Instruction::NOP(_, _) => {}
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
				let target_value = self.get_target_reg_value(_target);
				self.registers.f.zero = target_value == 0xFF;
				self.registers.f.substract = false;
				self.registers.f.half_carry = (target_value & 0xF) + 1 >= 0x10;
				self.set_target_reg_value(_target, target_value.overflowing_add(1).0);
			}
			Instruction::DECs(_, _, _target) => {
				let target_value = self.get_target_reg_value(_target);
				self.registers.f.zero = target_value == 0x01;
				self.registers.f.substract = true;
				self.registers.f.half_carry = target_value & 0xF < 1;
				self.set_target_reg_value(_target, target_value.overflowing_sub(1).0);
			}
			Instruction::INCss(_, _, _target) => {
				self.set_target_pair_value(_target, self.get_target_pair_value(_target).overflowing_add(1).0);
			}
			Instruction::DECss(_, _, _target) => {
				self.set_target_pair_value(_target, self.get_target_pair_value(_target).overflowing_sub(1).0);
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