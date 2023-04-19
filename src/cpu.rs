pub mod registers;
pub mod instructions;

use std::fmt::Debug;
use registers::Registers;
use crate::memory_bus::MemoryBus;
use instructions::Instruction;

#[derive(Debug, Clone, Copy)]
pub enum CpuState{
	Running, Halted, Stopped
}

//#[derive(Debug)]
pub struct Cpu {
	pub registers: Registers,
	pub current_op: Option<Instruction>,
	pub next_op: Option<Instruction>,
	ime_scheduled: bool,
	ime_set: bool,
	state: CpuState
}

impl Cpu {
	pub fn new() -> Self {
		let cpu = Cpu {
			registers: Registers::new(),
    		current_op: None,
			next_op: Some(Instruction::NOP(1, 1)),				// Fake 'execute' of first tick which is just a 'fetch' 
			ime_scheduled: false,
			ime_set: false,
			state: CpuState::Running
		};
		// cpu.registers.init();
		cpu
	}
	pub fn tick(&mut self, memory_bus: &mut MemoryBus) {
		self.current_op = self.next_op;								// Account for Sharp SM83 fetch/execute overlap
		self.exec_current_op(memory_bus);
		self.fetch_next_opcode(memory_bus);							// Account for Sharp SM83 fetch/execute overlap
	}
	fn fetch_next_opcode(&mut self, memory_bus: &MemoryBus) {
		self.next_op = Instruction::from_opcode(self.fetch_pc(memory_bus), self, memory_bus);
	}
	fn exec_current_op(&mut self, memory_bus: &mut MemoryBus) {
		self.execute_op(memory_bus,
			if let Some(instruction) = self.current_op {
				instruction
			} else {
				panic!("Unknown opcode 0x{:x} at location 0x{:x}!", memory_bus.read_byte(self.registers.program_counter - 1), self.registers.program_counter - 1);
			}
		)
	}
	fn fetch_pc(&mut self, memory_bus: &MemoryBus) -> u8 {
		let data = memory_bus.read_byte(self.registers.program_counter);
		self.registers.program_counter = self.registers.program_counter.overflowing_add(1).0;
		data
	}
}

impl Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cpu").field("registers", &self.registers).field("current_op", &self.current_op).finish()
    }
}