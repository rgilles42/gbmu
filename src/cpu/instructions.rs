use super::Cpu;

#[derive(Debug, Clone, Copy)]
pub enum ArithmeticTarget{
	RegA, RegB, RegC, RegD, RegE, RegH, RegL, HLPointee, RawByte
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
	ADD(ArithmeticTarget)
}

impl Instruction {
	pub fn from_opcode(opcode: u8) -> Option<Instruction> {
		match opcode {
			0x80 => Some(Instruction::ADD(ArithmeticTarget::RegB)),
			0x81 => Some(Instruction::ADD(ArithmeticTarget::RegC)),
			0x82 => Some(Instruction::ADD(ArithmeticTarget::RegD)),
			0x83 => Some(Instruction::ADD(ArithmeticTarget::RegE)),
			0x84 => Some(Instruction::ADD(ArithmeticTarget::RegH)),
			0x85 => Some(Instruction::ADD(ArithmeticTarget::RegL)),
			0x86 => Some(Instruction::ADD(ArithmeticTarget::HLPointee)),
			0x87 => Some(Instruction::ADD(ArithmeticTarget::RegA)),
			0xC6 => Some(Instruction::ADD(ArithmeticTarget::RawByte)),
			_ => None
		}
	}
}

impl Cpu {
	pub fn execute_op(&mut self, instruction: Instruction) {
		match instruction {
			Instruction::ADD(target) => {
				let x = self.registers.a	as u16;
				let y = match target {
					ArithmeticTarget::RegA => self.registers.a,
					ArithmeticTarget::RegB => self.registers.b,
					ArithmeticTarget::RegC => self.registers.c,
					ArithmeticTarget::RegD => self.registers.d,
					ArithmeticTarget::RegE => self.registers.e,
					ArithmeticTarget::RegH => self.registers.h,
					ArithmeticTarget::RegL => self.registers.l,
					ArithmeticTarget::HLPointee => self.registers.get_hl_pointee(&self.memory_bus),
					ArithmeticTarget::RawByte => self.fetch_pc(),
				}								as u16;
				// if carry let c = 1;
				let r = x + y; // + c
				self.registers.a = r as u8;
				self.registers.f.zero = self.registers.a == 0;
				self.registers.f.substract = false;
				self.registers.f.half_carry = (x & 0xF) + (y & 0xF) /* + c */ >= 0x10;
				self.registers.f.carry = r >= 0x100;
			}
		}
	}
}