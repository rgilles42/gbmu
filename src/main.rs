mod memory_bus;
mod cpu;
mod ppu;

use cpu::Cpu;
use memory_bus::MemoryBus;
use ppu::Ppu;

fn main() {
	let mut memory_bus = MemoryBus::new();
	memory_bus.load_dmg_bootrom();
	memory_bus.debug_insert_cart_logo();
	let mut cpu = Cpu::new();
	let mut ppu = Ppu::new(false, false);
	cpu.tick(&mut memory_bus);									// "Virtual" tick to realise first PC pointee byte fetch; no operation is executed
	while cpu.registers.program_counter - 1 != 0xFE {			// When the op at PC is about to be executed, PC is now PC+1
		let nb_cycles = cpu.tick(&mut memory_bus);
		for _ in 0..nb_cycles {
			ppu.tick(&mut memory_bus);
		}
	}
	println!("{:x?}", cpu);
}
