pub mod ppu_memory;
pub mod timer_memory;
pub mod input_memory;
mod cartridge;

use self::{ppu_memory::PPUMemory, cartridge::Cartridge, timer_memory::TimerMemory, input_memory::InputMemory};
use std::fmt::Debug;

pub struct MemoryBus {
	bootrom_1: [u8; 0x100],			// 0x0000 - 0x00FF
	bootrom_2: [u8; 0x700],			// 0x0200 - 0x08FF
	pub cartridge: Cartridge,		// 0x0000 - 0x7FFF + 0xA000 - 0xBFFF
	pub ppu_memory: PPUMemory,		// 0x8000 - 0x9FFF + 0xFE00 - 0xFE9F + 0xFF40 - 0xFF45 + 0xFF47 - 0xFF4B + 0xFF68 - 0xFF6B
	intern_ram: [u8; 0x1000],		// 0xC000 - 0xCFFF + echo at 0xE000 - 0xEFFF
	intern_ram2: [[u8; 0x1000]; 7],		// 0xD000 - 0xDFFF + echo at 0xF000 - 0xFDFF
	/* unmapped memory */			// 0xFEA0 - 0xFEFF => Read returns 0, write does nothing
	pub input_memory: InputMemory,	// 0xFF00
	pub timer_memory: TimerMemory,	// 0xFF04 - 0xFF07
	pub is_double_speed: bool,		// 0xFF4D & 0x80
	pub speed_chg_scheduled: bool,	// 0xFF4D & 0x01
	vbk_reg: bool,					// 0xFF4F
	bootrom_reg: u8,				// 0xFF50
	svbk_reg: u8,					// 0xFF70
	io_regis: [u8; 0x007F],			// 0xFF01 - 0xFF7F
	high_intern_ram: [u8; 0x007F],	// 0xFF80 - 0xFFFE
	interrupt_enable: u8,			// 0xFFFF

	pub is_cgb: bool
}

