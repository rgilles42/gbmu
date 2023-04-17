mod memory_bus;
mod cpu;
mod ppu;

use cpu::Cpu;
use memory_bus::MemoryBus;
use ppu::Ppu;
use std::{thread, time};

fn main() {
	let mut memory_bus = MemoryBus::new();
	let mut my_cpu = Cpu::new(&mut memory_bus);
	let mut _my_ppu = Ppu::new(my_cpu.memory_bus);
	my_cpu.tick();                                          // "Virtual" tick to realise first PC pointee byte fetch; no operation is executed
	//my_ppu.update();
	println!("CPU state {:x?}", my_cpu);
	for _ in 0..3*0x1FFE {
		my_cpu.tick();
		// my_ppu.update();
		//println!("CPU state {:x?}", my_cpu);
	}
	loop {
		my_cpu.tick();
		// my_ppu.update();
		println!("CPU state {:x?}", my_cpu);
		thread::sleep(time::Duration::from_millis(500));
	}
}
