use super::Cpu;

type InstrLength = u8;
type InstrCycles = u8;

#[derive(Debug, Clone, Copy)]
pub enum ArithmeticTarget{
	RegA, RegB, RegC, RegD, RegE, RegH, RegL, HLPointee, RawByte
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
	ADDAn(InstrLength, InstrCycles, ArithmeticTarget),
	ADCAn(InstrLength, InstrCycles, ArithmeticTarget)
}

impl Instruction {
	pub fn from_opcode(opcode: u8) -> Option<Instruction> {
		match opcode {
			0x80 => Some(Instruction::ADDAn(1, 4, ArithmeticTarget::RegB)),
			0x81 => Some(Instruction::ADDAn(1, 4, ArithmeticTarget::RegC)),
			0x82 => Some(Instruction::ADDAn(1, 4, ArithmeticTarget::RegD)),
			0x83 => Some(Instruction::ADDAn(1, 4, ArithmeticTarget::RegE)),
			0x84 => Some(Instruction::ADDAn(1, 4, ArithmeticTarget::RegH)),
			0x85 => Some(Instruction::ADDAn(1, 4, ArithmeticTarget::RegL)),
			0x86 => Some(Instruction::ADDAn(1, 8, ArithmeticTarget::HLPointee)),
			0x87 => Some(Instruction::ADDAn(1, 4, ArithmeticTarget::RegA)),
			0x88 => Some(Instruction::ADCAn(1, 4, ArithmeticTarget::RegB)),
			0x89 => Some(Instruction::ADCAn(1, 4, ArithmeticTarget::RegC)),
			0x8A => Some(Instruction::ADCAn(1, 4, ArithmeticTarget::RegD)),
			0x8B => Some(Instruction::ADCAn(1, 4, ArithmeticTarget::RegE)),
			0x8C => Some(Instruction::ADCAn(1, 4, ArithmeticTarget::RegH)),
			0x8D => Some(Instruction::ADCAn(1, 4, ArithmeticTarget::RegL)),
			0x8E => Some(Instruction::ADCAn(1, 8, ArithmeticTarget::HLPointee)),
			0x8F => Some(Instruction::ADCAn(1, 4, ArithmeticTarget::RegA)),

			0xC6 => Some(Instruction::ADDAn(2, 8, ArithmeticTarget::RawByte)),
			0xCE => Some(Instruction::ADCAn(2, 8, ArithmeticTarget::RawByte)),
			_ => None
		}
	}
}

impl Cpu {
	pub fn execute_op(&mut self, instruction: Instruction) {
		match instruction {
			Instruction::ADDAn(_, _, target) | Instruction::ADCAn(_, _, target) => {
				let reg_a_content = self.registers.a	as u16;
				let target_content = match target {
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
				// if instr is ADC and carry flag is true, let carry = 1;
				let carry = if let Instruction::ADCAn(_, _, _) = instruction {self.registers.f.carry as u16} else {0};
				let r = reg_a_content + target_content + carry;
				self.registers.f.zero = r == 0;
				self.registers.f.substract = false;
				self.registers.f.half_carry = (reg_a_content & 0xF) + (target_content & 0xF) + carry >= 0x10;
				self.registers.f.carry = r >= 0x100;
				self.registers.a = r as u8;
			}
		}
	}
}