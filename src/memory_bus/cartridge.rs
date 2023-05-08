use std::fs;
#[derive(Debug, Clone, Copy)]
pub enum MapperType {
	None, MBC1, MBC2, MBC3, MBC5
}

#[derive(Debug, Clone, Copy)]
pub enum ROMType {
	X2_32KiB, X4_64KiB, X8_128KiB, X16_256KiB, X32_512KiB, X64_1MiB, X128_2MiB, X256_4MiB, X512_8MiB
}

#[derive(Debug, Clone, Copy)]
pub enum RAMType {
	None, X1_8KiB, X4_32KiB, X8_64KiB, X16_128KiB
}

pub struct Cartridge {
	mapper_type: MapperType,
	rom_type: ROMType,
	rom_banks: Vec<[u8; 0x4000]>,
	current_1st_rom_bank: usize,
	current_2d_rom_bank: usize,
	ram_type: RAMType,
	ram_banks: Vec<[u8; 0x2000]>,
	current_ram_bank: usize
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
				mapper_type: MapperType::None,
				rom_type: ROMType::X2_32KiB,
				rom_banks: vec![[0xFF; 0x4000]; 2],
				current_1st_rom_bank: 0,
				current_2d_rom_bank: 1,
				ram_type: RAMType::None,
				ram_banks: Vec::new(),
				current_ram_bank: 0
			}
		}
	}
	fn load_from_path(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
		let rom_contents = fs::read(path)?;
		let mapper_type = match rom_contents[0x147] {
			0x01..=0x03 =>	MapperType::MBC1,
			0x05 | 0x06 =>	MapperType::MBC2,
			0x0F..=0x13 =>	MapperType::MBC3,
			0x19..=0x1E => MapperType::MBC5,
			_ => MapperType::None
		};
		let rom_type = match rom_contents[0x148] {
			0x01 => ROMType::X4_64KiB,
			0x02 => ROMType::X8_128KiB,
			0x03 => ROMType::X16_256KiB,
			0x04 => ROMType::X32_512KiB,
			0x05 => ROMType::X64_1MiB,
			0x06 => ROMType::X128_2MiB,
			0x07 => ROMType::X256_4MiB,
			0x08 => ROMType::X512_8MiB,
			_ 	=> ROMType::X2_32KiB
		};
		let ram_type = match rom_contents[0x149] {
			0x02 => RAMType::X1_8KiB,
			0x03 => RAMType::X4_32KiB,
			0x04 => RAMType::X16_128KiB,
			0x05 => RAMType::X8_64KiB,
			_ => RAMType::None
		};
		let mut rom_banks = vec![[0xFF; 0x4000]; match rom_type {
				ROMType::X2_32KiB =>	0x02,
				ROMType::X4_64KiB =>	0x04,
				ROMType::X8_128KiB =>	0x08,
				ROMType::X16_256KiB =>	0x10,
				ROMType::X32_512KiB =>	0x20,
				ROMType::X64_1MiB =>	0x40,
				ROMType::X128_2MiB =>	0x80,
				ROMType::X256_4MiB =>	0x100,
				ROMType::X512_8MiB =>	0x200,
			}
		];
		for (i, byte) in rom_contents.iter().enumerate() {
			rom_banks[i / 0x4000][i % 0x4000] = *byte;
		}
		Ok(Cartridge {
			mapper_type,
			rom_type,
			rom_banks,
			current_1st_rom_bank: 0x00,
			current_2d_rom_bank: 0x01,
			ram_type,
			ram_banks: vec![[0x00; 0x2000]; match ram_type {
				RAMType::None => 0,
				RAMType::X1_8KiB => 1,
				RAMType::X4_32KiB => 4,
				RAMType::X8_64KiB => 8,
				RAMType::X16_128KiB => 16,
				}
			],
			current_ram_bank: 0x00,
		})
	}
	pub fn read(&self, address: usize) -> u8 {
		match address {
			0x0000..=0x3FFF	=> self.rom_banks[self.current_1st_rom_bank][address],
			0x4000..=0x7FFF	=> self.rom_banks[self.current_2d_rom_bank][address - 0x4000],
			0xA000..=0xBFFF	=> if let RAMType::None = self.ram_type {0xFF}
								else {self.ram_banks[self.current_ram_bank][(address - 0xA000)]},
			_ => 0
		}
	}
	pub fn write(&mut self, address: usize, data: u8) {
		match address {
			0x0000..=0x7FFF	=> {println!("Oops, tried to write {} at 0x{:X}", data, address)},
			0xA000..=0xBFFF	=> if let RAMType::None = self.ram_type {}
								else {self.ram_banks[self.current_ram_bank][(address - 0xA000)] = data},
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
			self.rom_banks[0][0x104 + i] = *byte;
		}
		self.rom_banks[0][0x134] = 0xE7;
		for i in 0x135..0x14E {
			self.rom_banks[0][i] = 0x00;
		}
	}
}