#[derive(Debug, Copy, Clone)]
pub enum TilePixel {
    Zero,
    One,
    Two,
    Three,
}
pub type TileRow = [TilePixel; 8];
pub type Tile = [TileRow; 8];
pub struct VideoRam {
	pub is_locked: bool,
	video_ram: [u8; 0x2000],
	pub tiles: [[Tile; 0x80]; 3],			// 0x8000 - 0x97FF
	pub bg_tilemap0: [[u8; 0x20]; 0x20],	// 0x9800 - 0x9BFF
	pub bg_tilemap1: [[u8; 0x20]; 0x20],	// 0x9C00 - 0x9FFF
}

impl VideoRam {
	pub fn new() -> Self {
		VideoRam {
			is_locked: false,
			video_ram: [0; 0x2000],
			tiles: [ [[[TilePixel::Zero;8];8];0x80]; 3],
			bg_tilemap0: [[0; 0x20]; 0x20],
			bg_tilemap1: [[0; 0x20]; 0x20],
		}
	}
	fn write_tile(&mut self, floored_even_addr: usize) {
		let tile_reg = floored_even_addr / 0x800;
		let tile_index = (floored_even_addr - tile_reg * 0x800) / 16;
		let row_index = (floored_even_addr % 16) / 2;
		for pixel_index in 0..8{
			let msb = self.video_ram[floored_even_addr + 1] & (7 - pixel_index);
			let lsb = self.video_ram[floored_even_addr] & (7 - pixel_index);
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
	}
	pub fn read(&self, address: usize) -> u8 {
		self.video_ram[address]
	}
	pub fn get_bg_tile_index(&self, x: u8, y: u8, from_secondary_tilemap: bool) -> u8{
		if from_secondary_tilemap {
			self.bg_tilemap1[x as usize][y as usize]
		} else {
			self.bg_tilemap0[x as usize][y as usize]
		}
		
	}
	pub fn get_bg_tile(&self, mut tile_index: u8, using_8800_addressing: bool) -> Tile {
		let tile_reg = if tile_index >= 128 {
			tile_index -= 128;
			1
		} else { if using_8800_addressing {2} else {0} };
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