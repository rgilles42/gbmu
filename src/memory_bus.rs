pub mod video_ram;
use self::video_ram::VideoRam;
use std::fmt::Debug;

pub struct MemoryBus {
	pub is_bootrom_mapped: bool,
	pub bootrom: [u8; 0x100],			// 0x0000 - 0x00FF
	pub rom_bank0: [u8; 0x4000],		// 0x0000 - 0x3FFF
	pub rom_bank1: [u8; 0x4000],		// 0x4000 - 0x7FFF
	pub video_ram: VideoRam,			// 0x8000 - 0x9FFF
	pub cartr_ram: [u8; 0x2000],		// 0xA000 - 0xBFFF
	pub intern_ram: [u8; 0x2000],		// 0xC000 - 0xDFFF
/* echo of intern_ram: [u8; 0x1E00] */	// 0xE000 - 0xFDFF
//	oam: [u8; 0x00A0],					// 0xFE00 - 0xFE9F => Inside VideoRam
/* unmapped memory: [u8; 0x0060] */		// 0xFEA0 - 0xFEFF
	pub io_regis: [u8; 0x0080],			// 0xFF00 - 0xFF7F
	pub high_intern_ram: [u8; 0x007F],	// 0xFF80 - 0xFFFE
	pub interrupt_enable: u8			// 0xFFFF
}

