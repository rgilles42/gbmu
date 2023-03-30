mod cpu;

use crate::cpu::Cpu;

fn main() {
    let mut my_cpu = Cpu::new();
    println!("CPU state {:#?}", my_cpu);
    my_cpu.tick();
    println!("CPU state {:#?}", my_cpu);
}
