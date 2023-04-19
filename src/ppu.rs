use minifb::{Key, Window, WindowOptions};
use crate::memory_bus::MemoryBus;
use crate::memory_bus::video_ram::TilePixel;

const TILE_WIDTH: usize = 0x08;
const TILE_HEIGHT: usize = 0x08;

const TILESET_NB_TILES_WIDTH: usize		= 0x10;
const TILESET_NB_TILES_HEIGHT: usize	= 0x08;

const TILESET_PX_WIDTH: usize	= TILE_WIDTH * TILESET_NB_TILES_WIDTH;
const TILESET_PX_HEIGHT: usize	= TILE_HEIGHT * TILESET_NB_TILES_HEIGHT;
const TILESET_PX_SIZE: usize	= TILESET_PX_WIDTH * TILESET_PX_HEIGHT;

const NB_TILESETS: usize = 3;
const TILESET_VIEWER_PX_WIDTH: usize	= TILESET_PX_WIDTH;
const TILESET_VIEWER_PX_HEIGHT: usize	= TILESET_PX_HEIGHT * NB_TILESETS;
const TILESET_VIEWER_PX_SIZE: usize		= TILESET_VIEWER_PX_WIDTH * TILESET_VIEWER_PX_HEIGHT;

pub struct Ppu {
	tileset_viewer: Window,
	tileset_window_buf: Vec<u32>,
}

impl Ppu {
	pub fn new() -> Ppu {
		let ppu = Ppu {
			tileset_viewer: Window::new("Tileset", TILESET_VIEWER_PX_WIDTH, TILESET_VIEWER_PX_HEIGHT, WindowOptions {scale: minifb::Scale::X4,..WindowOptions::default()}).unwrap(),
			tileset_window_buf: vec![0; TILESET_VIEWER_PX_SIZE]
		};
		ppu
	}
	pub fn update(&mut self, memory_bus: &mut MemoryBus) {
		if self.tileset_viewer.is_open() && !self.tileset_viewer.is_key_down(Key::Escape) {
			for (id_bank, bank) in memory_bus.video_ram.tiles.iter().enumerate() {
				for (id_tile, tile) in bank.iter().enumerate() {
					for (id_row, row) in tile.iter().enumerate() {
						for (pixel_id, pixel) in row.iter().enumerate() {
							self.tileset_window_buf[
								id_bank * TILESET_PX_SIZE +
								(id_tile / TILESET_NB_TILES_WIDTH) * (TILESET_PX_WIDTH * TILE_HEIGHT) +
								id_row * TILESET_PX_WIDTH +
								(id_tile % TILESET_NB_TILES_WIDTH) * TILE_WIDTH +
								pixel_id
							] =
							if id_row == 0 || pixel_id == 0 {0x000000FF}
							else {
									match pixel {
										TilePixel::Zero => {0x00FFFFFF}
										TilePixel::One => {0x00A9A9A9}
										TilePixel::Two => {0x00545454}
										TilePixel::Three => {0x00000000}
									}
							};
						}
					}
				}
			}
			self.tileset_viewer
				.update_with_buffer(&self.tileset_window_buf, TILESET_VIEWER_PX_WIDTH, TILESET_VIEWER_PX_HEIGHT)
				.unwrap();
		}
	}
}

#[cfg(test)]
mod tests {
    use crate::memory_bus::MemoryBus;

    use super::*;

	#[test]
	fn test_ppu() {
		let mut memory_bus = MemoryBus::new();
		let mut ppu = Ppu::new();
		std::thread::sleep(std::time::Duration::from_millis(500));					// or first minifb update will fail??
		ppu.update(&mut memory_bus);
		std::thread::sleep(std::time::Duration::from_millis(1000));
		for i in 0..0x6000 {
			let tiles_bank =	(i / TILE_WIDTH / TILE_HEIGHT / TILESET_NB_TILES_HEIGHT / TILESET_NB_TILES_WIDTH ) % NB_TILESETS;
			let tile_index =	(i / TILE_WIDTH / TILE_HEIGHT) % (TILESET_NB_TILES_HEIGHT * TILESET_NB_TILES_WIDTH);
			let tile_row =	(i / TILE_WIDTH) % TILE_HEIGHT ;
			let tile_pixel =	 i % TILE_WIDTH;
			memory_bus.video_ram.tiles[tiles_bank][tile_index][tile_row][tile_pixel] = match tiles_bank {
				0 => TilePixel::One,
				1 => TilePixel::Two,
				2 => TilePixel::Three,
				_ => TilePixel::Zero
			};
			ppu.update(&mut memory_bus);
		}
	}
	#[test]
	fn test_ppu2() {
		let mut memory_bus = MemoryBus::new();
		let mut ppu = Ppu::new();
		memory_bus.video_ram.tiles[0][0x19][0] = [TilePixel::Zero, TilePixel::Zero, TilePixel::One, TilePixel::One, TilePixel::One, TilePixel::One, TilePixel::Zero, TilePixel::Zero];
		memory_bus.video_ram.tiles[0][0x19][1] = [TilePixel::Zero, TilePixel::One, TilePixel::Zero, TilePixel::Zero, TilePixel::Zero, TilePixel::Zero, TilePixel::One, TilePixel::Zero];
		memory_bus.video_ram.tiles[0][0x19][2] = [TilePixel::One, TilePixel::Zero, TilePixel::One, TilePixel::One, TilePixel::One, TilePixel::Zero, TilePixel::Zero, TilePixel::One];
		memory_bus.video_ram.tiles[0][0x19][3] = [TilePixel::One, TilePixel::Zero, TilePixel::One, TilePixel::Zero, TilePixel::Zero, TilePixel::One, TilePixel::Zero, TilePixel::One];
		memory_bus.video_ram.tiles[0][0x19][4] = [TilePixel::One, TilePixel::Zero, TilePixel::One, TilePixel::One, TilePixel::One, TilePixel::Zero, TilePixel::Zero, TilePixel::One];
		memory_bus.video_ram.tiles[0][0x19][5] = [TilePixel::One, TilePixel::Zero, TilePixel::One, TilePixel::Zero, TilePixel::Zero, TilePixel::One, TilePixel::Zero, TilePixel::One];
		memory_bus.video_ram.tiles[0][0x19][6] = [TilePixel::Zero, TilePixel::One, TilePixel::Zero, TilePixel::Zero, TilePixel::Zero, TilePixel::Zero, TilePixel::One, TilePixel::Zero];
		memory_bus.video_ram.tiles[0][0x19][7] = [TilePixel::Zero, TilePixel::Zero, TilePixel::One, TilePixel::One, TilePixel::One, TilePixel::One, TilePixel::Zero, TilePixel::Zero];
		loop {
			ppu.update(&mut memory_bus);
		}
	}
	#[test]
	fn test_ppu3() {
		let mut memory_bus = MemoryBus::new();
		let mut ppu = Ppu::new();
		memory_bus.video_ram.write(0x190, 0x3c);
		memory_bus.video_ram.write(0x192, 0x42);
		memory_bus.video_ram.write(0x194, 0xb9);
		memory_bus.video_ram.write(0x196, 0xa5);
		memory_bus.video_ram.write(0x198, 0xb9);
		memory_bus.video_ram.write(0x19a, 0xa5);
		memory_bus.video_ram.write(0x19c, 0x42);
		memory_bus.video_ram.write(0x19e, 0x3c);
		loop {
			ppu.update(&mut memory_bus);
		}
	}
}