impl MemoryBus {
	pub fn new(rom_path: Option<&str>, force_dmg: bool) -> Self {
		let mut res = MemoryBus {
			ppu_memory: PPUMemory::new(),
			timer_memory: TimerMemory::new(),
			input_memory: InputMemory::new(),
			cartridge: Cartridge::new(rom_path),
			bootrom_1: [0; 0x100],
			bootrom_2: [0; 0x700],
			intern_ram: [0; 0x1000],
			intern_ram2: [[0; 0x1000]; 7],
			io_regis: [0; 0x007F],
			is_double_speed: false,
			speed_chg_scheduled: false,
			vbk_reg: false,
			bootrom_reg: 0x01,
			svbk_reg: 0x01,
			high_intern_ram: [0; 0x007F],
			interrupt_enable: 0,
			is_cgb: false
		};
		if !force_dmg && res.cartridge.is_cgb {
			res.is_cgb = true;
		}
		if !res.is_cgb {
			res.load_dmg_bootrom();
		}  else {
		 	res.load_cgb_bootrom();
		}
		res
	}
	fn load_dmg_bootrom(&mut self) {
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
		self.bootrom_1.clone_from_slice(&dmg_bootrom);
		self.bootrom_reg = 0x00;
	}
	fn load_cgb_bootrom(&mut self) {
		let cgb_bootrom_1: [u8; 0x100] = [
			0x31, 0xFE, 0xFF, 0x3E, 0x02, 0xC3, 0x7C, 0x00, 0xD3, 0x00, 0x98, 0xA0,
			0x12, 0xD3, 0x00, 0x80, 0x00, 0x40, 0x1E, 0x53, 0xD0, 0x00, 0x1F, 0x42,
			0x1C, 0x00, 0x14, 0x2A, 0x4D, 0x19, 0x8C, 0x7E, 0x00, 0x7C, 0x31, 0x6E,
			0x4A, 0x45, 0x52, 0x4A, 0x00, 0x00, 0xFF, 0x53, 0x1F, 0x7C, 0xFF, 0x03,
			0x1F, 0x00, 0xFF, 0x1F, 0xA7, 0x00, 0xEF, 0x1B, 0x1F, 0x00, 0xEF, 0x1B,
			0x00, 0x7C, 0x00, 0x00, 0xFF, 0x03, 0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D,
			0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08,
			0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD,
			0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
			0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E, 0x3C, 0x42, 0xB9, 0xA5, 0xB9, 0xA5,
			0x42, 0x3C, 0x58, 0x43, 0xE0, 0x70, 0x3E, 0xFC, 0xE0, 0x47, 0xCD, 0x75,
			0x02, 0xCD, 0x00, 0x02, 0x26, 0xD0, 0xCD, 0x03, 0x02, 0x21, 0x00, 0xFE,
			0x0E, 0xA0, 0xAF, 0x22, 0x0D, 0x20, 0xFC, 0x11, 0x04, 0x01, 0x21, 0x10,
			0x80, 0x4C, 0x1A, 0xE2, 0x0C, 0xCD, 0xC6, 0x03, 0xCD, 0xC7, 0x03, 0x13,
			0x7B, 0xFE, 0x34, 0x20, 0xF1, 0x11, 0x72, 0x00, 0x06, 0x08, 0x1A, 0x13,
			0x22, 0x23, 0x05, 0x20, 0xF9, 0xCD, 0xF0, 0x03, 0x3E, 0x01, 0xE0, 0x4F,
			0x3E, 0x91, 0xE0, 0x40, 0x21, 0xB2, 0x98, 0x06, 0x4E, 0x0E, 0x44, 0xCD,
			0x91, 0x02, 0xAF, 0xE0, 0x4F, 0x0E, 0x80, 0x21, 0x42, 0x00, 0x06, 0x18,
			0xF2, 0x0C, 0xBE, 0x20, 0xFE, 0x23, 0x05, 0x20, 0xF7, 0x21, 0x34, 0x01,
			0x06, 0x19, 0x78, 0x86, 0x2C, 0x05, 0x20, 0xFB, 0x86, 0x20, 0xFE, 0xCD,
			0x1C, 0x03, 0x18, 0x02, 0x00, 0x00, 0xCD, 0xD0, 0x05, 0xAF, 0xE0, 0x70,
			0x3E, 0x11, 0xE0, 0x50
		];
		let cgb_bootrom_2: [u8; 0x700] = [
			0x21, 0x00, 0x80, 0xAF, 0x22, 0xCB, 0x6C, 0x28, 0xFB, 0xC9, 0x2A, 0x12,
			0x13, 0x0D, 0x20, 0xFA, 0xC9, 0xE5, 0x21, 0x0F, 0xFF, 0xCB, 0x86, 0xCB,
			0x46, 0x28, 0xFC, 0xE1, 0xC9, 0x11, 0x00, 0xFF, 0x21, 0x03, 0xD0, 0x0E,
			0x0F, 0x3E, 0x30, 0x12, 0x3E, 0x20, 0x12, 0x1A, 0x2F, 0xA1, 0xCB, 0x37,
			0x47, 0x3E, 0x10, 0x12, 0x1A, 0x2F, 0xA1, 0xB0, 0x4F, 0x7E, 0xA9, 0xE6,
			0xF0, 0x47, 0x2A, 0xA9, 0xA1, 0xB0, 0x32, 0x47, 0x79, 0x77, 0x3E, 0x30,
			0x12, 0xC9, 0x3E, 0x80, 0xE0, 0x68, 0xE0, 0x6A, 0x0E, 0x6B, 0x2A, 0xE2,
			0x05, 0x20, 0xFB, 0x4A, 0x09, 0x43, 0x0E, 0x69, 0x2A, 0xE2, 0x05, 0x20,
			0xFB, 0xC9, 0xC5, 0xD5, 0xE5, 0x21, 0x00, 0xD8, 0x06, 0x01, 0x16, 0x3F,
			0x1E, 0x40, 0xCD, 0x4A, 0x02, 0xE1, 0xD1, 0xC1, 0xC9, 0x3E, 0x80, 0xE0,
			0x26, 0xE0, 0x11, 0x3E, 0xF3, 0xE0, 0x12, 0xE0, 0x25, 0x3E, 0x77, 0xE0,
			0x24, 0x21, 0x30, 0xFF, 0xAF, 0x0E, 0x10, 0x22, 0x2F, 0x0D, 0x20, 0xFB,
			0xC9, 0xCD, 0x11, 0x02, 0xCD, 0x62, 0x02, 0x79, 0xFE, 0x38, 0x20, 0x14,
			0xE5, 0xAF, 0xE0, 0x4F, 0x21, 0xA7, 0x99, 0x3E, 0x38, 0x22, 0x3C, 0xFE,
			0x3F, 0x20, 0xFA, 0x3E, 0x01, 0xE0, 0x4F, 0xE1, 0xC5, 0xE5, 0x21, 0x43,
			0x01, 0xCB, 0x7E, 0xCC, 0x89, 0x05, 0xE1, 0xC1, 0xCD, 0x11, 0x02, 0x79,
			0xD6, 0x30, 0xD2, 0x06, 0x03, 0x79, 0xFE, 0x01, 0xCA, 0x06, 0x03, 0x7D,
			0xFE, 0xD1, 0x28, 0x21, 0xC5, 0x06, 0x03, 0x0E, 0x01, 0x16, 0x03, 0x7E,
			0xE6, 0xF8, 0xB1, 0x22, 0x15, 0x20, 0xF8, 0x0C, 0x79, 0xFE, 0x06, 0x20,
			0xF0, 0x11, 0x11, 0x00, 0x19, 0x05, 0x20, 0xE7, 0x11, 0xA1, 0xFF, 0x19,
			0xC1, 0x04, 0x78, 0x1E, 0x83, 0xFE, 0x62, 0x28, 0x06, 0x1E, 0xC1, 0xFE,
			0x64, 0x20, 0x07, 0x7B, 0xE0, 0x13, 0x3E, 0x87, 0xE0, 0x14, 0xFA, 0x02,
			0xD0, 0xFE, 0x00, 0x28, 0x0A, 0x3D, 0xEA, 0x02, 0xD0, 0x79, 0xFE, 0x01,
			0xCA, 0x91, 0x02, 0x0D, 0xC2, 0x91, 0x02, 0xC9, 0x0E, 0x26, 0xCD, 0x4A,
			0x03, 0xCD, 0x11, 0x02, 0xCD, 0x62, 0x02, 0x0D, 0x20, 0xF4, 0xCD, 0x11,
			0x02, 0x3E, 0x01, 0xE0, 0x4F, 0xCD, 0x3E, 0x03, 0xCD, 0x41, 0x03, 0xAF,
			0xE0, 0x4F, 0xCD, 0x3E, 0x03, 0xC9, 0x21, 0x08, 0x00, 0x11, 0x51, 0xFF,
			0x0E, 0x05, 0xCD, 0x0A, 0x02, 0xC9, 0xC5, 0xD5, 0xE5, 0x21, 0x40, 0xD8,
			0x0E, 0x20, 0x7E, 0xE6, 0x1F, 0xFE, 0x1F, 0x28, 0x01, 0x3C, 0x57, 0x2A,
			0x07, 0x07, 0x07, 0xE6, 0x07, 0x47, 0x3A, 0x07, 0x07, 0x07, 0xE6, 0x18,
			0xB0, 0xFE, 0x1F, 0x28, 0x01, 0x3C, 0x0F, 0x0F, 0x0F, 0x47, 0xE6, 0xE0,
			0xB2, 0x22, 0x78, 0xE6, 0x03, 0x5F, 0x7E, 0x0F, 0x0F, 0xE6, 0x1F, 0xFE,
			0x1F, 0x28, 0x01, 0x3C, 0x07, 0x07, 0xB3, 0x22, 0x0D, 0x20, 0xC7, 0xE1,
			0xD1, 0xC1, 0xC9, 0x0E, 0x00, 0x1A, 0xE6, 0xF0, 0xCB, 0x49, 0x28, 0x02,
			0xCB, 0x37, 0x47, 0x23, 0x7E, 0xB0, 0x22, 0x1A, 0xE6, 0x0F, 0xCB, 0x49,
			0x20, 0x02, 0xCB, 0x37, 0x47, 0x23, 0x7E, 0xB0, 0x22, 0x13, 0xCB, 0x41,
			0x28, 0x0D, 0xD5, 0x11, 0xF8, 0xFF, 0xCB, 0x49, 0x28, 0x03, 0x11, 0x08,
			0x00, 0x19, 0xD1, 0x0C, 0x79, 0xFE, 0x18, 0x20, 0xCC, 0xC9, 0x47, 0xD5,
			0x16, 0x04, 0x58, 0xCB, 0x10, 0x17, 0xCB, 0x13, 0x17, 0x15, 0x20, 0xF6,
			0xD1, 0x22, 0x23, 0x22, 0x23, 0xC9, 0x3E, 0x19, 0xEA, 0x10, 0x99, 0x21,
			0x2F, 0x99, 0x0E, 0x0C, 0x3D, 0x28, 0x08, 0x32, 0x0D, 0x20, 0xF9, 0x2E,
			0x0F, 0x18, 0xF3, 0xC9, 0x3E, 0x01, 0xE0, 0x4F, 0xCD, 0x00, 0x02, 0x11,
			0x07, 0x06, 0x21, 0x80, 0x80, 0x0E, 0xC0, 0x1A, 0x22, 0x23, 0x22, 0x23,
			0x13, 0x0D, 0x20, 0xF7, 0x11, 0x04, 0x01, 0xCD, 0x8F, 0x03, 0x01, 0xA8,
			0xFF, 0x09, 0xCD, 0x8F, 0x03, 0x01, 0xF8, 0xFF, 0x09, 0x11, 0x72, 0x00,
			0x0E, 0x08, 0x23, 0x1A, 0x22, 0x13, 0x0D, 0x20, 0xF9, 0x21, 0xC2, 0x98,
			0x06, 0x08, 0x3E, 0x08, 0x0E, 0x10, 0x22, 0x0D, 0x20, 0xFC, 0x11, 0x10,
			0x00, 0x19, 0x05, 0x20, 0xF3, 0xAF, 0xE0, 0x4F, 0x21, 0xC2, 0x98, 0x3E,
			0x08, 0x22, 0x3C, 0xFE, 0x18, 0x20, 0x02, 0x2E, 0xE2, 0xFE, 0x28, 0x20,
			0x03, 0x21, 0x02, 0x99, 0xFE, 0x38, 0x20, 0xED, 0x21, 0xD8, 0x08, 0x11,
			0x40, 0xD8, 0x06, 0x08, 0x3E, 0xFF, 0x12, 0x13, 0x12, 0x13, 0x0E, 0x02,
			0xCD, 0x0A, 0x02, 0x3E, 0x00, 0x12, 0x13, 0x12, 0x13, 0x13, 0x13, 0x05,
			0x20, 0xEA, 0xCD, 0x62, 0x02, 0x21, 0x4B, 0x01, 0x7E, 0xFE, 0x33, 0x20,
			0x0B, 0x2E, 0x44, 0x1E, 0x30, 0x2A, 0xBB, 0x20, 0x49, 0x1C, 0x18, 0x04,
			0x2E, 0x4B, 0x1E, 0x01, 0x2A, 0xBB, 0x20, 0x3E, 0x2E, 0x34, 0x01, 0x10,
			0x00, 0x2A, 0x80, 0x47, 0x0D, 0x20, 0xFA, 0xEA, 0x00, 0xD0, 0x21, 0xC7,
			0x06, 0x0E, 0x00, 0x2A, 0xB8, 0x28, 0x08, 0x0C, 0x79, 0xFE, 0x4F, 0x20,
			0xF6, 0x18, 0x1F, 0x79, 0xD6, 0x41, 0x38, 0x1C, 0x21, 0x16, 0x07, 0x16,
			0x00, 0x5F, 0x19, 0xFA, 0x37, 0x01, 0x57, 0x7E, 0xBA, 0x28, 0x0D, 0x11,
			0x0E, 0x00, 0x19, 0x79, 0x83, 0x4F, 0xD6, 0x5E, 0x38, 0xED, 0x0E, 0x00,
			0x21, 0x33, 0x07, 0x06, 0x00, 0x09, 0x7E, 0xE6, 0x1F, 0xEA, 0x08, 0xD0,
			0x7E, 0xE6, 0xE0, 0x07, 0x07, 0x07, 0xEA, 0x0B, 0xD0, 0xCD, 0xE9, 0x04,
			0xC9, 0x11, 0x91, 0x07, 0x21, 0x00, 0xD9, 0xFA, 0x0B, 0xD0, 0x47, 0x0E,
			0x1E, 0xCB, 0x40, 0x20, 0x02, 0x13, 0x13, 0x1A, 0x22, 0x20, 0x02, 0x1B,
			0x1B, 0xCB, 0x48, 0x20, 0x02, 0x13, 0x13, 0x1A, 0x22, 0x13, 0x13, 0x20,
			0x02, 0x1B, 0x1B, 0xCB, 0x50, 0x28, 0x05, 0x1B, 0x2B, 0x1A, 0x22, 0x13,
			0x1A, 0x22, 0x13, 0x0D, 0x20, 0xD7, 0x21, 0x00, 0xD9, 0x11, 0x00, 0xDA,
			0xCD, 0x64, 0x05, 0xC9, 0x21, 0x12, 0x00, 0xFA, 0x05, 0xD0, 0x07, 0x07,
			0x06, 0x00, 0x4F, 0x09, 0x11, 0x40, 0xD8, 0x06, 0x08, 0xE5, 0x0E, 0x02,
			0xCD, 0x0A, 0x02, 0x13, 0x13, 0x13, 0x13, 0x13, 0x13, 0xE1, 0x05, 0x20,
			0xF0, 0x11, 0x42, 0xD8, 0x0E, 0x02, 0xCD, 0x0A, 0x02, 0x11, 0x4A, 0xD8,
			0x0E, 0x02, 0xCD, 0x0A, 0x02, 0x2B, 0x2B, 0x11, 0x44, 0xD8, 0x0E, 0x02,
			0xCD, 0x0A, 0x02, 0xC9, 0x0E, 0x60, 0x2A, 0xE5, 0xC5, 0x21, 0xE8, 0x07,
			0x06, 0x00, 0x4F, 0x09, 0x0E, 0x08, 0xCD, 0x0A, 0x02, 0xC1, 0xE1, 0x0D,
			0x20, 0xEC, 0xC9, 0xFA, 0x08, 0xD0, 0x11, 0x18, 0x00, 0x3C, 0x3D, 0x28,
			0x03, 0x19, 0x20, 0xFA, 0xC9, 0xCD, 0x1D, 0x02, 0x78, 0xE6, 0xFF, 0x28,
			0x0F, 0x21, 0xE4, 0x08, 0x06, 0x00, 0x2A, 0xB9, 0x28, 0x08, 0x04, 0x78,
			0xFE, 0x0C, 0x20, 0xF6, 0x18, 0x2D, 0x78, 0xEA, 0x05, 0xD0, 0x3E, 0x1E,
			0xEA, 0x02, 0xD0, 0x11, 0x0B, 0x00, 0x19, 0x56, 0x7A, 0xE6, 0x1F, 0x5F,
			0x21, 0x08, 0xD0, 0x3A, 0x22, 0x7B, 0x77, 0x7A, 0xE6, 0xE0, 0x07, 0x07,
			0x07, 0x5F, 0x21, 0x0B, 0xD0, 0x3A, 0x22, 0x7B, 0x77, 0xCD, 0xE9, 0x04,
			0xCD, 0x28, 0x05, 0xC9, 0xCD, 0x11, 0x02, 0xFA, 0x43, 0x01, 0xCB, 0x7F,
			0x28, 0x04, 0xE0, 0x4C, 0x18, 0x28, 0x3E, 0x04, 0xE0, 0x4C, 0x3E, 0x01,
			0xE0, 0x6C, 0x21, 0x00, 0xDA, 0xCD, 0x7B, 0x05, 0x06, 0x10, 0x16, 0x00,
			0x1E, 0x08, 0xCD, 0x4A, 0x02, 0x21, 0x7A, 0x00, 0xFA, 0x00, 0xD0, 0x47,
			0x0E, 0x02, 0x2A, 0xB8, 0xCC, 0xDA, 0x03, 0x0D, 0x20, 0xF8, 0xC9, 0x01,
			0x0F, 0x3F, 0x7E, 0xFF, 0xFF, 0xC0, 0x00, 0xC0, 0xF0, 0xF1, 0x03, 0x7C,
			0xFC, 0xFE, 0xFE, 0x03, 0x07, 0x07, 0x0F, 0xE0, 0xE0, 0xF0, 0xF0, 0x1E,
			0x3E, 0x7E, 0xFE, 0x0F, 0x0F, 0x1F, 0x1F, 0xFF, 0xFF, 0x00, 0x00, 0x01,
			0x01, 0x01, 0x03, 0xFF, 0xFF, 0xE1, 0xE0, 0xC0, 0xF0, 0xF9, 0xFB, 0x1F,
			0x7F, 0xF8, 0xE0, 0xF3, 0xFD, 0x3E, 0x1E, 0xE0, 0xF0, 0xF9, 0x7F, 0x3E,
			0x7C, 0xF8, 0xE0, 0xF8, 0xF0, 0xF0, 0xF8, 0x00, 0x00, 0x7F, 0x7F, 0x07,
			0x0F, 0x9F, 0xBF, 0x9E, 0x1F, 0xFF, 0xFF, 0x0F, 0x1E, 0x3E, 0x3C, 0xF1,
			0xFB, 0x7F, 0x7F, 0xFE, 0xDE, 0xDF, 0x9F, 0x1F, 0x3F, 0x3E, 0x3C, 0xF8,
			0xF8, 0x00, 0x00, 0x03, 0x03, 0x07, 0x07, 0xFF, 0xFF, 0xC1, 0xC0, 0xF3,
			0xE7, 0xF7, 0xF3, 0xC0, 0xC0, 0xC0, 0xC0, 0x1F, 0x1F, 0x1E, 0x3E, 0x3F,
			0x1F, 0x3E, 0x3E, 0x80, 0x00, 0x00, 0x00, 0x7C, 0x1F, 0x07, 0x00, 0x0F,
			0xFF, 0xFE, 0x00, 0x7C, 0xF8, 0xF0, 0x00, 0x1F, 0x0F, 0x0F, 0x00, 0x7C,
			0xF8, 0xF8, 0x00, 0x3F, 0x3E, 0x1C, 0x00, 0x0F, 0x0F, 0x0F, 0x00, 0x7C,
			0xFF, 0xFF, 0x00, 0x00, 0xF8, 0xF8, 0x00, 0x07, 0x0F, 0x0F, 0x00, 0x81,
			0xFF, 0xFF, 0x00, 0xF3, 0xE1, 0x80, 0x00, 0xE0, 0xFF, 0x7F, 0x00, 0xFC,
			0xF0, 0xC0, 0x00, 0x3E, 0x7C, 0x7C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x88, 0x16, 0x36, 0xD1, 0xDB, 0xF2, 0x3C, 0x8C, 0x92, 0x3D, 0x5C, 0x58,
			0xC9, 0x3E, 0x70, 0x1D, 0x59, 0x69, 0x19, 0x35, 0xA8, 0x14, 0xAA, 0x75,
			0x95, 0x99, 0x34, 0x6F, 0x15, 0xFF, 0x97, 0x4B, 0x90, 0x17, 0x10, 0x39,
			0xF7, 0xF6, 0xA2, 0x49, 0x4E, 0x43, 0x68, 0xE0, 0x8B, 0xF0, 0xCE, 0x0C,
			0x29, 0xE8, 0xB7, 0x86, 0x9A, 0x52, 0x01, 0x9D, 0x71, 0x9C, 0xBD, 0x5D,
			0x6D, 0x67, 0x3F, 0x6B, 0xB3, 0x46, 0x28, 0xA5, 0xC6, 0xD3, 0x27, 0x61,
			0x18, 0x66, 0x6A, 0xBF, 0x0D, 0xF4, 0x42, 0x45, 0x46, 0x41, 0x41, 0x52,
			0x42, 0x45, 0x4B, 0x45, 0x4B, 0x20, 0x52, 0x2D, 0x55, 0x52, 0x41, 0x52,
			0x20, 0x49, 0x4E, 0x41, 0x49, 0x4C, 0x49, 0x43, 0x45, 0x20, 0x52, 0x7C,
			0x08, 0x12, 0xA3, 0xA2, 0x07, 0x87, 0x4B, 0x20, 0x12, 0x65, 0xA8, 0x16,
			0xA9, 0x86, 0xB1, 0x68, 0xA0, 0x87, 0x66, 0x12, 0xA1, 0x30, 0x3C, 0x12,
			0x85, 0x12, 0x64, 0x1B, 0x07, 0x06, 0x6F, 0x6E, 0x6E, 0xAE, 0xAF, 0x6F,
			0xB2, 0xAF, 0xB2, 0xA8, 0xAB, 0x6F, 0xAF, 0x86, 0xAE, 0xA2, 0xA2, 0x12,
			0xAF, 0x13, 0x12, 0xA1, 0x6E, 0xAF, 0xAF, 0xAD, 0x06, 0x4C, 0x6E, 0xAF,
			0xAF, 0x12, 0x7C, 0xAC, 0xA8, 0x6A, 0x6E, 0x13, 0xA0, 0x2D, 0xA8, 0x2B,
			0xAC, 0x64, 0xAC, 0x6D, 0x87, 0xBC, 0x60, 0xB4, 0x13, 0x72, 0x7C, 0xB5,
			0xAE, 0xAE, 0x7C, 0x7C, 0x65, 0xA2, 0x6C, 0x64, 0x85, 0x80, 0xB0, 0x40,
			0x88, 0x20, 0x68, 0xDE, 0x00, 0x70, 0xDE, 0x20, 0x78, 0x20, 0x20, 0x38,
			0x20, 0xB0, 0x90, 0x20, 0xB0, 0xA0, 0xE0, 0xB0, 0xC0, 0x98, 0xB6, 0x48,
			0x80, 0xE0, 0x50, 0x1E, 0x1E, 0x58, 0x20, 0xB8, 0xE0, 0x88, 0xB0, 0x10,
			0x20, 0x00, 0x10, 0x20, 0xE0, 0x18, 0xE0, 0x18, 0x00, 0x18, 0xE0, 0x20,
			0xA8, 0xE0, 0x20, 0x18, 0xE0, 0x00, 0x20, 0x18, 0xD8, 0xC8, 0x18, 0xE0,
			0x00, 0xE0, 0x40, 0x28, 0x28, 0x28, 0x18, 0xE0, 0x60, 0x20, 0x18, 0xE0,
			0x00, 0x00, 0x08, 0xE0, 0x18, 0x30, 0xD0, 0xD0, 0xD0, 0x20, 0xE0, 0xE8,
			0xFF, 0x7F, 0xBF, 0x32, 0xD0, 0x00, 0x00, 0x00, 0x9F, 0x63, 0x79, 0x42,
			0xB0, 0x15, 0xCB, 0x04, 0xFF, 0x7F, 0x31, 0x6E, 0x4A, 0x45, 0x00, 0x00,
			0xFF, 0x7F, 0xEF, 0x1B, 0x00, 0x02, 0x00, 0x00, 0xFF, 0x7F, 0x1F, 0x42,
			0xF2, 0x1C, 0x00, 0x00, 0xFF, 0x7F, 0x94, 0x52, 0x4A, 0x29, 0x00, 0x00,
			0xFF, 0x7F, 0xFF, 0x03, 0x2F, 0x01, 0x00, 0x00, 0xFF, 0x7F, 0xEF, 0x03,
			0xD6, 0x01, 0x00, 0x00, 0xFF, 0x7F, 0xB5, 0x42, 0xC8, 0x3D, 0x00, 0x00,
			0x74, 0x7E, 0xFF, 0x03, 0x80, 0x01, 0x00, 0x00, 0xFF, 0x67, 0xAC, 0x77,
			0x13, 0x1A, 0x6B, 0x2D, 0xD6, 0x7E, 0xFF, 0x4B, 0x75, 0x21, 0x00, 0x00,
			0xFF, 0x53, 0x5F, 0x4A, 0x52, 0x7E, 0x00, 0x00, 0xFF, 0x4F, 0xD2, 0x7E,
			0x4C, 0x3A, 0xE0, 0x1C, 0xED, 0x03, 0xFF, 0x7F, 0x5F, 0x25, 0x00, 0x00,
			0x6A, 0x03, 0x1F, 0x02, 0xFF, 0x03, 0xFF, 0x7F, 0xFF, 0x7F, 0xDF, 0x01,
			0x12, 0x01, 0x00, 0x00, 0x1F, 0x23, 0x5F, 0x03, 0xF2, 0x00, 0x09, 0x00,
			0xFF, 0x7F, 0xEA, 0x03, 0x1F, 0x01, 0x00, 0x00, 0x9F, 0x29, 0x1A, 0x00,
			0x0C, 0x00, 0x00, 0x00, 0xFF, 0x7F, 0x7F, 0x02, 0x1F, 0x00, 0x00, 0x00,
			0xFF, 0x7F, 0xE0, 0x03, 0x06, 0x02, 0x20, 0x01, 0xFF, 0x7F, 0xEB, 0x7E,
			0x1F, 0x00, 0x00, 0x7C, 0xFF, 0x7F, 0xFF, 0x3F, 0x00, 0x7E, 0x1F, 0x00,
			0xFF, 0x7F, 0xFF, 0x03, 0x1F, 0x00, 0x00, 0x00, 0xFF, 0x03, 0x1F, 0x00,
			0x0C, 0x00, 0x00, 0x00, 0xFF, 0x7F, 0x3F, 0x03, 0x93, 0x01, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x42, 0x7F, 0x03, 0xFF, 0x7F, 0xFF, 0x7F, 0x8C, 0x7E,
			0x00, 0x7C, 0x00, 0x00, 0xFF, 0x7F, 0xEF, 0x1B, 0x80, 0x61, 0x00, 0x00,
			0xFF, 0x7F, 0x00, 0x7C, 0xE0, 0x03, 0x1F, 0x7C, 0x1F, 0x00, 0xFF, 0x03,
			0x40, 0x41, 0x42, 0x20, 0x21, 0x22, 0x80, 0x81, 0x82, 0x10, 0x11, 0x12,
			0x12, 0xB0, 0x79, 0xB8, 0xAD, 0x16, 0x17, 0x07, 0xBA, 0x05, 0x7C, 0x13,
			0x00, 0x00, 0x00, 0x00
		];
		self.bootrom_1.clone_from_slice(&cgb_bootrom_1);
		self.bootrom_2.clone_from_slice(&cgb_bootrom_2);
		self.bootrom_reg = 0x00;
	}
	pub fn read_byte(&self, address: u16) -> u8 {
		match address {
			0x0000..=0x00FF	=> if self.bootrom_reg == 0x00
									 {self.bootrom_1[(address - 0x0000) as usize]}
							else     {self.cartridge.read(address as usize)},
			0x0100..=0x01FF	=>		  self.cartridge.read(address as usize),
			0x0200..=0x08FF	=>	if self.bootrom_reg == 0x00 {
									self.bootrom_2[(address - 0x0200) as usize]
								} else {
									self.cartridge.read(address as usize)
								}
			0x0900..=0x7FFF	=>		  self.cartridge.read(address as usize),
			0x8000..=0x9FFF	=>		  self.ppu_memory.read(address as usize, self.is_cgb && self.vbk_reg),
			0xA000..=0xBFFF	=>		  self.cartridge.read(address as usize),
			0xC000..=0xCFFF	=>		  self.intern_ram[(address - 0xC000) as usize],
			0xD000..=0xDFFF	=>		  self.intern_ram2[self.svbk_reg as usize - 1][(address - 0xD000) as usize],
			0xE000..=0xEFFF	=>		  self.intern_ram[(address - 0xE000) as usize],
			0xF000..=0xFDFF	=>		  self.intern_ram2[self.svbk_reg as usize - 1][(address - 0xF000) as usize],
			0xFE00..=0xFE9F	=>		  self.ppu_memory.read(address as usize, false),
			0xFEA0..=0xFEFF	=> 0,
			0xFF00			=>		self.input_memory.read(),
			0xFF04..=0xFF07 =>		self.timer_memory.read(address as usize),
			0xFF26			=>		0x00,
			0xFF40 | 0xFF47 =>		  self.ppu_memory.read(address as usize, false),
			0xFF41			=>		self.ppu_memory.read(address as usize, false),
			0xFF42			=>		  self.ppu_memory.scy_ram,
			0xFF43			=>		  self.ppu_memory.scx_ram,
			0xFF44			=>		  self.ppu_memory.read(address as usize, false),
			0xFF45			=>		  self.ppu_memory.lyc_ram,
			0xFF46			=>		  self.ppu_memory.oam_dma_reg,
			0xFF48 | 0xFF49 =>		self.ppu_memory.read(address as usize, false),
			0xFF4A			=>		  self.ppu_memory.wy_ram,
			0xFF4B			=>		  self.ppu_memory.wx_ram,
			0xFF4D			=>		(self.is_double_speed as u8) << 7 | self.speed_chg_scheduled as u8,
			0xFF4F			=>		  0xFE | self.vbk_reg as u8,
			0xFF50			=>		  self.bootrom_reg,
			0xFF51			=>		((self.ppu_memory.vram_dma_src_regs & 0xFF00) >> 8) as u8,
			0xFF52			=>		(self.ppu_memory.vram_dma_src_regs & 0x00F0) as u8,
			0xFF53			=>		((self.ppu_memory.vram_dma_dst_regs & 0x1F00) >> 8) as u8,
			0xFF54			=>		(self.ppu_memory.vram_dma_dst_regs & 0x00F0) as u8,
			0xFF55			=>		(!self.ppu_memory.vram_dma_is_active as u8) << 7 | self.ppu_memory.vram_dma_stat & 0x7F,
			0xFF68..=0xFF6B =>		self.ppu_memory.read(address as usize, false),
			0xFF70			=>		self.svbk_reg,
			0xFF01..=0xFF7F	=>		  self.io_regis[(address - 0xFF01) as usize],
			0xFF80..=0xFFFE	=> self.high_intern_ram[(address - 0xFF80) as usize],
			0xFFFF			=> self.interrupt_enable
		}
	}
	pub fn write_byte(&mut self, address: u16, data: u8) {
		match address {
			0x0000..=0x00FF	=> if self.bootrom_reg == 0x00 {}
							   else   {self.cartridge.write(address as usize, data)},
			0x0100..=0x01FF	=>		  {self.cartridge.write(address as usize, data)},
			0x0200..=0x08FF	=> if self.bootrom_reg == 0x00 {}
								else  {self.cartridge.write(address as usize, data)},
			0x0900..=0x7FFF	=>		  {self.cartridge.write(address as usize, data)},
			0x8000..=0x9FFF	=>		   self.ppu_memory.write(address as usize, data, self.is_cgb && self.vbk_reg),
			0xA000..=0xBFFF	=>		  {self.cartridge.write(address as usize, data)},
			0xC000..=0xCFFF	=>		  {self.intern_ram[(address - 0xC000) as usize] = data},
			0xD000..=0xDFFF	=>		  {self.intern_ram2[self.svbk_reg as usize - 1][(address - 0xD000) as usize] = data},
			0xE000..=0xEFFF	=>		  {self.intern_ram[(address - 0xE000) as usize] = data},
			0xF000..=0xFDFF	=>		  {self.intern_ram2[self.svbk_reg as usize - 1][(address - 0xF000) as usize] = data},
			0xFE00..=0xFE9F	=>		   self.ppu_memory.write(address as usize, data, false),
			0xFEA0..=0xFEFF	=> {},
			0xFF00			=>		{self.input_memory.write(data)}
			0xFF04..=0xFF07 =>		{self.timer_memory.write(address as usize, data)},
			0xFF40 | 0xFF47 =>		   self.ppu_memory.write(address as usize, data, false),
			0xFF41			=>		self.ppu_memory.write(address as usize, data, false),
			0xFF42			=>		  {self.ppu_memory.scy_ram = data},
			0xFF43			=>		  {self.ppu_memory.scx_ram = data},
			0xFF44			=>		  {},
			0xFF45			=>		  {self.ppu_memory.lyc_ram = data},
			0xFF46			=>		  {self.ppu_memory.oam_dma_reg = data; self.ppu_memory.oam_dma_is_active = true;},
			0xFF48 | 0xFF49 =>		self.ppu_memory.write(address as usize, data, false),
			0xFF4A			=>		  {self.ppu_memory.wy_ram = data},
			0xFF4B			=>		  {self.ppu_memory.wx_ram = data},
			0xFF4D			=>		{self.speed_chg_scheduled = (data & 0x01) != 0}
			0xFF4F			=>		  {self.vbk_reg = (data & 0x01) != 0}
			0xFF50			=>		  {self.bootrom_reg = data},
			0xFF51			=>		{self.ppu_memory.vram_dma_src_regs = (data as u16) << 8 | self.ppu_memory.vram_dma_src_regs & 0x00F0}
			0xFF52			=>		{self.ppu_memory.vram_dma_src_regs = self.ppu_memory.vram_dma_src_regs & 0xFF00 | (data & 0xF0) as u16}
			0xFF53			=>		{self.ppu_memory.vram_dma_dst_regs = ((data & 0x1F) as u16) << 8 | self.ppu_memory.vram_dma_dst_regs & 0x00F0}
			0xFF54			=>		{self.ppu_memory.vram_dma_dst_regs = self.ppu_memory.vram_dma_dst_regs & 0x1F00 | (data & 0xF0) as u16}
			0xFF55			=>		if self.ppu_memory.vram_dma_is_active && data & 0x80 == 0 {
										self.ppu_memory.vram_dma_is_active = false;
									} else {
										self.ppu_memory.vram_dma_is_hblank_mode = data & 0x80 != 0;
										self.ppu_memory.vram_dma_stat = data & 0x7F;
										self.ppu_memory.vram_dma_is_active = true;
									}
			0xFF68..=0xFF6B =>		self.ppu_memory.write(address as usize, data, false),
			0xFF70			=>		{if self.is_cgb {self.svbk_reg = data & 0x07; if self.svbk_reg == 0 {self.svbk_reg += 1}}}
			0xFF01..=0xFF7F	=>		  {self.io_regis[(address - 0xFF01) as usize] = data},
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