use std::collections::HashMap;
//use minifb::{Key, Window, WindowOptions};
use crate::memory_bus::MemoryBus;
use crate::memory_bus::ppu_memory::{TilePixel, PixelColour, TileRow};

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

#[derive(Debug, Clone, Copy)]
enum PPUModes {
	OAMSearch(u8, usize),
	LineDraw(u8, usize),
	HBlank(u8, usize),
	VBlank(u8, usize)
}

pub struct Ppu {
	ppu_mode: PPUModes,
	palette_translation: HashMap<PixelColour, [u8; 4]>,

	current_line_obj_rows: Vec<(usize, TileRow, bool, bool)>,

	oam_dma_count: usize,

	// tileset_viewer: Option<Window>,
	// tileset_window_buf: Vec<u32>,
	// tilemap_viewer: Option<Window>,
	// tilemap_buf: Vec<u32>,
	// viewport: Window,
	// viewport_buffer: Vec<u32>
}

impl Ppu {
	pub fn new(with_tileset: bool, with_tilemap: bool) -> Ppu {
		let ppu = Ppu {
			ppu_mode: PPUModes::OAMSearch(0, 0),
			palette_translation: HashMap::from([(PixelColour::White, [0xFF, 0xFF, 0xFF, 0xFF]), (PixelColour::LightGray, [0xA9, 0xA9, 0xA9, 0xFF]), (PixelColour::DarkGray, [0x54, 0x54, 0x54, 0xFF]), (PixelColour::Black, [0x00, 0x00, 0x00, 0xFF])]),
			current_line_obj_rows: Vec::new(),
			oam_dma_count: 0,
			// tileset_viewer: if with_tileset {Some(Window::new("Tileset", TILESET_VIEWER_PX_WIDTH, TILESET_VIEWER_PX_HEIGHT, WindowOptions {scale: minifb::Scale::X4,..WindowOptions::default()}).unwrap())} else {None},
			// tileset_window_buf: vec![0; TILESET_VIEWER_PX_SIZE],
			// tilemap_viewer: if with_tilemap {Some(Window::new("Tilemap", TILEMAP_PX_WIDTH, TILEMAP_PX_HEIGHT, WindowOptions {scale: minifb::Scale::X4,..WindowOptions::default()}).unwrap())} else {None},
			// tilemap_buf: vec![0; TILEMAP_PX_SIZE],
			// viewport: Window::new("GBMU", VIEWPORT_PX_WIDTH, VIEWPORT_PX_HEIGHT, WindowOptions {scale: minifb::Scale::X4,..WindowOptions::default()}).unwrap(),
			// viewport_buffer: vec![0; VIEWPORT_PX_SIZE]
		};
		ppu
	}
	fn tick_ppu_mode(&mut self, memory_bus: &mut MemoryBus) {
		self.ppu_mode = match self.ppu_mode {
			PPUModes::OAMSearch(line_index, count) =>
				if count == 79 {
					memory_bus.ppu_memory.is_vram_locked = true;
					memory_bus.ppu_memory.ppu_mode_id = 3;
					PPUModes::LineDraw(line_index, 0)
				} else {
					PPUModes::OAMSearch(line_index, count + 1)
				},
			PPUModes::LineDraw(line_index, count) =>
				if count == 167 {
					memory_bus.ppu_memory.is_vram_locked = false;
					memory_bus.ppu_memory.is_oam_locked = false;
					memory_bus.ppu_memory.ppu_mode_id = 0;
					if memory_bus.ppu_memory.ppu_mode_0_interrupt_enable {
						memory_bus.write_byte(0xFF0F, memory_bus.read_byte(0xFF0F) | (1 << 1));
					}
					PPUModes::HBlank(line_index, count + 1)
				} else {
					PPUModes::LineDraw(line_index, count + 1)
				},
			PPUModes::HBlank(line_index, count) =>
				if count == 375 {
					if line_index == 143 {
						memory_bus.write_byte(0xFF0F, memory_bus.read_byte(0xFF0F) | (1 << 0));
						memory_bus.ppu_memory.ppu_mode_id = 1;
						if memory_bus.ppu_memory.ppu_mode_1_interrupt_enable {
							memory_bus.write_byte(0xFF0F, memory_bus.read_byte(0xFF0F) | (1 << 1));
						}
						PPUModes::VBlank(144, 0)
					} else {
						memory_bus.ppu_memory.is_oam_locked = true;
						memory_bus.ppu_memory.ppu_mode_id = 2;
						if memory_bus.ppu_memory.ppu_mode_2_interrupt_enable {
							memory_bus.write_byte(0xFF0F, memory_bus.read_byte(0xFF0F) | (1 << 1));
						}
						PPUModes::OAMSearch(line_index + 1, 0)
					}
				} else {
					PPUModes::HBlank(line_index, count + 1)
				},
			PPUModes::VBlank(_, count) =>
				if count == 4559 {
					memory_bus.ppu_memory.is_oam_locked = true;
					memory_bus.ppu_memory.ppu_mode_id = 2;
						if memory_bus.ppu_memory.ppu_mode_2_interrupt_enable {
							memory_bus.write_byte(0xFF0F, memory_bus.read_byte(0xFF0F) | (1 << 1));
						}
					PPUModes::OAMSearch(0, 0)
				} else {
					PPUModes::VBlank(144 + ((count + 1) / 456) as u8, count + 1)
				},
		}
	}
	fn tick_viewport(&mut self, memory_bus: &mut MemoryBus, framebuffer: &mut [u8]) {
		match self.ppu_mode {
			PPUModes::OAMSearch(line, count) => {
				if count == 0 {
					memory_bus.ppu_memory.ly_ram = line;
					memory_bus.ppu_memory.lyc_match_flag = line == memory_bus.ppu_memory.lyc_ram;
					if memory_bus.ppu_memory.lyc_match_flag && memory_bus.ppu_memory.lyc_interrupt_enable {
						memory_bus.write_byte(0xFF0F, memory_bus.read_byte(0xFF0F) | (1 << 1));
					}
					self.current_line_obj_rows.clear();
				}
				if count % 2 == 0 && self.current_line_obj_rows.len() != 10 {
					let examined_obj = memory_bus.ppu_memory.objects[count / 2];
					let obj_bottom_line_plus_1 = examined_obj.pos_y;
					if memory_bus.ppu_memory.double_heigth_obj && line < obj_bottom_line_plus_1 && line + 16 >= obj_bottom_line_plus_1 {
						let mut row = memory_bus.ppu_memory.get_obj_row(examined_obj.tile_id,
							if examined_obj.is_y_flipped {obj_bottom_line_plus_1 - line - 1} else {16 - (obj_bottom_line_plus_1 - line)}
						);
						self.current_line_obj_rows.push(
							(	examined_obj.pos_x as usize, 
								if examined_obj.is_x_flipped {row.reverse(); row} else {row},
								examined_obj.is_using_obp1,
								examined_obj.is_under_bg_win	)
						);
					}
					if !memory_bus.ppu_memory.double_heigth_obj && line + 8 < obj_bottom_line_plus_1 && line + 16 >= obj_bottom_line_plus_1 {
						let mut row = memory_bus.ppu_memory.get_obj_row(examined_obj.tile_id,
							if examined_obj.is_y_flipped {obj_bottom_line_plus_1 - line - 9} else {8 - (obj_bottom_line_plus_1 - line - 8)}
						);
						self.current_line_obj_rows.push(
							(	examined_obj.pos_x as usize, 
								if examined_obj.is_x_flipped {row.reverse(); row} else {row},
								examined_obj.is_using_obp1,
								examined_obj.is_under_bg_win	)
						);
					}
				}
			},
			PPUModes::LineDraw(line, count) => {
				if count == 0 {
					// if non-cgb
					self.current_line_obj_rows.sort_by(|row_a, row_b| row_a.0.cmp(&row_b.0))
				}
				if count < VIEWPORT_PX_WIDTH {
					let viewport_pixel = &mut framebuffer[((line as usize) * VIEWPORT_PX_WIDTH + count) * 4..((line as usize) * VIEWPORT_PX_WIDTH + count + 1) * 4];
					if memory_bus.ppu_memory.bg_win_enable {
						if memory_bus.ppu_memory.win_enable && line >= memory_bus.ppu_memory.wy_ram && count as u8 + 7 >= memory_bus.ppu_memory.wx_ram {
							let tile_index = memory_bus.ppu_memory.get_win_tile_index((count as u8 + 7 - memory_bus.ppu_memory.wx_ram) / 8, (line - memory_bus.ppu_memory.wy_ram) / 8);
							let tile = memory_bus.ppu_memory.get_bg_win_tile(tile_index);
							let pixel = tile[(line - memory_bus.ppu_memory.wy_ram) as usize % 8][(count + 7 - memory_bus.ppu_memory.wx_ram as usize) % 8];
							viewport_pixel.clone_from_slice(match pixel {
								TilePixel::Zero =>	&self.palette_translation[&memory_bus.ppu_memory.bg_palette[0]],
								TilePixel::One =>	&self.palette_translation[&memory_bus.ppu_memory.bg_palette[1]],
								TilePixel::Two =>	&self.palette_translation[&memory_bus.ppu_memory.bg_palette[2]],
								TilePixel::Three =>	&self.palette_translation[&memory_bus.ppu_memory.bg_palette[3]],
							})
						} else {
							let tile_index = memory_bus.ppu_memory.get_bg_tile_index(memory_bus.ppu_memory.scx_ram.overflowing_add(count as u8).0 / 8, memory_bus.ppu_memory.scy_ram.overflowing_add(line).0 / 8);
							let tile = memory_bus.ppu_memory.get_bg_win_tile(tile_index);
							let pixel = tile[(memory_bus.ppu_memory.scy_ram as usize + line as usize) % 8][(memory_bus.ppu_memory.scx_ram as usize + count) % 8];
							viewport_pixel.clone_from_slice(match pixel {
								TilePixel::Zero =>	&self.palette_translation[&memory_bus.ppu_memory.bg_palette[0]],
								TilePixel::One =>	&self.palette_translation[&memory_bus.ppu_memory.bg_palette[1]],
								TilePixel::Two =>	&self.palette_translation[&memory_bus.ppu_memory.bg_palette[2]],
								TilePixel::Three =>	&self.palette_translation[&memory_bus.ppu_memory.bg_palette[3]],
							})
						};
						
					} else {
						viewport_pixel.clone_from_slice(&self.palette_translation[&PixelColour::White]);
					}
					if memory_bus.ppu_memory.obj_enable {
						let mut pixel = (TilePixel::Zero, 0, false);
						for relevant_row in &self.current_line_obj_rows {
							if count < relevant_row.0 && count + 8 >= relevant_row.0 {
								pixel = (relevant_row.1[8 - (relevant_row.0 - count)], relevant_row.2 as usize, relevant_row.3);
							}
						}
						if !pixel.2 || viewport_pixel == self.palette_translation[&memory_bus.ppu_memory.bg_palette[0]] {
							match pixel.0 {
								TilePixel::Zero =>	{}
								TilePixel::One =>	{viewport_pixel.clone_from_slice(&self.palette_translation[&memory_bus.ppu_memory.obj_palette[pixel.1][0]])}
								TilePixel::Two =>	{viewport_pixel.clone_from_slice(&self.palette_translation[&memory_bus.ppu_memory.obj_palette[pixel.1][1]])}
								TilePixel::Three =>	{viewport_pixel.clone_from_slice(&self.palette_translation[&memory_bus.ppu_memory.obj_palette[pixel.1][2]])}
							}
						}
					}
				}
			},
			PPUModes::HBlank(_, _) => {},
			PPUModes::VBlank(line, _) => {
				memory_bus.ppu_memory.ly_ram = line;
			},
		}
	}
	// fn update_tileset_win(&mut self, memory_bus: &mut MemoryBus) {
	// 	for (id_bank, bank) in memory_bus.ppu_memory.tiles.iter().enumerate() {
	// 		for (id_tile, tile) in bank.iter().enumerate() {
	// 			for (id_row, row) in tile.iter().enumerate() {
	// 				for (pixel_id, pixel) in row.iter().enumerate() {
	// 					self.tileset_window_buf[
	// 						id_bank * TILESET_PX_SIZE +
	// 						(id_tile / TILESET_NB_TILES_WIDTH) * (TILESET_PX_WIDTH * TILE_HEIGHT) +
	// 						id_row * TILESET_PX_WIDTH +
	// 						(id_tile % TILESET_NB_TILES_WIDTH) * TILE_WIDTH +
	// 						pixel_id
	// 					] = if id_row == 0 || pixel_id == 0 {0x000000FF}
	// 					else {
	// 						match pixel {
	// 							TilePixel::Zero => 0x00FFFFFF,
	// 							TilePixel::One => 0x00A9A9A9,
	// 							TilePixel::Two => 0x00545454,
	// 							TilePixel::Three => 0x00000000,
	// 						}
	// 					};
	// 				}
	// 			}
	// 		}
	// 		self.tileset_viewer.as_mut().unwrap()
	// 			.update_with_buffer(&self.tileset_window_buf, TILESET_VIEWER_PX_WIDTH, TILESET_VIEWER_PX_HEIGHT)
	// 			.unwrap();
	// 	}
	// }
	// fn update_tilemap_win(&mut self, memory_bus: &mut MemoryBus) {
	// 	for y in 0..TILEMAP_NB_TILES_HEIGHT {
	// 		for x in 0..TILEMAP_NB_TILES_WIDTH {
	// 			let tile_index = memory_bus.ppu_memory.get_bg_tile_index(x as u8, y as u8);
	// 			let tile = memory_bus.ppu_memory.get_bg_win_tile(tile_index);
	// 			for (row_index, row) in tile.iter().enumerate() {
	// 				for (pixel_index, pixel) in row.iter().enumerate() {
	// 					self.tilemap_buf[
	// 						y * TILEMAP_PX_WIDTH * TILE_HEIGHT +
	// 						row_index * TILEMAP_PX_WIDTH +
	// 						x * TILE_WIDTH +
	// 						pixel_index
	// 					] =	match pixel {
	// 						TilePixel::Zero => self.palette_translation[&memory_bus.ppu_memory.bg_palette[0]],
	// 						TilePixel::One => self.palette_translation[&memory_bus.ppu_memory.bg_palette[1]],
	// 						TilePixel::Two => self.palette_translation[&memory_bus.ppu_memory.bg_palette[2]],
	// 						TilePixel::Three => self.palette_translation[&memory_bus.ppu_memory.bg_palette[3]],
	// 					}
	// 				}
	// 			}
	// 		}
	// 	}
	// 	for x in 0..VIEWPORT_PX_WIDTH {
	// 		self.tilemap_buf[memory_bus.ppu_memory.scy_ram as usize * TILEMAP_PX_WIDTH + (memory_bus.ppu_memory.scx_ram as usize + x) % TILEMAP_PX_WIDTH] = 0x00FF0000;
	// 		self.tilemap_buf[((memory_bus.ppu_memory.scy_ram as usize + VIEWPORT_PX_HEIGHT - 1) % TILEMAP_PX_HEIGHT) * TILEMAP_PX_WIDTH + (memory_bus.ppu_memory.scx_ram as usize + x) % TILEMAP_PX_WIDTH] = 0x00FF0000;
	// 	}
	// 	for y in 0..VIEWPORT_PX_HEIGHT {
	// 		self.tilemap_buf[((memory_bus.ppu_memory.scy_ram as usize + y) % TILEMAP_PX_HEIGHT) * TILEMAP_PX_WIDTH + memory_bus.ppu_memory.scx_ram as usize] = 0x00FF0000;
	// 		self.tilemap_buf[((memory_bus.ppu_memory.scy_ram as usize + y) % TILEMAP_PX_HEIGHT) * TILEMAP_PX_WIDTH + (memory_bus.ppu_memory.scx_ram as usize + VIEWPORT_PX_WIDTH - 1) % TILEMAP_PX_WIDTH] = 0x00FF0000;
	// 	}
	// 	self.tilemap_viewer.as_mut().unwrap()
	// 		.update_with_buffer(&self.tilemap_buf, TILEMAP_PX_WIDTH, TILEMAP_PX_HEIGHT)
	// 		.unwrap();
	// }
	// fn update_windows(&mut self, memory_bus: &mut MemoryBus) {
	// 	self.viewport
	// 		.update_with_buffer(&self.viewport_buffer, VIEWPORT_PX_WIDTH, VIEWPORT_PX_HEIGHT)
	// 		.unwrap();
	// 	if let Some(_) = self.tilemap_viewer {
	// 		if self.tilemap_viewer.as_mut().unwrap().is_open() && !self.tilemap_viewer.as_mut().unwrap().is_key_down(Key::Escape) {
	// 			self.update_tilemap_win(memory_bus);
	// 		}
	// 	}
	// 	if let Some(_) = self.tileset_viewer {
	// 		if self.tileset_viewer.as_mut().unwrap().is_open() && !self.tileset_viewer.as_mut().unwrap().is_key_down(Key::Escape) {
	// 			self.update_tileset_win(memory_bus);
	// 		}
	// 	}
	// }

