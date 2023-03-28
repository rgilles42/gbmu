pub mod registers;
pub mod instructions;
pub mod memory_bus;
use registers::Registers;
use memory_bus::MemoryBus;
use instructions::Instruction;

pub struct Cpu {
	registers: Registers,
	memory_bus: MemoryBus,
	current_op: Option<Instruction>
}

impl Cpu {
	pub fn new() -> Self {
		Cpu {
			registers: Registers {a: 0, b: 0, c: 0, d: 0, e: 0, f: 0.into(), h: 0, l: 0, program_counter: 0x0000},
    		memory_bus: MemoryBus{memory: [0; 0xFFFF]},
    		current_op: None,
		}
	}
	pub fn tick(&mut self) {
		self.fetch_opcode();
		self.exec();
	}
	fn fetch_opcode(&mut self) {
		self.current_op = Instruction::from_opcode(self.fetch_pc());
	}
	fn exec(&mut self) {
		self.execute_op(
			if let Some(instruction) = self.current_op {
				instruction
			} else {
				panic!("Unknown opcode 0x{:x}.", self.memory_bus.read_byte(self.registers.program_counter - 1));
			}
		)
	}
	fn fetch_pc(&mut self) -> u8 {
		let v = self.memory_bus.read_byte(self.registers.program_counter);
		self.registers.program_counter += 1;
		v
	}
}