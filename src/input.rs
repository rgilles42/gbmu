use winit_input_helper::WinitInputHelper;
use crate::memory_bus::{MemoryBus, input_memory::InputState};

pub fn tick(memory_bus: &mut MemoryBus, input: &WinitInputHelper) {
	let input_state = InputState {
		is_up_pressed: input.key_held(winit::event::VirtualKeyCode::W),
		is_down_pressed: input.key_held(winit::event::VirtualKeyCode::S),
		is_left_pressed: input.key_held(winit::event::VirtualKeyCode::A),
		is_right_pressed: input.key_held(winit::event::VirtualKeyCode::D),
		is_a_pressed: input.key_held(winit::event::VirtualKeyCode::Backslash),
		is_b_pressed: input.key_held(winit::event::VirtualKeyCode::Return),
		is_start_pressed: input.key_held(winit::event::VirtualKeyCode::LShift),
		is_select_pressed: input.key_held(winit::event::VirtualKeyCode::Space),
	};
	memory_bus.input_memory.update(&input_state)
}