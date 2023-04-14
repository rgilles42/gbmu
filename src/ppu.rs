use minifb::{Key, Window, WindowOptions};
use crate::cpu::memory_bus::MemoryBus;

const TILESET_VIEWER_HEIGHT: usize = 0x08;
const TILESET_VIEWER_WIDTH: usize = 0x10;

pub struct Ppu<'a> {
	memory_bus: &'a mut MemoryBus,
	tileset_viewer: Window,
	tileset_window_buf: Vec<u32>,
}

impl Ppu<'_> {
	pub fn new(memory_bus: &'_ mut MemoryBus) -> Ppu<'_> {
		Ppu {
			memory_bus,
			tileset_viewer: Window::new("Tileset", TILESET_VIEWER_WIDTH * 8, TILESET_VIEWER_HEIGHT * 8, WindowOptions::default()).unwrap(),
			tileset_window_buf: vec![0; TILESET_VIEWER_HEIGHT * TILESET_VIEWER_WIDTH * 64]
		}
	}
	pub fn init(&mut self) {
		self.tileset_viewer.limit_update_rate(Some(std::time::Duration::from_micros(16600)))
	}
	pub fn update(&mut self) {
		if self.tileset_viewer.is_open() && !self.tileset_viewer.is_key_down(Key::Escape) {
			for i in self.tileset_window_buf.iter_mut() {
				*i = 0; // write something more funny here!
			}
			self.tileset_viewer
				.update_with_buffer(&self.tileset_window_buf, TILESET_VIEWER_WIDTH * 8, TILESET_VIEWER_HEIGHT * 8)
				.unwrap();
		}
	}
}