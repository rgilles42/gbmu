pub mod registers;
pub mod instructions;
pub mod memory_bus;
use registers::Registers;
use memory_bus::MemoryBus;
use instructions::Instruction;

#[derive(Debug)]
pub struct Cpu {
	registers: Registers,
	memory_bus: MemoryBus,
	current_op: Option<Instruction>,
	next_op: Option<Instruction>
}

impl Cpu {
	pub fn new() -> Self {
		let mut cpu = Cpu {
			registers: Registers::new(),
    		memory_bus: MemoryBus::new(),
    		current_op: Some(Instruction::NOP(1, 1)),				// Fake 'execute' of first tick which is just a 'fetch' 
			next_op: None
		};
		cpu.registers.init();
		cpu.memory_bus.init();
		cpu
	}
	pub fn tick(&mut self) {
		self.exec_current_op();
		self.fetch_next_opcode();									// Account for Sharp SM83 fetch/execute overlap
		self.current_op = self.next_op;
	}
	fn fetch_next_opcode(&mut self) {
		self.next_op = Instruction::from_opcode(self.fetch_pc(), self);
	}
	fn exec_current_op(&mut self) {
		self.execute_op(
			if let Some(instruction) = self.current_op {
				instruction
			} else {
				panic!("Unknown opcode 0x{:x} at location 0x{:x}!", self.memory_bus.read_byte(self.registers.program_counter - 1), self.registers.program_counter - 1);
			}
		)
	}
	fn fetch_pc(&mut self) -> u8 {
		let data = self.memory_bus.read_byte(self.registers.program_counter);
		self.registers.program_counter = self.registers.program_counter.overflowing_add(1).0;
		data
	}
}