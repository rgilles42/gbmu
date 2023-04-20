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
	pub is_locked: bool,
	video_ram: [u8; 0x2000],
	lcdc_ram: u8,							// 0xFF40
	pub scy_ram: u8,						// 0xFF42
	pub scx_ram: u8,						// 0xFF43
	bgp_ram: u8,							// 0xFF47
	pub tiles: [[Tile; 0x80]; 3],			// 0x8000 - 0x97FF
	pub bg_tilemap0: [[u8; 0x20]; 0x20],	// 0x9800 - 0x9BFF
	pub bg_tilemap1: [[u8; 0x20]; 0x20],	// 0x9C00 - 0x9FFF
	pub using_fully_common_bg_tileset: bool,// 0xFF40 & (1 << 4)
	pub using_secondary_tilemap: bool,		// 0xFF40 & (1 << 5)
	pub background_palette: [PixelColour; 4]// 0xFF47
}

impl VideoRam {
	pub fn new() -> Self {
		VideoRam {
			is_locked: false,
			video_ram: [0; 0x2000],
			lcdc_ram: 0,
			bgp_ram: 0,
			scx_ram: 0,
			scy_ram: 0,
			tiles: [ [[[TilePixel::Zero;8];8];0x80]; 3],
			bg_tilemap0: [[0; 0x20]; 0x20],
			bg_tilemap1: [[0; 0x20]; 0x20],
			using_fully_common_bg_tileset: true,
			using_secondary_tilemap: false,
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
		if address < 0x2000 {
			if self.is_locked {return}
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
		} else if address == 0xFF40 {
			self.lcdc_ram = data;
			self.using_fully_common_bg_tileset	= data & (1 << 4) != 0;
			self.using_secondary_tilemap		= data & (1 << 5) != 0;
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
		if address < 0x2000 {
			self.video_ram[address]
		}
		else if address == 0xFF40 {
			self.lcdc_ram
		}
		else {							// 0xFF47
			self.bgp_ram
		}
	}
	pub fn get_bg_tile_index(&self, x: u8, y: u8) -> u8{
		if self.using_secondary_tilemap {
			self.bg_tilemap1[y as usize][x as usize]
		} else {
			self.bg_tilemap0[y as usize][x as usize]
		}
		
	}
	pub fn get_bg_tile(&self, mut tile_index: u8) -> Tile {
		let tile_reg = if tile_index >= 128 {
			tile_index -= 128;
			1
		} else { if self.using_fully_common_bg_tileset {0} else {2} };
		self.tiles[tile_reg][tile_index as usize]
	}
	pub fn get_obj_tile(&self, mut tile_index: u8) -> Tile {
		let tile_reg = if tile_index >= 128 {tile_index -= 128; 1} else {0};
		self.tiles[tile_reg][tile_index as usize]
	}
	pub fn get_large_obj_tiles(&self, tile_index: u8) -> (Tile, Tile) {
		let mut tile_index = tile_index & 0xFE;
		let tile_reg = if tile_index >= 128 {tile_index -= 128; 1} else {0};
		(self.tiles[tile_reg][tile_index as usize], self.tiles[tile_reg][tile_index as usize + 1])
	}
}