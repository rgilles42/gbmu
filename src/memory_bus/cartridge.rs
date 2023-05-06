use std::fs;
#[derive(Debug, Clone, Copy)]
pub enum MapperType {
	None, 
}

#[derive(Debug, Clone, Copy)]
pub enum ROMType {
	Two32KiB
}

#[derive(Debug, Clone, Copy)]
pub enum RAMType {
	None, One8KiB, Four32KiB, Eight64KiB, Sixteen128KiB
}

pub struct Cartridge {
	rom_contents: Vec<u8>,
	mapper_type: MapperType,
	rom_type: ROMType,
	ram_type: RAMType,
	cartr_ram: [u8; 0x2000]
}

impl Cartridge {
	pub fn new(rom_path: Option<&str>) -> Self {
		if let Some(path) = rom_path {
			if let Ok(cart) = Cartridge::load_from_path(path) {
				cart
			} else {
				panic!("Unable to open ROM file at {}", path)
			}
		} else {
			Cartridge {
					rom_contents: vec![0xFF; 0x8000],
					mapper_type: MapperType::None,
					rom_type: ROMType::Two32KiB,
					ram_type: RAMType::None,
					cartr_ram: [0x00; 0x2000]
			}
		}
	}
	fn load_from_path(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
		let rom_contents = fs::read(path)?;
		let mapper_type = match rom_contents[0x147] {
			_ => MapperType::None
		};
		let rom_type = match rom_contents[0x148] {
			_ => ROMType::Two32KiB
		};
		let ram_type = match rom_contents[0x149] {
			_ => RAMType::None
		};
		Ok(Cartridge {
			rom_contents,
			mapper_type,
			rom_type,
			ram_type,
			cartr_ram: [0x00; 0x2000]
		})
	}
	pub fn read(&self, address: usize) -> u8 {
		match address {
			0x0000..=0x7FFF	=> self.rom_contents[address],
			0xA000..=0xBFFF	=> if let RAMType::None = self.ram_type {0xFF}
								else {self.cartr_ram[(address - 0xA000)]},
			_ => 0
		}
	}
	pub fn write(&mut self, address: usize, data: u8) {
		match address {
			0x0000..=0x7FFF	=> {println!("Oops, tried to write {} at 0x{:X}", data, address)},
			0xA000..=0xBFFF	=> if let RAMType::None = self.ram_type {}
								else {self.cartr_ram[(address - 0xA000)] = data},
			_ => {}
		}
	}
	pub fn debug_insert_cart_logo(&mut self) {
		let logo_data : [u8; 48] = [
			0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 
			0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 
			0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 
			0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E
		];
		for (i, byte) in logo_data.iter().enumerate() {
			self.rom_contents[0x104 + i] = *byte;
		}
		self.rom_contents[0x134] = 0xE7;
		for i in 0x135..0x14E {
			self.rom_contents[i] = 0x00;
		}
	}
}