	fn tick_oam_dma(&mut self, memory_bus: &mut MemoryBus) {
		if self.oam_dma_count % 4 == 3 && self.oam_dma_count != 3{
			//TODO do not use general read/write
			memory_bus.ppu_memory.write(0xFE00 + self.oam_dma_count / 4 - 1,
				memory_bus.read_byte(memory_bus.oam_dma_reg as u16 * 0x100 + self.oam_dma_count as u16 / 4 - 1)
			)
		}
		if self.oam_dma_count == 643 {
			memory_bus.oam_dma_reg = 0x00;
			self.oam_dma_count = 0;
		} else {
			self.oam_dma_count += 1;
		}
	}

	pub fn tick(&mut self, memory_bus: &mut MemoryBus, framebuffer: &mut [u8]) -> bool {
		let mut frame_completed = false;
		if memory_bus.ppu_memory.lcd_enable {
			self.tick_viewport(memory_bus, framebuffer);
			if let PPUModes::VBlank(_, 0) = self.ppu_mode  {
				frame_completed = true;
				//self.update_windows(memory_bus);
			}
			self.tick_ppu_mode(memory_bus);
		}
		else {
			self.ppu_mode = PPUModes::OAMSearch(0, 0)
		}
		if memory_bus.oam_dma_reg != 0x00 {
			self.tick_oam_dma(memory_bus);
		}
		frame_completed
	}
}

