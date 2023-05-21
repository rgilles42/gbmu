#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct InputState {
	pub is_up_pressed: bool,
	pub is_down_pressed: bool,
	pub is_left_pressed: bool,
	pub is_right_pressed: bool,
	pub is_a_pressed: bool,
	pub is_b_pressed: bool,
	pub is_start_pressed: bool,
	pub is_select_pressed: bool,
}

#[derive(Debug)]
pub struct InputMemory {
	joypad_state: InputState,

	is_reading_action_buttons: bool,
	is_reading_direction_buttons: bool,
}

impl InputMemory {
	pub fn new() -> Self {
		InputMemory {
			joypad_state: InputState {
				is_up_pressed: false,
				is_down_pressed: false,
				is_left_pressed: false,
				is_right_pressed: false,
				is_a_pressed: false,
				is_b_pressed: false,
				is_start_pressed: false,
				is_select_pressed: false
			},
			is_reading_action_buttons: false,
			is_reading_direction_buttons: false,
		}
	}
	pub fn write(&mut self, data: u8) {
		if data & (1 << 5) == 0 {
			self.is_reading_action_buttons = true
		} else {
			self.is_reading_action_buttons = false
		}
		if data & (1 << 4) == 0 {
			self.is_reading_direction_buttons = true
		} else {
			self.is_reading_direction_buttons = false
		}
	}
	pub fn read(&self) -> u8 {
		let mut res = (self.is_reading_action_buttons as u8) << 5 | (self.is_reading_direction_buttons as u8) << 4;
		if self.is_reading_action_buttons && !self.is_reading_direction_buttons {
			res |= (!self.joypad_state.is_start_pressed as u8)	<< 3
				| (!self.joypad_state.is_select_pressed as u8)	<< 2
				| (!self.joypad_state.is_b_pressed as u8)		<< 1
				| (!self.joypad_state.is_a_pressed as u8);

		} else if self.is_reading_direction_buttons && !self.is_reading_action_buttons {
			res |= (!self.joypad_state.is_down_pressed as u8)	<< 3
				| (!self.joypad_state.is_up_pressed as u8)		<< 2
				| (!self.joypad_state.is_left_pressed as u8)	<< 1
				| (!self.joypad_state.is_right_pressed as u8);
		} else {
			res |= 0x0F
		}
		res
	}
	pub fn update(&mut self, input_state: &InputState) {
		self.joypad_state = *input_state;
	}
}