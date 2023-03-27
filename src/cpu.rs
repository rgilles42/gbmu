pub struct cpu {
	registers: Registers,
	program_counter: u16,
	stack_pointer: u16,
	memory_bus: [u8; 0xFFFF]
}

impl cpu {
	fn execute(&mut self, instruction: Instruction) {
		match instruction {
			Instruction::ADD(target) => {
				match target {
					ArithmeticTarget::C => {
						//TODO: implement add on reg C
					}
					_ => { /* TODO: more targets */}
				}
			}
			_ => { /* TODO: more instr */}
		}
	}
}