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
pub const TILESET_VIEWER_PX_WIDTH: usize	= TILESET_PX_WIDTH * 2;
pub const TILESET_VIEWER_PX_HEIGHT: usize	= TILESET_PX_HEIGHT * TILESETS_NB;

const TILEMAP_NB_TILES_WIDTH: usize		= 0x20;
const TILEMAP_NB_TILES_HEIGHT: usize	= 0x20;
pub const TILEMAP_PX_WIDTH: usize	= TILE_WIDTH * TILEMAP_NB_TILES_WIDTH;
pub const TILEMAP_PX_HEIGHT: usize	= TILE_HEIGHT * TILEMAP_NB_TILES_HEIGHT;

pub const VIEWPORT_PX_WIDTH: usize	= 160;
pub const VIEWPORT_PX_HEIGHT: usize	= 144;

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
}

impl Ppu {
	pub fn new() -> Ppu {
		let ppu = Ppu {
			ppu_mode: PPUModes::VBlank(153, 4559),
			palette_translation: HashMap::from([(PixelColour::White, [0xFF, 0xFF, 0xFF, 0xFF]), (PixelColour::LightGray, [0xAA, 0xAA, 0xAA, 0xFF]), (PixelColour::DarkGray, [0x55, 0x55, 0x55, 0xFF]), (PixelColour::Black, [0x00, 0x00, 0x00, 0xFF])]),
			current_line_obj_rows: Vec::new(),
			oam_dma_count: 0,
		};
		ppu
	}
	pub fn update_tileset_win(&mut self, memory_bus: &mut MemoryBus, tileset_framebuffer: &mut [u8]) {
		for (id_bank, bank) in memory_bus.ppu_memory.tiles.iter().enumerate() {
			for (id_tile, tile) in bank.iter().enumerate() {
				for (id_row, row) in tile.iter().enumerate() {
					for (pixel_id, pixel) in row.iter().enumerate() {
						let tileset_pixel_pos = id_bank * TILESET_PX_SIZE * 2 +
							(id_tile / TILESET_NB_TILES_WIDTH) * (TILESET_PX_WIDTH * 2 * TILE_HEIGHT) +
							id_row * TILESET_PX_WIDTH * 2 +
							(id_tile % TILESET_NB_TILES_WIDTH) * TILE_WIDTH +
							pixel_id;
						let tileset_pixel = &mut tileset_framebuffer[tileset_pixel_pos * 4..(tileset_pixel_pos + 1) * 4];
						tileset_pixel.clone_from_slice(
							if id_row == 0 || pixel_id == 0 {&[0x00, 0x00, 0xFF, 0xFF]}
							else {
								match pixel {
									TilePixel::Zero =>	&[0xFF, 0xFF, 0xFF, 0xFF],
									TilePixel::One => 	&[0xA9, 0xA9, 0xA9, 0xFF],
									TilePixel::Two => 	&[0x54, 0x54, 0x54, 0x54],
									TilePixel::Three => &[0x00, 0x00, 0x00, 0xFF]
								}
							}
						)
					}
				}
			}
		}
		for (id_bank, bank) in memory_bus.ppu_memory.tiles2.iter().enumerate() {
			for (id_tile, tile) in bank.iter().enumerate() {
				for (id_row, row) in tile.iter().enumerate() {
					for (pixel_id, pixel) in row.iter().enumerate() {
						let tileset_pixel_pos = id_bank * TILESET_PX_SIZE * 2 +
							(id_tile / TILESET_NB_TILES_WIDTH) * (TILESET_PX_WIDTH * 2 * TILE_HEIGHT) +
							id_row * TILESET_PX_WIDTH * 2 +
							(id_tile % TILESET_NB_TILES_WIDTH) * TILE_WIDTH +
							pixel_id + TILESET_PX_WIDTH;
						let tileset_pixel = &mut tileset_framebuffer[tileset_pixel_pos * 4..(tileset_pixel_pos + 1) * 4];
						tileset_pixel.clone_from_slice(
							if id_row == 0 || pixel_id == 0 {&[0x00, 0xFF, 0x00, 0xFF]}
							else {
								match pixel {
									TilePixel::Zero =>	&[0xFF, 0xFF, 0xFF, 0xFF],
									TilePixel::One => 	&[0xA9, 0xA9, 0xA9, 0xFF],
									TilePixel::Two => 	&[0x54, 0x54, 0x54, 0x54],
									TilePixel::Three => &[0x00, 0x00, 0x00, 0xFF]
								}
							}
						)
					}
				}
			}
		}
	}
	pub fn update_tilemap_win(&mut self, memory_bus: &mut MemoryBus, tilemap_framebuffer: &mut [u8]) {
		for y in 0..TILEMAP_NB_TILES_HEIGHT {
			for x in 0..TILEMAP_NB_TILES_WIDTH {
				let tile_index = memory_bus.ppu_memory.get_bg_tile_index(x as u8, y as u8);
				let tile = memory_bus.ppu_memory.get_bg_win_tile(tile_index, false);
				for (row_index, row) in tile.iter().enumerate() {
					for (pixel_index, pixel) in row.iter().enumerate() {
						let tilemap_pixel_pos = y * TILEMAP_PX_WIDTH * TILE_HEIGHT +
							row_index * TILEMAP_PX_WIDTH +
							x * TILE_WIDTH +
							pixel_index;
						let tilemap_pixel = &mut tilemap_framebuffer[tilemap_pixel_pos * 4..(tilemap_pixel_pos + 1) * 4];
						tilemap_pixel.clone_from_slice(
							match pixel {
								TilePixel::Zero =>	&self.palette_translation[&memory_bus.ppu_memory.bg_palette[0]],
								TilePixel::One =>	&self.palette_translation[&memory_bus.ppu_memory.bg_palette[1]],
								TilePixel::Two =>	&self.palette_translation[&memory_bus.ppu_memory.bg_palette[2]],
								TilePixel::Three =>	&self.palette_translation[&memory_bus.ppu_memory.bg_palette[3]]
							}
						)
					}
				}
			}
		}
		for x in 0..VIEWPORT_PX_WIDTH {
			let tilemap_pixel_pos = memory_bus.ppu_memory.scy_ram as usize * TILEMAP_PX_WIDTH + (memory_bus.ppu_memory.scx_ram as usize + x) % TILEMAP_PX_WIDTH;
			tilemap_framebuffer[tilemap_pixel_pos * 4..(tilemap_pixel_pos + 1) * 4].copy_from_slice(&[0xFF, 0x00, 0x00, 0xFF]);
			let tilemap_pixel_pos = ((memory_bus.ppu_memory.scy_ram as usize + VIEWPORT_PX_HEIGHT - 1) % TILEMAP_PX_HEIGHT) * TILEMAP_PX_WIDTH + (memory_bus.ppu_memory.scx_ram as usize + x) % TILEMAP_PX_WIDTH;
			tilemap_framebuffer[tilemap_pixel_pos * 4..(tilemap_pixel_pos + 1) * 4].copy_from_slice(&[0xFF, 0x00, 0x00, 0xFF]);
		}
		for y in 0..VIEWPORT_PX_HEIGHT {
			let tilemap_pixel_pos = ((memory_bus.ppu_memory.scy_ram as usize + y) % TILEMAP_PX_HEIGHT) * TILEMAP_PX_WIDTH + memory_bus.ppu_memory.scx_ram as usize;
			tilemap_framebuffer[tilemap_pixel_pos * 4..(tilemap_pixel_pos + 1) * 4].copy_from_slice(&[0xFF, 0x00, 0x00, 0xFF]);
			let tilemap_pixel_pos = ((memory_bus.ppu_memory.scy_ram as usize + y) % TILEMAP_PX_HEIGHT) * TILEMAP_PX_WIDTH + (memory_bus.ppu_memory.scx_ram as usize + VIEWPORT_PX_WIDTH - 1) % TILEMAP_PX_WIDTH;
			tilemap_framebuffer[tilemap_pixel_pos * 4..(tilemap_pixel_pos + 1) * 4].copy_from_slice(&[0xFF, 0x00, 0x00, 0xFF]);
		}
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
							if examined_obj.is_y_flipped {obj_bottom_line_plus_1 - line - 1} else {16 - (obj_bottom_line_plus_1 - line)},
							false
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
							if examined_obj.is_y_flipped {obj_bottom_line_plus_1 - line - 9} else {8 - (obj_bottom_line_plus_1 - line - 8)},
							false
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
				if count == 0 && !memory_bus.is_cgb{
					self.current_line_obj_rows.sort_by(|row_a, row_b| row_a.0.cmp(&row_b.0));
				}
				if count < VIEWPORT_PX_WIDTH {
					let viewport_pixel = &mut framebuffer[((line as usize) * VIEWPORT_PX_WIDTH + count) * 4..((line as usize) * VIEWPORT_PX_WIDTH + count + 1) * 4];
					if memory_bus.ppu_memory.bg_win_enable {
						if memory_bus.ppu_memory.win_enable && line >= memory_bus.ppu_memory.wy_ram && count as u8 + 7 >= memory_bus.ppu_memory.wx_ram {
							let tile_index = memory_bus.ppu_memory.get_win_tile_index((count as u8 + 7 - memory_bus.ppu_memory.wx_ram) / 8, (line - memory_bus.ppu_memory.wy_ram) / 8);
							let tile = memory_bus.ppu_memory.get_bg_win_tile(tile_index, false);
							let pixel = tile[(line - memory_bus.ppu_memory.wy_ram) as usize % 8][(count + 7 - memory_bus.ppu_memory.wx_ram as usize) % 8];
							viewport_pixel.clone_from_slice(match pixel {
								TilePixel::Zero =>	&self.palette_translation[&memory_bus.ppu_memory.bg_palette[0]],
								TilePixel::One =>	&self.palette_translation[&memory_bus.ppu_memory.bg_palette[1]],
								TilePixel::Two =>	&self.palette_translation[&memory_bus.ppu_memory.bg_palette[2]],
								TilePixel::Three =>	&self.palette_translation[&memory_bus.ppu_memory.bg_palette[3]],
							})
						} else {
							let tile_index = memory_bus.ppu_memory.get_bg_tile_index(memory_bus.ppu_memory.scx_ram.overflowing_add(count as u8).0 / 8, memory_bus.ppu_memory.scy_ram.overflowing_add(line).0 / 8);
							let tile = memory_bus.ppu_memory.get_bg_win_tile(tile_index, false);
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
						for relevant_row in self.current_line_obj_rows.iter().rev() {
							if count < relevant_row.0 && count + 8 >= relevant_row.0 {
								let pixel_value = relevant_row.1[8 - (relevant_row.0 - count)];
								if pixel_value != TilePixel::Zero {
									pixel = (pixel_value, relevant_row.2 as usize, relevant_row.3);
								}
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
	fn tick_oam_dma(&mut self, memory_bus: &mut MemoryBus) {
		if self.oam_dma_count % 4 == 3 && self.oam_dma_count != 3{
			//TODO do not use general read/write
			memory_bus.ppu_memory.write(0xFE00 + self.oam_dma_count / 4 - 1,
				memory_bus.read_byte(memory_bus.ppu_memory.oam_dma_reg as u16 * 0x100 + self.oam_dma_count as u16 / 4 - 1),
				false
			)
		}
		if self.oam_dma_count == 643 {
			memory_bus.ppu_memory.oam_dma_reg = 0x00;
			self.oam_dma_count = 0;
		} else {
			self.oam_dma_count += 1;
		}
	}

	pub fn tick(&mut self, memory_bus: &mut MemoryBus, framebuffer: &mut [u8]) -> bool {
		let mut frame_completed = false;
		if memory_bus.ppu_memory.lcd_enable {
			self.tick_viewport(memory_bus, framebuffer);
			if let PPUModes::VBlank(153, 4559) = self.ppu_mode  {
				frame_completed = true;
			}
			self.tick_ppu_mode(memory_bus);
		}
		else {
			self.ppu_mode = PPUModes::VBlank(153, 4559)
		}
		if memory_bus.ppu_memory.oam_dma_reg != 0x00 {
			self.tick_oam_dma(memory_bus);
		}
		frame_completed
	}
}
