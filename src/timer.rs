use crate::memory_bus::{MemoryBus, timer_memory::CPUFreqDivider};

pub struct Timer {
	nb_ticks_div: usize,
	nb_ticks_tima: usize
}

impl Timer {
	pub fn new() -> Self {
		Timer {
			nb_ticks_div: 0,
			nb_ticks_tima: 0,
		}
	}
	pub fn tick(&mut self, memory_bus: &mut MemoryBus) {
		if memory_bus.timer_memory.timer_enabled {
			self.nb_ticks_tima = if self.nb_ticks_tima ==
									match memory_bus.timer_memory.timer_freq_div {
										CPUFreqDivider::Ratio0x10 => 0x0F,
										CPUFreqDivider::Ratio0x40 => 0x3F,
										CPUFreqDivider::Ratio0x100 => 0xFF,
										CPUFreqDivider::Ratio0x400 => 0x3FF,
									}{
				memory_bus.timer_memory.increase_tima();
				0x00
			} else {self.nb_ticks_tima + 1};
		}
		self.nb_ticks_div = if self.nb_ticks_div == 0xFF {
			memory_bus.timer_memory.increase_div();
			0x00
		} else {self.nb_ticks_div + 1};
	}
}

#[cfg(test)]
mod tests {
    use crate::memory_bus::MemoryBus;
    use super::Timer;

	#[test]
	fn test_timer() {
		let mut memory_bus = MemoryBus::new(None);
		let mut timer = Timer::new();
		println!("DIV is {:x}, TIMA is {:x}", memory_bus.timer_memory.read(0xFF04), memory_bus.timer_memory.read(0xFF05));
		for _ in 0..0x100 {
			timer.tick(&mut memory_bus);
			println!("DIV is {:x}, TIMA is {:x}", memory_bus.timer_memory.read(0xFF04), memory_bus.timer_memory.read(0xFF05));
		}
		assert_eq!(memory_bus.timer_memory.read(0xFF04), 0x01);
		assert_eq!(memory_bus.timer_memory.read(0xFF05), 0x00);
		memory_bus.timer_memory.write(0xFF07, 0x04);
		for _ in 0..0x400 {
			timer.tick(&mut memory_bus);
			println!("DIV is {:x}, TIMA is {:x}", memory_bus.timer_memory.read(0xFF04), memory_bus.timer_memory.read(0xFF05));
		}
		assert_eq!(memory_bus.timer_memory.read(0xFF04), 0x05);
		assert_eq!(memory_bus.timer_memory.read(0xFF05), 0x01);
		memory_bus.timer_memory.write(0xFF07, 0x05);
		for _ in 0..0x100 {
			timer.tick(&mut memory_bus);
			println!("DIV is {:x}, TIMA is {:x}", memory_bus.timer_memory.read(0xFF04), memory_bus.timer_memory.read(0xFF05));
		}
		assert_eq!(memory_bus.timer_memory.read(0xFF04), 0x06);
		assert_eq!(memory_bus.timer_memory.read(0xFF05), 0x11);
	}
}