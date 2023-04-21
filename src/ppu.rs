use std::collections::HashMap;

use minifb::{Key, Window, WindowOptions};
use crate::memory_bus::MemoryBus;
use crate::memory_bus::video_ram::{TilePixel, PixelColour};

const TILE_WIDTH: usize = 0x08;
const TILE_HEIGHT: usize = 0x08;

const TILESET_NB_TILES_WIDTH: usize		= 0x10;
const TILESET_NB_TILES_HEIGHT: usize	= 0x08;
const TILESET_PX_WIDTH: usize	= TILE_WIDTH * TILESET_NB_TILES_WIDTH;
const TILESET_PX_HEIGHT: usize	= TILE_HEIGHT * TILESET_NB_TILES_HEIGHT;
const TILESET_PX_SIZE: usize	= TILESET_PX_WIDTH * TILESET_PX_HEIGHT;
const TILESETS_NB: usize = 3;
const TILESET_VIEWER_PX_WIDTH: usize	= TILESET_PX_WIDTH;
const TILESET_VIEWER_PX_HEIGHT: usize	= TILESET_PX_HEIGHT * TILESETS_NB;
const TILESET_VIEWER_PX_SIZE: usize		= TILESET_VIEWER_PX_WIDTH * TILESET_VIEWER_PX_HEIGHT;

const TILEMAP_NB_TILES_WIDTH: usize		= 0x20;
const TILEMAP_NB_TILES_HEIGHT: usize	= 0x20;
const TILEMAP_PX_WIDTH: usize	= TILE_WIDTH * TILEMAP_NB_TILES_WIDTH;
const TILEMAP_PX_HEIGHT: usize	= TILE_HEIGHT * TILEMAP_NB_TILES_HEIGHT;
const TILEMAP_PX_SIZE: usize	= TILEMAP_PX_WIDTH * TILEMAP_PX_HEIGHT;

const VIEWPORT_PX_WIDTH: usize	= 160;
const VIEWPORT_PX_HEIGHT: usize	= 144;
const VIEWPORT_PX_SIZE: usize	= VIEWPORT_PX_WIDTH * VIEWPORT_PX_HEIGHT;

pub struct Ppu {
	palette_translation: HashMap<PixelColour, u32>,

	tileset_viewer: Window,
	tileset_window_buf: Vec<u32>,
	tilemap_viewer: Window,
	tilemap_buf: Vec<u32>,
	viewport: Window,
	viewport_buffer: Vec<u32>
}

