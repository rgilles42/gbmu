mod memory_bus;
mod cpu;
mod ppu;
mod timer;

use cpu::Cpu;
use memory_bus::MemoryBus;
use ppu::Ppu;
use timer::Timer;

fn main() {
	let mut memory_bus = MemoryBus::new(Some("/home/rgilles/Desktop/roms/Tetris_Rev01.gb"));
	memory_bus.load_dmg_bootrom();
	//memory_bus.cartridge.debug_insert_cart_logo();
	let mut cpu = Cpu::new();
	let mut ppu = Ppu::new(false, false);
	let mut timer = Timer::new();
	cpu.tick(&mut memory_bus);									// "Virtual" tick to realise first PC pointee byte fetch; no operation is executed
	while cpu.registers.program_counter - 1 != 0x2F2 {			// When the op at PC is about to be executed, PC is now PC+1
		let nb_cycles = cpu.tick(&mut memory_bus);
		for _ in 0..nb_cycles {
			ppu.tick(&mut memory_bus);
			timer.tick(&mut memory_bus);
		}
	}
	loop {
		println!("{:x?}", cpu);
		let nb_cycles = cpu.tick(&mut memory_bus);
		for _ in 0..nb_cycles {
			ppu.tick(&mut memory_bus);
			timer.tick(&mut memory_bus);
		}
		if cpu.registers.program_counter - 1 == 0x2F2 {
			break
			// loop {
			// 	ppu.tick(&mut memory_bus);
			// }
		}
	}
	std::thread::sleep(std::time::Duration::from_millis(3000));
}
