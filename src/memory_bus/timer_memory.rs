#[derive(Debug, Clone, Copy)]
pub enum CPUFreqDivider {
	Ratio0x10, Ratio0x40, Ratio0x100, Ratio0x400
}
pub struct TimerMemory {
	pub div_reg: u8,	// 0xFF04
	pub tima_reg: u8,	// 0xFF05
	pub tim_reg: u8,	// 0xFF06
	tac_reg: u8,	// 0xFF07

	pub timer_enabled: bool,
	pub timer_freq_div: CPUFreqDivider
}

impl TimerMemory {
	pub fn new() -> Self {
		TimerMemory{
			div_reg: 0x00,
			tima_reg: 0x00,
			tim_reg: 0x00,
			tac_reg: 0x00,
			timer_enabled: false,
			timer_freq_div: CPUFreqDivider::Ratio0x400
		}
	}
	pub fn read(&self, address: usize) -> u8 {
		match address {
			0xFF04	=> self.div_reg,
			0xFF05	=> self.tima_reg,
			0xFF06	=> self.tim_reg,
			_		=> self.tac_reg
		}
	}
	pub fn write(&mut self, address: usize, data: u8) {
		match address {
			0xFF04	=> self.div_reg = 0x00,
			0xFF05	=> self.tima_reg = data,
			0xFF06	=> self.tim_reg = data,
			_		=> {
				self.tac_reg = data;
				self.timer_enabled = (data & (1 << 2)) != 0;
				self.timer_freq_div = match data & 0b11 {
					0b00 => CPUFreqDivider::Ratio0x400,
					0b01 => CPUFreqDivider::Ratio0x10,
					0b10 => CPUFreqDivider::Ratio0x40,
					_ => CPUFreqDivider::Ratio0x100
				}
			}
		}
	}
}