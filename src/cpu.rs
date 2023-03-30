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
	current_op: Option<Instruction>
}

impl Cpu {
	pub fn new() -> Self {
		let mut cpu = Cpu {
			registers: Registers {a: 0, b: 0, c: 0, d: 0, e: 0, f: 0.into(), h: 0, l: 0, program_counter: 0x0000, stack_pointer : 0x0000},
    		memory_bus: MemoryBus{memory: [0x80; 0xFFFF]},
    		current_op: None,
		};
		cpu.registers.program_counter = 0x100;
		cpu.registers.set_af(0x01B0);
		cpu.registers.set_bc(0x0013);
		cpu.registers.set_de(0x00D8);
		cpu.registers.set_hl(0x014D);
		cpu.registers.stack_pointer = 0xFFFE;
		cpu
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
				panic!("Unknown opcode 0x{:x}!", self.memory_bus.read_byte(self.registers.program_counter - 1));
			}
		)
	}
	fn fetch_pc(&mut self) -> u8 {
		let data = self.memory_bus.read_byte(self.registers.program_counter);
		self.registers.program_counter += 1;
		data
	}
}