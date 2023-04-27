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
	pub fn tick(&mut self, memory_bus: &mut MemoryBus) -> u8 {
		self.current_op = self.next_op;								// Account for Sharp SM83 fetch/execute overlap
		if self.ime_scheduled {
			self.ime_set = true;
			self.ime_scheduled = false;
		}
		self.exec_current_op(memory_bus);
		self.fetch_next_opcode(memory_bus);							// Account for Sharp SM83 fetch/execute overlap
		self.get_nb_clock_current_op()
	}
	fn fetch_next_opcode(&mut self, memory_bus: &MemoryBus) {
		if self.ime_set && memory_bus.read_byte(0xFFFF) & memory_bus.read_byte(0xFF0F) != 0 
		{
			self.next_op = Some(Instruction::ISR(0, 20));
		} else {
			self.next_op = Instruction::from_opcode(self.fetch_pc(memory_bus), self, memory_bus);
		}
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
	fn get_nb_clock_current_op(&mut self) -> u8 {
		match self.current_op.unwrap() {
			Instruction::LD(_, nb_cycles, _, _) => nb_cycles,
			Instruction::LDI(_, nb_cycles, _, _) => nb_cycles,
			Instruction::LDD(_, nb_cycles, _, _) => nb_cycles,
			Instruction::LD16(_, nb_cycles, _, _) => nb_cycles,
			Instruction::PUSH(_, nb_cycles, _) => nb_cycles,
			Instruction::POP(_, nb_cycles, _) => nb_cycles,
			Instruction::ADDAs(_, nb_cycles, _) => nb_cycles,
			Instruction::ADCAs(_, nb_cycles, _) => nb_cycles,
			Instruction::SUBs(_, nb_cycles, _) => nb_cycles,
			Instruction::SBCAs(_, nb_cycles, _) => nb_cycles,
			Instruction::ANDs(_, nb_cycles, _) => nb_cycles,
			Instruction::XORs(_, nb_cycles, _) => nb_cycles,
			Instruction::ORs(_, nb_cycles, _) => nb_cycles,
			Instruction::CPs(_, nb_cycles, _) => nb_cycles,
			Instruction::INCs(_, nb_cycles, _) => nb_cycles,
			Instruction::DECs(_, nb_cycles, _) => nb_cycles,
			Instruction::DAA(_, nb_cycles) => nb_cycles,
			Instruction::CPL(_, nb_cycles) => nb_cycles,
			Instruction::ADDHLss(_, nb_cycles, _) => nb_cycles,
			Instruction::INCss(_, nb_cycles, _) => nb_cycles,
			Instruction::DECss(_, nb_cycles, _) => nb_cycles,
			Instruction::ADDSPe(_, nb_cycles) => nb_cycles,
			Instruction::LDHLSPe(_, nb_cycles) => nb_cycles,
			Instruction::RLCA(_, nb_cycles) => nb_cycles,
			Instruction::RLA(_, nb_cycles) => nb_cycles,
			Instruction::RRCA(_, nb_cycles) => nb_cycles,
			Instruction::RRA(_, nb_cycles) => nb_cycles,
			Instruction::RLC(_, nb_cycles, _) => nb_cycles,
			Instruction::RL(_, nb_cycles, _) => nb_cycles,
			Instruction::RRC(_, nb_cycles, _) => nb_cycles,
			Instruction::RR(_, nb_cycles, _) => nb_cycles,
			Instruction::SLA(_, nb_cycles, _) => nb_cycles,
			Instruction::SWAP(_, nb_cycles, _) => nb_cycles,
			Instruction::SRA(_, nb_cycles, _) => nb_cycles,
			Instruction::SRL(_, nb_cycles, _) => nb_cycles,
			Instruction::BIT(_, nb_cycles, _, _) => nb_cycles,
			Instruction::SET(_, nb_cycles, _, _) => nb_cycles,
			Instruction::RES(_, nb_cycles, _, _) => nb_cycles,
			Instruction::CCF(_, nb_cycles) => nb_cycles,
			Instruction::SCF(_, nb_cycles) => nb_cycles,
			Instruction::NOP(_, nb_cycles) => nb_cycles,
			Instruction::HALT(_, nb_cycles) => nb_cycles,
			Instruction::STOP(_, nb_cycles) => nb_cycles,
			Instruction::DI(_, nb_cycles) => nb_cycles,
			Instruction::EI(_, nb_cycles) => nb_cycles,
			Instruction::JPnn(_, nb_cycles) => nb_cycles,
			Instruction::JPHL(_, nb_cycles) => nb_cycles,
			Instruction::JPfnn(_, nb_cycles, _) => nb_cycles,
			Instruction::JR(_, nb_cycles) => nb_cycles,
			Instruction::JRf(_, nb_cycles, _) => nb_cycles,
			Instruction::CALL(_, nb_cycles) => nb_cycles,
			Instruction::CALLf(_, nb_cycles, _) => nb_cycles,
			Instruction::ISR(_, nb_cycles) => nb_cycles,
			Instruction::RET(_, nb_cycles) => nb_cycles,
			Instruction::RETf(_, nb_cycles, _) => nb_cycles,
			Instruction::RETI(_, nb_cycles) => nb_cycles,
			Instruction::RST(_, nb_cycles, _) => nb_cycles
		}
	}
}

impl Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cpu").field("regs", &self.registers).field("op", &self.next_op).finish()
    }
}