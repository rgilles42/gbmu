pub mod registers;
pub mod instructions;

use std::{fmt::Debug, cell::RefMut};
use registers::Registers;
use crate::memory_bus::MemoryBus;
use instructions::Instruction;

#[derive(Debug, Clone, Copy)]
pub enum CpuState{
	Running, Halted, Stopped
}

//#[derive(Debug)]
pub struct Cpu<'a> {
	registers: Registers,
	pub memory_bus: RefMut<'a, MemoryBus>,
	pub current_op: Option<Instruction>,
	pub next_op: Option<Instruction>,
	ime_scheduled: bool,
	ime_set: bool,
	state: CpuState
}

impl Cpu<'_> {
	pub fn new(memory_bus: RefMut<'_, MemoryBus>) -> Cpu<'_> {
		let cpu = Cpu {
			registers: Registers::new(),
    		memory_bus,
    		current_op: None,
			next_op: Some(Instruction::NOP(1, 1)),				// Fake 'execute' of first tick which is just a 'fetch' 
			ime_scheduled: false,
			ime_set: false,
			state: CpuState::Running
		};
		// cpu.registers.init();
		cpu
	}
	pub fn tick(&mut self) {
		self.current_op = self.next_op;								// Account for Sharp SM83 fetch/execute overlap
		self.exec_current_op();
		self.fetch_next_opcode();									// Account for Sharp SM83 fetch/execute overlap
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

impl Debug for Cpu<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cpu").field("registers", &self.registers).field("current_op", &self.current_op).finish()
    }
}