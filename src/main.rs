mod memory_bus;
mod cpu;
mod ppu;

use cpu::Cpu;
use memory_bus::MemoryBus;
use ppu::Ppu;
use std::{thread, time, cell::RefCell};

fn main() {
	let memory_bus = RefCell::new(MemoryBus::new());
	//memory_bus.borrow_mut().init();
	memory_bus.borrow_mut().load_dmg_bootrom();
	let mut my_cpu = Cpu::new(memory_bus.borrow_mut());
	let mut _my_ppu = Ppu::new(memory_bus.borrow_mut());
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
