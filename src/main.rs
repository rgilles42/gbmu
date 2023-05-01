mod memory_bus;
mod cpu;
mod ppu;
mod timer;

use cpu::Cpu;
use memory_bus::MemoryBus;
use ppu::Ppu;
use timer::Timer;

fn main() {
	let mut debug_enabled = false;
	let mut memory_bus = MemoryBus::new(Some("/home/rgilles/Desktop/roms/Tetris_Rev01.gb"));
	memory_bus.load_dmg_bootrom();
	let mut cpu = Cpu::new();
	let mut ppu = Ppu::new(false, false);
	let mut timer = Timer::new();
	cpu.tick(&mut memory_bus);									// "Virtual" tick to realise first PC pointee byte fetch; no operation is executed
	while cpu.registers.program_counter - 1 != 0xFFFF {			// When the op at PC is about to be executed, PC is now PC+1
		if cpu.registers.program_counter - 1 == 0xFFFF && cpu.registers.a == 0{	
			debug_enabled = true
		}
		if debug_enabled {
			println!("{:x?}", cpu);
		}
		let nb_cycles = cpu.tick(&mut memory_bus);
		for _ in 0..nb_cycles {
			ppu.tick(&mut memory_bus);
			timer.tick(&mut memory_bus);
		}
	}
}
