use std::cell::RefMut;

use minifb::{Key, Window, WindowOptions};
use crate::memory_bus::MemoryBus;
use crate::memory_bus::video_ram::TilePixel;

const TILE_WIDTH: usize = 0x08;
const TILE_HEIGHT: usize = 0x08;
const TILE_SIZE: usize	= TILE_WIDTH * TILE_HEIGHT;

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
		let mut ppu = Ppu {
			tileset_viewer: Window::new("Tileset", TILESET_VIEWER_PX_WIDTH, TILESET_VIEWER_PX_HEIGHT, WindowOptions::default()).unwrap(),
			tileset_window_buf: vec![0; TILESET_VIEWER_PX_SIZE]
		};
		ppu.tileset_viewer.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
		ppu
	}
	pub fn update(&mut self, memory_bus: &mut MemoryBus) {
		if self.tileset_viewer.is_open() && !self.tileset_viewer.is_key_down(Key::Escape) {
			for (i, pixel) in self.tileset_window_buf.iter_mut().enumerate() {
				let tiles_bank =	 i / TILESET_PX_SIZE;
				let tile_index =	(i - tiles_bank * TILESET_PX_SIZE) / TILE_SIZE;
				let tile_row =	(i - tiles_bank * TILESET_PX_SIZE - tile_index * TILE_SIZE) / TILE_WIDTH;
				let tile_pixel =	 i - tiles_bank * TILESET_PX_SIZE - tile_index * TILE_SIZE - tile_row * TILE_WIDTH;
				*pixel = match memory_bus.video_ram.tiles[tiles_bank][tile_index][tile_row][tile_pixel] {
					TilePixel::Zero => {0x00FFFFFF}
					TilePixel::One => {0x00A9A9A9}
					TilePixel::Two => {0x00545454}
					TilePixel::Three => {0x00000000}
				}
			}
			self.tileset_viewer
				.update_with_buffer(&self.tileset_window_buf, TILESET_VIEWER_PX_WIDTH, TILESET_VIEWER_PX_HEIGHT)
				.unwrap();
		}
	}
}