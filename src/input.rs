use minifb::Key;
use crate::{memory_bus::{MemoryBus, input_memory::InputState}, ppu::Ppu};

pub fn input_tick(memory_bus: &mut MemoryBus, ppu: &Ppu) {
	let keys = ppu.viewport.get_keys();
	let input_state = InputState {
		is_up_pressed: keys.contains(&Key::W),
		is_down_pressed: keys.contains(&Key::S),
		is_left_pressed: keys.contains(&Key::A),
		is_right_pressed: keys.contains(&Key::D),
		is_a_pressed: keys.contains(&Key::Backslash),
		is_b_pressed: keys.contains(&Key::Enter),
		is_start_pressed: keys.contains(&Key::Space),
		is_select_pressed: keys.contains(&Key::LeftShift),
	};
	memory_bus.input_memory.update(&input_state)
}

#[cfg(test)]
mod test {
    use crate::{ppu::Ppu, memory_bus::MemoryBus};

    use super::input_tick;

	#[test]
	fn test_input() {
		let mut memory_bus = MemoryBus::new(None);
		let mut ppu = Ppu::new(false, false);
		memory_bus.ppu_memory.lcd_enable = true;
		memory_bus.write_byte(0xFF00, 0xF0);
		loop {
			ppu.tick(&mut memory_bus);
			input_tick(&mut memory_bus, &ppu);
			println!("{:x?}", memory_bus.input_memory);
			//std::thread::sleep(std::time::Duration::from_millis(1000))

		}
	}
}