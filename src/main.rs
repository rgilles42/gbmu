mod memory_bus;
mod cpu;
mod ppu;
mod timer;

use cpu::Cpu;
use memory_bus::MemoryBus;
use ppu::Ppu;
use timer::Timer;

fn main() {
	let mut nb_ticks = 0;

	let mut debug_enabled = false;
	let mut memory_bus = MemoryBus::new(Some("/home/rgilles/Desktop/roms/Tetris_Rev01.gb"));
	memory_bus.load_dmg_bootrom();
	let mut cpu = Cpu::new();
	let mut ppu = Ppu::new(false, false);
	let mut timer = Timer::new();
	cpu.tick(&mut memory_bus);									// "Virtual" tick to realise first PC pointee byte fetch; no operation is executed
	while cpu.registers.program_counter - 1 != 0xFFFF {			// When the op at PC is about to be executed, PC is now PC+1
	// while nb_ticks <= 23579000 {
		if nb_ticks == u64::MAX {	
			debug_enabled = true;
		}
		if debug_enabled {
			println!("{:x?}", cpu);
			println!("Tick count: {}", nb_ticks);
			std::thread::sleep(std::time::Duration::from_millis(50))
		}
		let nb_cycles = cpu.tick(&mut memory_bus);
		for _ in 0..nb_cycles {
			ppu.tick(&mut memory_bus);
			timer.tick(&mut memory_bus);
		}
		nb_ticks += nb_cycles as u64;
	}
}
