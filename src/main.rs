mod memory_bus;
mod cpu;
mod ppu;

use cpu::Cpu;
use memory_bus::MemoryBus;
use ppu::Ppu;
use std::{thread, time};

fn main() {
	let mut memory_bus = MemoryBus::new();
	//memory_bus.init();
	memory_bus.load_dmg_bootrom();
	memory_bus.debug_insert_cart_logo();
	let mut cpu = Cpu::new();
	let mut ppu = Ppu::new();
	std::thread::sleep(std::time::Duration::from_millis(500));		// or first minifb update will fail??
	cpu.tick(&mut memory_bus);                                          // "Virtual" tick to realise first PC pointee byte fetch; no operation is executed
	ppu.update(&mut memory_bus);
	println!("CPU state {:x?}", cpu);
	for _ in 0..3*0x1FFE {
		cpu.tick(&mut memory_bus);
		//println!("CPU state {:x?}", cpu);
	}
//	thread::sleep(time::Duration::from_millis(5000));
	while cpu.registers.program_counter - 1 != 0x55 {
		cpu.tick(&mut memory_bus);
		// ppu.update(&mut memory_bus);
		// println!("CPU state {:x?}", cpu);
		// thread::sleep(time::Duration::from_millis(100));
	}
	loop {
		ppu.update(&mut memory_bus);
	}
}
