#[derive(Debug, Copy, Clone)]
enum TilePixel {
    Zero,
    One,
    Two,
    Three,
}
type TileRow = [TilePixel; 8];
type Tile = [TileRow; 8];
fn empty_tile() -> Tile {
    [[TilePixel::Zero; 8]; 8]
}

pub struct VideoRam {
	vram: [u8; 0x2000],
	tiles: [[Tile; 0x80]; 3],
	bg_tilemap1: [u8; 0x400],
	bg_tilemap2: [u8; 0x400],
}

impl VideoRam {
	pub fn new() -> Self {
		VideoRam {
			vram: [0; 0x2000],
			tiles: [[empty_tile(); 0x80]; 3],
			bg_tilemap1: [0; 0x400],
			bg_tilemap2: [0; 0x400],
		}
	}

	pub fn read(&self, address: usize) -> u8 {
		self.vram[address]
	}

	pub fn write(&mut self, address: usize, data: u8) {
		self.vram[address] = data;
		if address < 0x1800 {
			self.write_tile(address & 0xFFFE)
		}
		else {
			if address < 0x1C00 {
				self.bg_tilemap1[address - 0x1800] = data
			} else {
				self.bg_tilemap2[address - 0x1C00] = data
			}
		}
	}
	pub fn write_tile(&mut self, floored_even_addr: usize) {
		let tile_reg = floored_even_addr / 0x800;
		let tile_index = floored_even_addr / 16;
		let row_index = floored_even_addr % 16;
		for pixel_index in 0..8{
			let msb = self.vram[floored_even_addr + 1] & (7 - pixel_index);
			let lsb = self.vram[floored_even_addr] & (7 - pixel_index);
			let value = match (msb != 0, lsb != 0) {
				(false, false) => TilePixel::Zero,
				(false, true) => TilePixel::One,
				(true, false) => TilePixel::Two,
				(true, true) => TilePixel::Three
			};
			self.tiles[tile_reg][tile_index][row_index][pixel_index as usize] = value;
		}
	}
}