mod cpu;
mod ppu;

use crate::cpu::Cpu;
use crate::ppu::Ppu;
use std::{thread, time};

fn main() {
	let mut my_cpu = Cpu::new();
	let mut my_ppu = Ppu::new(&mut my_cpu.memory_bus);
	my_cpu.tick();                                          // "Virtual" tick to fetch; no operation is executed
	my_ppu.update();
	println!("CPU state {:x?}", my_cpu);
	for _ in 0..3*0x1FFE {
		my_cpu.tick();
		my_ppu.update();
		println!("CPU state {:x?}", my_cpu);
		//thread::sleep(time::Duration::from_millis(1));
	}
	loop {
		my_cpu.tick();
		my_ppu.update();
		println!("CPU state {:x?}", my_cpu);
		thread::sleep(time::Duration::from_millis(500));
	}
}