// #[cfg(test)]
// mod tests {
//     use crate::memory_bus::MemoryBus;

//     use super::*;
// 	#[test]
// 	fn test_ppu_mode() {
// 		let mut memory_bus = MemoryBus::new(None);
// 		let mut ppu = Ppu::new(false, false);
// 		memory_bus.ppu_memory.lcd_enable = true;
// 		std::thread::sleep(std::time::Duration::from_millis(500));					// or first minifb update will fail
// 		for _ in 0..70224 {
// 			ppu.tick(&mut memory_bus);
// 			println!("{:?}", ppu.ppu_mode);
// 		}
// 		// loop {
// 		// }
// 	}
// 	#[test]
// 	fn test_tileset_fill() {
// 		let mut memory_bus = MemoryBus::new(None);
// 		let mut ppu = Ppu::new(true, false);
// 		std::thread::sleep(std::time::Duration::from_millis(500));					// or first minifb update will fail
// 		for _i in 0..70224 {
// 			ppu.tick(&mut memory_bus);
// 		}
// 		std::thread::sleep(std::time::Duration::from_millis(1000));
// 		for i in 0..0x6000 {
// 			let tiles_bank =	(i / TILE_WIDTH / TILE_HEIGHT / TILESET_NB_TILES_HEIGHT / TILESET_NB_TILES_WIDTH ) % TILESETS_NB;
// 			let tile_index =	(i / TILE_WIDTH / TILE_HEIGHT) % (TILESET_NB_TILES_HEIGHT * TILESET_NB_TILES_WIDTH);
// 			let tile_row =	(i / TILE_WIDTH) % TILE_HEIGHT ;
// 			let tile_pixel =	 i % TILE_WIDTH;
// 			memory_bus.ppu_memory.tiles[tiles_bank][tile_index][tile_row][tile_pixel] = match tiles_bank {
// 				0 => TilePixel::One,
// 				1 => TilePixel::Two,
// 				2 => TilePixel::Three,
// 				_ => TilePixel::Zero
// 			};
// 			ppu.update_tileset_win(&mut memory_bus);
// 		}
// 	}
// 	#[test]
// 	fn test_tileset_direct() {
// 		let mut memory_bus = MemoryBus::new(None);
// 		let mut ppu = Ppu::new(true, false);
// 		memory_bus.ppu_memory.tiles[0][0x19][0] = [TilePixel::Zero, TilePixel::Zero, TilePixel::One, TilePixel::One, TilePixel::One, TilePixel::One, TilePixel::Zero, TilePixel::Zero];
// 		memory_bus.ppu_memory.tiles[0][0x19][1] = [TilePixel::Zero, TilePixel::One, TilePixel::Zero, TilePixel::Zero, TilePixel::Zero, TilePixel::Zero, TilePixel::One, TilePixel::Zero];
// 		memory_bus.ppu_memory.tiles[0][0x19][2] = [TilePixel::One, TilePixel::Zero, TilePixel::One, TilePixel::One, TilePixel::One, TilePixel::Zero, TilePixel::Zero, TilePixel::One];
// 		memory_bus.ppu_memory.tiles[0][0x19][3] = [TilePixel::One, TilePixel::Zero, TilePixel::One, TilePixel::Zero, TilePixel::Zero, TilePixel::One, TilePixel::Zero, TilePixel::One];
// 		memory_bus.ppu_memory.tiles[0][0x19][4] = [TilePixel::One, TilePixel::Zero, TilePixel::One, TilePixel::One, TilePixel::One, TilePixel::Zero, TilePixel::Zero, TilePixel::One];
// 		memory_bus.ppu_memory.tiles[0][0x19][5] = [TilePixel::One, TilePixel::Zero, TilePixel::One, TilePixel::Zero, TilePixel::Zero, TilePixel::One, TilePixel::Zero, TilePixel::One];
// 		memory_bus.ppu_memory.tiles[0][0x19][6] = [TilePixel::Zero, TilePixel::One, TilePixel::Zero, TilePixel::Zero, TilePixel::Zero, TilePixel::Zero, TilePixel::One, TilePixel::Zero];
// 		memory_bus.ppu_memory.tiles[0][0x19][7] = [TilePixel::Zero, TilePixel::Zero, TilePixel::One, TilePixel::One, TilePixel::One, TilePixel::One, TilePixel::Zero, TilePixel::Zero];
// 		loop {
// 			ppu.update_tileset_win(&mut memory_bus);						// Should display an ® logo in tile {1; 9}
// 		}
// 	}
// 	#[test]
// 	fn test_tileset_generation() {
// 		let mut memory_bus = MemoryBus::new(None);
// 		let mut ppu = Ppu::new(true, false);
// 		memory_bus.ppu_memory.write(0x8190, 0x3c);
// 		memory_bus.ppu_memory.write(0x8192, 0x42);
// 		memory_bus.ppu_memory.write(0x8194, 0xb9);
// 		memory_bus.ppu_memory.write(0x8196, 0xa5);
// 		memory_bus.ppu_memory.write(0x8198, 0xb9);
// 		memory_bus.ppu_memory.write(0x819a, 0xa5);
// 		memory_bus.ppu_memory.write(0x819c, 0x42);
// 		memory_bus.ppu_memory.write(0x819e, 0x3c);
// 		loop {
// 			ppu.update_tileset_win(&mut memory_bus);						// Should display an ® logo in tile {1; 9}
// 		}
// 	}
// 	#[test]
// 	fn test_tilemap_and_viewport_composition() {
// 		let mut memory_bus = MemoryBus::new(None);
// 		let mut ppu = Ppu::new(false, true);
// 		let mut cpu = crate::Cpu::new();
// 		memory_bus.load_dmg_bootrom();
// 		memory_bus.cartridge._debug_insert_cart_logo();
// 		cpu.tick(&mut memory_bus);
// 		while cpu.registers.program_counter - 1 != 0x55 {
// 			cpu.tick(&mut memory_bus);
// 		}
// 		memory_bus.ppu_memory.bg_win_enable = true;
// 		memory_bus.ppu_memory.using_fully_common_bg_tileset = true;
// 		memory_bus.ppu_memory.lcd_enable = true;
// 		memory_bus.ppu_memory.bg_tilemap0 = [
// 			[0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19],
// 			[0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00],
// 			[0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00],
// 			[0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00],
// 			[0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00],
// 			[0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00],
// 			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
// 			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
// 			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
// 			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
// 			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
// 			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
// 			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
// 			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
// 			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
// 			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
// 			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
// 			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
// 			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
// 			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
// 			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
// 			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
// 			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
// 			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
// 			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
// 			[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
// 			[0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00],
// 			[0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00],
// 			[0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00],
// 			[0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00],
// 			[0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00],
// 			[0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19],
// 		];
// 		loop {
// 			for _i in 0..70224 {
// 				ppu.tick(&mut memory_bus);
// 			}
// 			memory_bus.write_byte(0xFF42, memory_bus.read_byte(0xFF42).overflowing_add(1).0);
// 			memory_bus.write_byte(0xFF43, memory_bus.read_byte(0xFF43).overflowing_add(1).0);
// 		}
// 	}
// }