use std::fmt::Debug;

pub struct MemoryBus {
	pub rom_bank0: [u8; 0x4000],		// 0x0000 - 0x3FFF
	pub rom_bank1: [u8; 0x4000],		// 0x4000 - 0x7FFF
	pub video_ram: [u8; 0x2000],		// 0x8000 - 0x9FFF
	pub cartr_ram: [u8; 0x2000],		// 0xA000 - 0xBFFF
	pub intern_ram: [u8; 0x2000],		// 0xC000 - 0xDFFF
/* echo of intern_ram: [u8; 0x1E00] */	// 0xE000 - 0xFDFF
	pub sprite_oam: [u8; 0x00A0],		// 0xFE00 - 0xFE9F
/* unmapped memory: [u8; 0x0060] */		// 0xFEA0 - 0xFEFF
	pub io_regis: [u8; 0x0080],			// 0xFF00 - 0xFF7F
	pub high_intern_ram: [u8; 0x007F],	// 0xFF80 - 0xFFFE
	pub interrupt_enable: u8			// 0xFFFF
}

impl MemoryBus {
	pub fn new() -> Self {
		MemoryBus {
			rom_bank0: [0; 0x4000],
			rom_bank1: [0; 0x4000],
			video_ram: [0; 0x2000],
			cartr_ram: [0; 0x2000],
			intern_ram: [0; 0x2000],
			sprite_oam: [0; 0x00A0],
			io_regis: [0; 0x0080],
			high_intern_ram: [0; 0x007F],
			interrupt_enable: 0
		}
	}
	pub fn init(&mut self) {
		self.write_byte(0xFF05, 0x00);
		self.write_byte(0xFF06, 0x00);
		self.write_byte(0xFF07, 0x00);
		self.write_byte(0xFF10, 0x80);
		self.write_byte(0xFF11, 0xBF);
		self.write_byte(0xFF12, 0xF3);
		self.write_byte(0xFF14, 0xBF);
		self.write_byte(0xFF16, 0x3F);
		self.write_byte(0xFF17, 0x00);
		self.write_byte(0xFF19, 0xBF);
		self.write_byte(0xFF1A, 0x7F);
		self.write_byte(0xFF1B, 0xFF);
		self.write_byte(0xFF1C, 0x9F);
		self.write_byte(0xFF1E, 0xBF);
		self.write_byte(0xFF20, 0xFF);
		self.write_byte(0xFF21, 0x00);
		self.write_byte(0xFF22, 0x00);
		self.write_byte(0xFF23, 0xBF);
		self.write_byte(0xFF24, 0x77);
		self.write_byte(0xFF25, 0xF3);
		self.write_byte(0xFF26, 0xF1);	//GB - 0xF1; SGB - 0xF0
		self.write_byte(0xFF40, 0x91);
		self.write_byte(0xFF42, 0x00);
		self.write_byte(0xFF43, 0x00);
		self.write_byte(0xFF45, 0x00);
		self.write_byte(0xFF47, 0xFC);
		self.write_byte(0xFF48, 0xFF);
		self.write_byte(0xFF49, 0xFF);
		self.write_byte(0xFF4A, 0x00);
		self.write_byte(0xFF4B, 0x00);
		self.write_byte(0xFFFF, 0x00)
	}
	pub fn read_byte(&self, address: u16) -> u8 {
		match address {
			0x0000..=0x3FFF	=>		 self.rom_bank0[(address - 0x0000) as usize],
			0x4000..=0x7FFF	=>		 self.rom_bank1[(address - 0x4000) as usize],
			0x8000..=0x9FFF	=>		 self.video_ram[(address - 0x8000) as usize],
			0xA000..=0xBFFF	=>		 self.cartr_ram[(address - 0xA000) as usize],
			0xC000..=0xDFFF	=>		self.intern_ram[(address - 0xC000) as usize],
			0xE000..=0xFDFF	=>		self.intern_ram[(address - 0xE000) as usize],
			0xFE00..=0xFE9F	=>		self.sprite_oam[(address - 0xFE00) as usize],
			0xFEA0..=0xFEFF	=>	0,
			0xFF00..=0xFF7F	=>		  self.io_regis[(address - 0xFF00) as usize],
			0xFF80..=0xFFFE	=> self.high_intern_ram[(address - 0xFF80) as usize],
			0xFFFF			=> self.interrupt_enable
		}
	}
	pub fn write_byte(&mut self, address: u16, data: u8) {
		match address {
			0x0000..=0x3FFF	=>		 {self.rom_bank0[(address - 0x0000) as usize] = data},
			0x4000..=0x7FFF	=>		 {self.rom_bank1[(address - 0x4000) as usize] = data},
			0x8000..=0x9FFF	=>		 {self.video_ram[(address - 0x8000) as usize] = data},
			0xA000..=0xBFFF	=>		 {self.cartr_ram[(address - 0xA000) as usize] = data},
			0xC000..=0xDFFF	=>		{self.intern_ram[(address - 0xC000) as usize] = data},
			0xE000..=0xFDFF	=>		{self.intern_ram[(address - 0xE000) as usize] = data},
			0xFE00..=0xFE9F	=>		{self.sprite_oam[(address - 0xFE00) as usize] = data},
			0xFEA0..=0xFEFF	=>	{},
			0xFF00..=0xFF7F	=>		  {self.io_regis[(address - 0xFF00) as usize] = data},
			0xFF80..=0xFFFE	=> {self.high_intern_ram[(address - 0xFF80) as usize] = data},
			0xFFFF			=> {self.interrupt_enable = data}
		}
	}
}

impl Debug for MemoryBus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemoryBus").field("interrupt_enable", &self.interrupt_enable).finish()
    }
}