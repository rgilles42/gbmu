mod cpu;

use crate::cpu::Cpu;

fn main() {
    let mut my_cpu = Cpu::new();
    my_cpu.tick();
}
