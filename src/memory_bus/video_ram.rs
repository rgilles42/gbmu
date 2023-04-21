#[derive(Debug, Copy, Clone)]
pub enum TilePixel {
    Zero,
    One,
    Two,
    Three,
}
pub type TileRow = [TilePixel; 8];
pub type Tile = [TileRow; 8];
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PixelColour {
    White,
    LightGray,
    DarkGray,
    Black,
}
pub struct VideoRam {
	pub is_vram_locked: bool,
	pub is_oam_locked: bool,

	video_ram: [u8; 0x2000],
	oam: [u8; 0x00A0],
	lcdc_ram: u8,							// 0xFF40
	pub scy_ram: u8,						// 0xFF42
	pub scx_ram: u8,						// 0xFF43
	pub ly_ram: u8,							// 0xFF44
	bgp_ram: u8,							// 0xFF47

	pub tiles: [[Tile; 0x80]; 3],			// 0x8000 - 0x97FF
	pub bg_tilemap0: [[u8; 0x20]; 0x20],	// 0x9800 - 0x9BFF
	pub bg_tilemap1: [[u8; 0x20]; 0x20],	// 0x9C00 - 0x9FFF
	pub lcd_enable: bool,					// 0xFF40 & (1 << 7)
	pub win_using_secondary_tilemap: bool,	// 0xFF40 & (1 << 6)
	// pub win_enable: bool,					// 0xFF40 & (1 << 5)
	pub using_fully_common_bg_tileset: bool,// 0xFF40 & (1 << 4)
	pub bg_using_secondary_tilemap: bool,	// 0xFF40 & (1 << 3)
	// pub double_heigth_obj: bool,			// 0xFF40 & (1 << 2)
	// pub obj_enable: bool,					// 0xFF40 & (1 << 1)
	pub bg_win_enable: bool,				// 0xFF40 & (1 << 0)
	pub background_palette: [PixelColour; 4]// 0xFF47
}

impl VideoRam {
	pub fn new() -> Self {
		VideoRam {
			is_vram_locked: false,
			is_oam_locked: false,
			video_ram: [0; 0x2000],
			oam: [0; 0xA0],
			lcdc_ram: 0,
			bgp_ram: 0,
			scx_ram: 0,
			scy_ram: 0,
			ly_ram: 153,
			tiles: [ [[[TilePixel::Zero;8];8];0x80]; 3],
			bg_tilemap0: [[0; 0x20]; 0x20],
			bg_tilemap1: [[0; 0x20]; 0x20],
			lcd_enable: false,
			win_using_secondary_tilemap: false,
			// win_enable: false,
			using_fully_common_bg_tileset: false,
			bg_using_secondary_tilemap: false,
			// double_heigth_obj: false,
			// obj_enable: false,
			bg_win_enable: false,
			background_palette: [PixelColour::White, PixelColour::LightGray, PixelColour::DarkGray, PixelColour::Black]
		}
	}
	fn write_tile(&mut self, floored_even_addr: usize) {
		let tile_reg = floored_even_addr / 0x800;
		let tile_index = (floored_even_addr % 0x800) / 16;
		let row_index = (floored_even_addr % 16) / 2;
		for pixel_index in 0..8{
			let msb = self.video_ram[floored_even_addr + 1] & (1 << (7 - pixel_index));
			let lsb = self.video_ram[floored_even_addr] & (1 << (7 - pixel_index));
			let value = match (msb != 0, lsb != 0) {
				(false, false) => TilePixel::Zero,
				(false, true) => TilePixel::One,
				(true, false) => TilePixel::Two,
				(true, true) => TilePixel::Three
			};
			self.tiles[tile_reg][tile_index][row_index][pixel_index as usize] = value;
		}
	}
	pub fn write(&mut self, address: usize, data: u8) {
		if address < 0xA000 {
			if self.is_vram_locked {return}
			let address = address - 0x8000;
			self.video_ram[address] = data;
			if address < 0x1800 {
				self.write_tile(address & 0xFFFE)
			}
			else {
				if address < 0x1C00 {
					self.bg_tilemap0[(address - 0x1800) / 0x20][(address - 0x1800) % 0x20] = data
				} else {
					self.bg_tilemap1[(address - 0x1C00) / 0x20][(address - 0x1C00) % 0x20] = data
				}
			}
		} else if address < 0xFEA0 {
			if self.is_oam_locked {return}
			let address = address - 0xFE00;
			self.oam[address] = data;
			// OAM formatting

		} else if address == 0xFF40 {
			self.lcdc_ram = data;
			self.lcd_enable						= (data & (1 << 7)) != 0;
			self.win_using_secondary_tilemap	= (data & (1 << 6)) != 0;
			// self.win_enable						= (data & (1 << 5)) != 0;
			self.using_fully_common_bg_tileset	= (data & (1 << 4)) != 0;
			self.bg_using_secondary_tilemap		= (data & (1 << 3)) != 0;
			// self.double_heigth_obj				= (data & (1 << 2)) != 0;
			// self.obj_enable						= (data & (1 << 1)) != 0;
			self.bg_win_enable					= (data & 1) != 0;
		}
		else {										// 0xFF47
			self.bgp_ram = data;
			for i in 0..4 {
				let colour_code = (data >> 2*i) & 0x03;
				self.background_palette[i] = match colour_code {
					0 => PixelColour::White,
					1 => PixelColour::LightGray,
					2 => PixelColour::DarkGray,
					_ => PixelColour::Black
				}
			}
		}
	}
	pub fn read(&self, address: usize) -> u8 {
		if address < 0xA000 {
			if self.is_vram_locked {
				0xFF
			} else {
				let address = address - 0x8000;
				self.video_ram[address]
			}
		} else if address < 0xFEA0 {
			if self.is_oam_locked {
				0xFF
			} else {
				let address = address - 0xFE00;
				self.oam[address]
			}
		} else if address == 0xFF40 {
			self.lcdc_ram
		} else {							// 0xFF47
			self.bgp_ram
		}
	}
	pub fn get_bg_tile_index(&self, x: u8, y: u8) -> u8{
		if self.bg_using_secondary_tilemap {
			self.bg_tilemap1[y as usize][x as usize]
		} else {
			self.bg_tilemap0[y as usize][x as usize]
		}
		
	}
	// pub fn get_win_tile_index(&self, x: u8, y: u8) -> u8{
	// 	if self.win_using_secondary_tilemap {
	// 		self.bg_tilemap1[y as usize][x as usize]
	// 	} else {
	// 		self.bg_tilemap0[y as usize][x as usize]
	// 	}
		
	// }
	pub fn get_bg_win_tile(&self, mut tile_index: u8) -> Tile {
		let tile_reg = if tile_index >= 128 {
			tile_index -= 128;
			1
		} else { if self.using_fully_common_bg_tileset {0} else {2} };
		self.tiles[tile_reg][tile_index as usize]
	}
	// pub fn get_obj_tile(&self, mut tile_index: u8) -> Tile {
	// 	let tile_reg = if tile_index >= 128 {tile_index -= 128; 1} else {0};
	// 	self.tiles[tile_reg][tile_index as usize]
	// }
	// pub fn get_large_obj_tiles(&self, tile_index: u8) -> (Tile, Tile) {
	// 	let mut tile_index = tile_index & 0xFE;
	// 	let tile_reg = if tile_index >= 128 {tile_index -= 128; 1} else {0};
	// 	(self.tiles[tile_reg][tile_index as usize], self.tiles[tile_reg][tile_index as usize + 1])
	// }
}