impl Ppu {
	pub fn new() -> Ppu {
		let ppu = Ppu {
			palette_translation: HashMap::from([(PixelColour::White, 0x00FFFFFF), (PixelColour::LightGray, 0x00A9A9A9), (PixelColour::DarkGray, 0x00545454), (PixelColour::Black, 0x00000000)]),
			tileset_viewer: Window::new("Tileset", TILESET_VIEWER_PX_WIDTH, TILESET_VIEWER_PX_HEIGHT, WindowOptions {scale: minifb::Scale::X4,..WindowOptions::default()}).unwrap(),
			tileset_window_buf: vec![0; TILESET_VIEWER_PX_SIZE],
			tilemap_viewer: Window::new("Tilemap", TILEMAP_PX_WIDTH, TILEMAP_PX_HEIGHT, WindowOptions {scale: minifb::Scale::X4,..WindowOptions::default()}).unwrap(),
			tilemap_buf: vec![0; TILEMAP_PX_SIZE],
			viewport: Window::new("GBMU", VIEWPORT_PX_WIDTH, VIEWPORT_PX_HEIGHT, WindowOptions {scale: minifb::Scale::X4,..WindowOptions::default()}).unwrap(),
			viewport_buffer: vec![0; VIEWPORT_PX_SIZE]
		};
		ppu
	}
	fn update_tileset_win(&mut self, memory_bus: &mut MemoryBus) {
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
									TilePixel::Zero => 0x00FFFFFF,
									TilePixel::One => 0x00A9A9A9,
									TilePixel::Two => 0x00545454,
									TilePixel::Three => 0x00000000,
								}
						};
					}
				}
			}
			self.tileset_viewer
				.update_with_buffer(&self.tileset_window_buf, TILESET_VIEWER_PX_WIDTH, TILESET_VIEWER_PX_HEIGHT)
				.unwrap();
		}
	}
	fn update_tilemap_buf(&mut self, memory_bus: &mut MemoryBus) {
		for y in 0..TILEMAP_NB_TILES_HEIGHT {
			for x in 0..TILEMAP_NB_TILES_WIDTH {
				let tile_index = memory_bus.video_ram.get_bg_tile_index(x as u8, y as u8);
				let tile = memory_bus.video_ram.get_bg_win_tile(tile_index);
				for (row_index, row) in tile.iter().enumerate() {
					for (pixel_index, pixel) in row.iter().enumerate() {
						self.tilemap_buf[
							y * TILEMAP_PX_WIDTH * TILE_HEIGHT +
							row_index * TILEMAP_PX_WIDTH +
							x * TILE_WIDTH +
							pixel_index
						] =	match pixel {
							TilePixel::Zero => self.palette_translation[&memory_bus.video_ram.background_palette[0]],
							TilePixel::One => self.palette_translation[&memory_bus.video_ram.background_palette[1]],
							TilePixel::Two => self.palette_translation[&memory_bus.video_ram.background_palette[2]],
							TilePixel::Three => self.palette_translation[&memory_bus.video_ram.background_palette[3]],
						}
					}
				}
			}
		}
	}
	fn update_viewport(&mut self, memory_bus: &mut MemoryBus) {
		for y in 0..VIEWPORT_PX_HEIGHT {
			if !memory_bus.video_ram.lcd_enable {break};
			memory_bus.video_ram.ly_ram = y as u8;
			for x in 0..VIEWPORT_PX_WIDTH {
				if memory_bus.video_ram.bg_win_enable {
					self.viewport_buffer[y * VIEWPORT_PX_WIDTH + x] = self.tilemap_buf[
						((memory_bus.video_ram.scy_ram as usize + y) % TILEMAP_PX_HEIGHT) * TILEMAP_PX_WIDTH +
						(memory_bus.video_ram.scx_ram as usize + x) % TILEMAP_PX_WIDTH
					]
				} else {
					self.viewport_buffer[y * VIEWPORT_PX_WIDTH + x] = self.palette_translation[&PixelColour::White];
				}
			}
		}
		memory_bus.video_ram.ly_ram = VIEWPORT_PX_HEIGHT as u8;
		self.viewport
			.update_with_buffer(&self.viewport_buffer, VIEWPORT_PX_WIDTH, VIEWPORT_PX_HEIGHT)
			.unwrap();
	}
	fn update_tilemap_win(&mut self, memory_bus: &mut MemoryBus) {
			for x in 0..VIEWPORT_PX_WIDTH {
				self.tilemap_buf[memory_bus.video_ram.scy_ram as usize * TILEMAP_PX_WIDTH + (memory_bus.video_ram.scx_ram as usize + x) % TILEMAP_PX_WIDTH] = 0x00FF0000;
				self.tilemap_buf[((memory_bus.video_ram.scy_ram as usize + VIEWPORT_PX_HEIGHT - 1) % TILEMAP_PX_HEIGHT) * TILEMAP_PX_WIDTH + (memory_bus.video_ram.scx_ram as usize + x) % TILEMAP_PX_WIDTH] = 0x00FF0000;
			}
			for y in 0..VIEWPORT_PX_HEIGHT {
				self.tilemap_buf[((memory_bus.video_ram.scy_ram as usize + y) % TILEMAP_PX_HEIGHT) * TILEMAP_PX_WIDTH + memory_bus.video_ram.scx_ram as usize] = 0x00FF0000;
				self.tilemap_buf[((memory_bus.video_ram.scy_ram as usize + y) % TILEMAP_PX_HEIGHT) * TILEMAP_PX_WIDTH + (memory_bus.video_ram.scx_ram as usize + VIEWPORT_PX_WIDTH) % TILEMAP_PX_WIDTH - 1] = 0x00FF0000;
			}
			self.tilemap_viewer
				.update_with_buffer(&self.tilemap_buf, TILEMAP_PX_WIDTH, TILEMAP_PX_HEIGHT)
				.unwrap();
	}
	pub fn update(&mut self, memory_bus: &mut MemoryBus) {
		if self.tileset_viewer.is_open() && !self.tileset_viewer.is_key_down(Key::Escape) {
			self.update_tileset_win(memory_bus);
		}
		self.update_tilemap_buf(memory_bus);
		self.update_viewport(memory_bus);
		if self.tilemap_viewer.is_open() && !self.tilemap_viewer.is_key_down(Key::Escape) {
			self.update_tilemap_win(memory_bus);
		}
	}
}

#[cfg(test)]
mod tests {
    use crate::memory_bus::MemoryBus;

    use super::*;

	#[test]
	fn test_tileset_fill() {
		let mut memory_bus = MemoryBus::new();
		let mut ppu = Ppu::new();
		std::thread::sleep(std::time::Duration::from_millis(500));					// or first minifb update will fail??
		ppu.update(&mut memory_bus);
		std::thread::sleep(std::time::Duration::from_millis(1000));
		for i in 0..0x6000 {
			let tiles_bank =	(i / TILE_WIDTH / TILE_HEIGHT / TILESET_NB_TILES_HEIGHT / TILESET_NB_TILES_WIDTH ) % TILESETS_NB;
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
	fn test_tileset_direct() {
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
			ppu.update(&mut memory_bus);						// Should display an ® logo in tile {1; 9}
		}
	}
	#[test]
	fn test_tileset_generation() {
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
			ppu.update(&mut memory_bus);						// Should display an ® logo in tile {1; 9}
		}
	}
	#[test]
	fn test_tilemap_and_viewport_composition() {
		let mut memory_bus = MemoryBus::new();
		let mut ppu = Ppu::new();
		let mut cpu = crate::Cpu::new();
		memory_bus.load_dmg_bootrom();
		memory_bus.debug_insert_cart_logo();
		cpu.tick(&mut memory_bus);
		while cpu.registers.program_counter - 1 != 0x55 {
			cpu.tick(&mut memory_bus);
		}
		memory_bus.video_ram.bg_win_enable = true;
		memory_bus.video_ram.using_fully_common_bg_tileset = true;
		memory_bus.video_ram.lcd_enable = true;
		memory_bus.video_ram.bg_tilemap0 = [
			[0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19],
			[0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00],
			[0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00],
			[0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00],
			[0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00],
			[0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00],
			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
			[0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00],
			[0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00],
			[0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00],
			[0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00],
			[0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00],
			[0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19],
		];
		loop {
			ppu.update(&mut memory_bus);
			memory_bus.write_byte(0xFF42, memory_bus.read_byte(0xFF42).overflowing_add(1).0);
			memory_bus.write_byte(0xFF43, memory_bus.read_byte(0xFF43).overflowing_add(1).0);
		}
	}
}