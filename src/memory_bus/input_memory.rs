#[derive(Debug)]
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
	joyp_reg: u8,

	is_reading_action_buttons: bool,
	is_reading_direction_buttons: bool,
}

impl InputMemory {
	pub fn new() -> Self {
		InputMemory {
			joyp_reg: 0x0F,

			is_reading_action_buttons: false,
			is_reading_direction_buttons: false,
		}
	}
	pub fn write(&mut self, data: u8) {
		self.joyp_reg = (data & 0xF0) | (self.joyp_reg & 0x0F);
		if data & (1 << 5) != 0 {
			self.is_reading_action_buttons = true
		} else {
			self.is_reading_action_buttons = false
		}
		if data & (1 << 4) != 0 {
			self.is_reading_direction_buttons = true
		} else {
			self.is_reading_direction_buttons = false
		}

	}
	pub fn read(&self) -> u8 {
		self.joyp_reg
	}
	pub fn update(&mut self, input_state: &InputState) {
		self.joyp_reg &= 0xF0;
		if self.is_reading_action_buttons {
			self.joyp_reg |= (input_state.is_start_pressed as u8)	<< 3
							| (input_state.is_select_pressed as u8)	<< 2
							| (input_state.is_b_pressed as u8)		<< 1
							| (input_state.is_a_pressed as u8);

		}
		if self.is_reading_direction_buttons {
			self.joyp_reg |= (input_state.is_down_pressed as u8)	<< 3
							| (input_state.is_up_pressed as u8)		<< 2
							| (input_state.is_left_pressed as u8)	<< 1
							| (input_state.is_right_pressed as u8);
		}
	}
}