impl MemoryBus {
	pub fn new() -> Self {
		MemoryBus {
			bootrom: [0; 0x100],
			is_bootrom_mapped: false,
			rom_bank0: [0xFF; 0x4000],
			rom_bank1: [0; 0x4000],
			video_ram: VideoRam::new(),
			cartr_ram: [0; 0x2000],
			intern_ram: [0; 0x2000],
			io_regis: [0; 0x0080],
			high_intern_ram: [0; 0x007F],
			interrupt_enable: 0
		}
	}
	pub fn load_dmg_bootrom(&mut self) {
		let dmg_bootrom:[u8; 0x100] = [
			0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32, 0xCB, 0x7C, 0x20, 0xFB, 0x21, 0x26, 0xFF, 0x0E, 
			0x11, 0x3E, 0x80, 0x32, 0xE2, 0x0C, 0x3E, 0xF3, 0xE2, 0x32, 0x3E, 0x77, 0x77, 0x3E, 0xFC, 0xE0, 
			0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1A, 0xCD, 0x95, 0x00, 0xCD, 0x96, 0x00, 0x13, 0x7B, 
			0xFE, 0x34, 0x20, 0xF3, 0x11, 0xD8, 0x00, 0x06, 0x08, 0x1A, 0x13, 0x22, 0x23, 0x05, 0x20, 0xF9, 
			0x3E, 0x19, 0xEA, 0x10, 0x99, 0x21, 0x2F, 0x99, 0x0E, 0x0C, 0x3D, 0x28, 0x08, 0x32, 0x0D, 0x20, 
			0xF9, 0x2E, 0x0F, 0x18, 0xF3, 0x67, 0x3E, 0x64, 0x57, 0xE0, 0x42, 0x3E, 0x91, 0xE0, 0x40, 0x04, 
			0x1E, 0x02, 0x0E, 0x0C, 0xF0, 0x44, 0xFE, 0x90, 0x20, 0xFA, 0x0D, 0x20, 0xF7, 0x1D, 0x20, 0xF2, 
			0x0E, 0x13, 0x24, 0x7C, 0x1E, 0x83, 0xFE, 0x62, 0x28, 0x06, 0x1E, 0xC1, 0xFE, 0x64, 0x20, 0x06, 
			0x7B, 0xE2, 0x0C, 0x3E, 0x87, 0xE2, 0xF0, 0x42, 0x90, 0xE0, 0x42, 0x15, 0x20, 0xD2, 0x05, 0x20, 
			0x4F, 0x16, 0x20, 0x18, 0xCB, 0x4F, 0x06, 0x04, 0xC5, 0xCB, 0x11, 0x17, 0xC1, 0xCB, 0x11, 0x17, 
			0x05, 0x20, 0xF5, 0x22, 0x23, 0x22, 0x23, 0xC9, 0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 
			0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 
			0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 
			0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E, 0x3C, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x3C, 
			0x21, 0x04, 0x01, 0x11, 0xA8, 0x00, 0x1A, 0x13, 0xBE, 0x20, 0xFE, 0x23, 0x7D, 0xFE, 0x34, 0x20, 
			0xF5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xFB, 0x86, 0x20, 0xFE, 0x3E, 0x01, 0xE0, 0x50
		];
		self.bootrom.clone_from_slice(&dmg_bootrom);
		self.is_bootrom_mapped = true;
	}
	pub fn debug_insert_cart_logo(&mut self) {
		let logo_data : [u8; 48] = [
			0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 
			0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 
			0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 
			0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E
		];
		for (i, byte) in logo_data.iter().enumerate() {
			self.rom_bank0[0x104 + i] = *byte;
		}
	}
	// pub fn init(&mut self) {
	// 	self.write_byte(0xFF05, 0x00);
	// 	self.write_byte(0xFF06, 0x00);
	// 	self.write_byte(0xFF07, 0x00);
	// 	self.write_byte(0xFF10, 0x80);
	// 	self.write_byte(0xFF11, 0xBF);
	// 	self.write_byte(0xFF12, 0xF3);
	// 	self.write_byte(0xFF14, 0xBF);
	// 	self.write_byte(0xFF16, 0x3F);
	// 	self.write_byte(0xFF17, 0x00);
	// 	self.write_byte(0xFF19, 0xBF);
	// 	self.write_byte(0xFF1A, 0x7F);
	// 	self.write_byte(0xFF1B, 0xFF);
	// 	self.write_byte(0xFF1C, 0x9F);
	// 	self.write_byte(0xFF1E, 0xBF);
	// 	self.write_byte(0xFF20, 0xFF);
	// 	self.write_byte(0xFF21, 0x00);
	// 	self.write_byte(0xFF22, 0x00);
	// 	self.write_byte(0xFF23, 0xBF);
	// 	self.write_byte(0xFF24, 0x77);
	// 	self.write_byte(0xFF25, 0xF3);
	// 	self.write_byte(0xFF26, 0xF1);	// GB - 0xF1; SGB - 0xF0
	// 	self.write_byte(0xFF40, 0x91);	// LCDC: 10010001
	// 	self.write_byte(0xFF42, 0x00);	// SCY
	// 	self.write_byte(0xFF43, 0x00);	// SCX
	// 	self.write_byte(0xFF45, 0x00);	// LYC
	// 	self.write_byte(0xFF47, 0xFC);
	// 	self.write_byte(0xFF48, 0xFF);
	// 	self.write_byte(0xFF49, 0xFF);
	// 	self.write_byte(0xFF4A, 0x00);	// WY
	// 	self.write_byte(0xFF4B, 0x00);	// WX
	// 	self.write_byte(0xFFFF, 0x00)
	// }
	pub fn read_byte(&self, address: u16) -> u8 {
		match address {
			0x0000..=0x00FF	=> if self.is_bootrom_mapped
									  {self.bootrom[(address - 0x0000) as usize]}
							   else {self.rom_bank0[(address - 0x0000) as usize]},
			0x0100..=0x3FFF	=>		 self.rom_bank0[(address - 0x0000) as usize],
			0x4000..=0x7FFF	=>		 self.rom_bank1[(address - 0x4000) as usize],
			0x8000..=0x9FFF	=>		 self.video_ram.read(address as usize),
			0xA000..=0xBFFF	=>		 self.cartr_ram[(address - 0xA000) as usize],
			0xC000..=0xDFFF	=>		self.intern_ram[(address - 0xC000) as usize],
			0xE000..=0xFDFF	=>		self.intern_ram[(address - 0xE000) as usize],
			0xFE00..=0xFE9F	=>		self.video_ram.read(address as usize),
			0xFEA0..=0xFEFF	=> 0,
			0xFF40 | 0xFF47 =>		self.video_ram.read(address as usize),
			0xFF42			=>		self.video_ram.scy_ram,
			0xFF43			=>		self.video_ram.scx_ram,
			0xFF44			=>		self.video_ram.ly_ram,
			0xFF00..=0xFF7F	=>		  self.io_regis[(address - 0xFF00) as usize],
			0xFF80..=0xFFFE	=> self.high_intern_ram[(address - 0xFF80) as usize],
			0xFFFF			=> self.interrupt_enable
		}
	}
	pub fn write_byte(&mut self, address: u16, data: u8) {
		match address {
			0x0000..=0x00FF	=> if self.is_bootrom_mapped {}
							   else  {self.rom_bank0[(address - 0x0000) as usize] = data},
			0x0000..=0x3FFF	=>		 {self.rom_bank0[(address - 0x0000) as usize] = data},
			0x4000..=0x7FFF	=>		 {self.rom_bank1[(address - 0x4000) as usize] = data},
			0x8000..=0x9FFF	=>		  self.video_ram.write(address as usize, data),
			0xA000..=0xBFFF	=>		 {self.cartr_ram[(address - 0xA000) as usize] = data},
			0xC000..=0xDFFF	=>		{self.intern_ram[(address - 0xC000) as usize] = data},
			0xE000..=0xFDFF	=>		{self.intern_ram[(address - 0xE000) as usize] = data},
			0xFE00..=0xFE9F	=>		  self.video_ram.write(address as usize, data),
			0xFEA0..=0xFEFF	=> {},
			0xFF40 | 0xFF47 =>		  self.video_ram.write(address as usize, data),
			0xFF42			=>		 {self.video_ram.scy_ram = data},
			0xFF43			=>		 {self.video_ram.scx_ram = data},
			0xFF44			=>		 {self.video_ram.ly_ram = data},
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