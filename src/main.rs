mod cpu;

use crate::cpu::Cpu;
use std::{thread, time};

fn main() {
    let mut my_cpu = Cpu::new();
    my_cpu.tick();                          // "Virtual" tick to fetch; no operation is executed
    println!("CPU state {:x?}", my_cpu);
    for _ in 0..0x1FFE {
        my_cpu.tick();
        println!("CPU state {:x?}", my_cpu);
        //thread::sleep(time::Duration::from_millis(100));
        my_cpu.tick();
        println!("CPU state {:x?}", my_cpu);
        //thread::sleep(time::Duration::from_millis(100));
        my_cpu.tick();
        println!("CPU state {:x?}", my_cpu);
        //thread::sleep(time::Duration::from_millis(100));
    }
    loop {
        my_cpu.tick();
        println!("CPU state {:x?}", my_cpu);
        thread::sleep(time::Duration::from_millis(50));
    }
}
