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
	current_line_obj_rows: Vec<(usize, TileRow, bool, bool, u8)>,
	oam_dma_count: usize,
	vram_dma_count: u16
}

impl Ppu {
	pub fn new() -> Ppu {
		let ppu = Ppu {
			ppu_mode: PPUModes::VBlank(153, 4559),
			current_line_obj_rows: Vec::new(),
			oam_dma_count: 0,
			vram_dma_count: 0x00
		};
		ppu
	}
	fn palette_translation(pixel_colour: &PixelColour) -> [u8; 4] {
		match pixel_colour {
			PixelColour::White => [0xFF, 0xFF, 0xFF, 0xFF],
			PixelColour::LightGray => [0xAA, 0xAA, 0xAA, 0xFF],
			PixelColour::DarkGray => [0x55, 0x55, 0x55, 0xFF],
			PixelColour::Black => [0x00, 0x00, 0x00, 0xFF],
			PixelColour::RGBColour(r, g, b) => [*r, *g, *b, 0xFF]
		}
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
				let tile_attrs = if memory_bus.is_cgb {Some(memory_bus.ppu_memory.get_bg_tile_cgb_attr(x as u8, y as u8))} else {None};
				let mut tile = memory_bus.ppu_memory.get_bg_win_tile(tile_index, memory_bus.is_cgb && tile_attrs.unwrap().is_from_bank1);
				if memory_bus.is_cgb && tile_attrs.unwrap().vertical_flip {tile.reverse()}
				for (row_index, row) in tile.iter_mut().enumerate() {
					if memory_bus.is_cgb && tile_attrs.unwrap().horizontal_flip {row.reverse()}
					for (pixel_index, pixel) in row.iter().enumerate() {
						let tilemap_pixel_pos = y * TILEMAP_PX_WIDTH * TILE_HEIGHT +
							row_index * TILEMAP_PX_WIDTH +
							x * TILE_WIDTH +
							pixel_index;
						let tilemap_pixel = &mut tilemap_framebuffer[tilemap_pixel_pos * 4..(tilemap_pixel_pos + 1) * 4];
						tilemap_pixel.clone_from_slice(&Ppu::palette_translation(&if memory_bus.is_cgb {memory_bus.ppu_memory.cgb_bg_palettes[tile_attrs.unwrap().bg_palette_index as usize]} else {memory_bus.ppu_memory.bg_palette}[
							match pixel {
								TilePixel::Zero =>	0,
								TilePixel::One =>	1,
								TilePixel::Two =>	2,
								TilePixel::Three =>	3
							}
						]))
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
							memory_bus.is_cgb && examined_obj.cgb_is_from_bank1
						);
						self.current_line_obj_rows.push(
							(	examined_obj.pos_x as usize, 
								if examined_obj.is_x_flipped {row.reverse(); row} else {row},
								examined_obj.is_using_obp1,
								examined_obj.is_under_bg_win,
								examined_obj.cgb_palette_number	)
						);
					}
					if !memory_bus.ppu_memory.double_heigth_obj && line + 8 < obj_bottom_line_plus_1 && line + 16 >= obj_bottom_line_plus_1 {
						let mut row = memory_bus.ppu_memory.get_obj_row(examined_obj.tile_id,
							if examined_obj.is_y_flipped {obj_bottom_line_plus_1 - line - 9} else {8 - (obj_bottom_line_plus_1 - line - 8)},
							memory_bus.is_cgb && examined_obj.cgb_is_from_bank1
						);
						self.current_line_obj_rows.push(
							(	examined_obj.pos_x as usize, 
								if examined_obj.is_x_flipped {row.reverse(); row} else {row},
								examined_obj.is_using_obp1,
								examined_obj.is_under_bg_win,
								examined_obj.cgb_palette_number	)
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
					let mut bgwin_is_a_zero_pixel = true;
					let mut tile_attrs = None;
					if memory_bus.ppu_memory.bg_win_enable || memory_bus.is_cgb {
						let pixel;
						if memory_bus.ppu_memory.win_enable && line >= memory_bus.ppu_memory.wy_ram && count as u8 + 7 >= memory_bus.ppu_memory.wx_ram {
							let tile_index = memory_bus.ppu_memory.get_win_tile_index((count as u8 + 7 - memory_bus.ppu_memory.wx_ram) / 8, (line - memory_bus.ppu_memory.wy_ram) / 8);
							tile_attrs = if memory_bus.is_cgb {Some(memory_bus.ppu_memory.get_win_tile_cgb_attr((count as u8 + 7 - memory_bus.ppu_memory.wx_ram) / 8, (line - memory_bus.ppu_memory.wy_ram) / 8))} else {None};
							let mut tile = memory_bus.ppu_memory.get_bg_win_tile(tile_index, memory_bus.is_cgb && tile_attrs.unwrap().is_from_bank1);
							if memory_bus.is_cgb && tile_attrs.unwrap().vertical_flip {tile.reverse()}
							let mut row = tile[(line - memory_bus.ppu_memory.wy_ram) as usize % 8];
							if memory_bus.is_cgb && tile_attrs.unwrap().horizontal_flip {row.reverse()}
							pixel = row[(count + 7 - memory_bus.ppu_memory.wx_ram as usize) % 8];
						} else {
							let tile_index = memory_bus.ppu_memory.get_bg_tile_index(memory_bus.ppu_memory.scx_ram.overflowing_add(count as u8).0 / 8, memory_bus.ppu_memory.scy_ram.overflowing_add(line).0 / 8);
							tile_attrs = if memory_bus.is_cgb {Some(memory_bus.ppu_memory.get_bg_tile_cgb_attr(memory_bus.ppu_memory.scx_ram.overflowing_add(count as u8).0 / 8, memory_bus.ppu_memory.scy_ram.overflowing_add(line).0 / 8))} else {None};
							let mut tile = memory_bus.ppu_memory.get_bg_win_tile(tile_index, memory_bus.is_cgb && tile_attrs.unwrap().is_from_bank1);
							if memory_bus.is_cgb && tile_attrs.unwrap().vertical_flip {tile.reverse()}
							let mut row = tile[(memory_bus.ppu_memory.scy_ram as usize + line as usize) % 8];
							if memory_bus.is_cgb && tile_attrs.unwrap().horizontal_flip {row.reverse()}
							pixel = row[(memory_bus.ppu_memory.scx_ram as usize + count) % 8];
						};
						viewport_pixel.clone_from_slice(&Ppu::palette_translation(&if memory_bus.is_cgb {memory_bus.ppu_memory.cgb_bg_palettes[tile_attrs.unwrap().bg_palette_index as usize]} else {memory_bus.ppu_memory.bg_palette} [
							match pixel {
								TilePixel::Zero =>	0,
								TilePixel::One =>	{bgwin_is_a_zero_pixel = false; 1},
								TilePixel::Two =>	{bgwin_is_a_zero_pixel = false; 2},
								TilePixel::Three =>	{bgwin_is_a_zero_pixel = false; 3},
							}
						]))
					} else {
						viewport_pixel.clone_from_slice(&Ppu::palette_translation(&PixelColour::White));
					}
					if memory_bus.ppu_memory.obj_enable {
						let mut pixel = (TilePixel::Zero, 0, false, 0x00);
						for relevant_row in self.current_line_obj_rows.iter().rev() {
							if count < relevant_row.0 && count + 8 >= relevant_row.0 {
								let pixel_value = relevant_row.1[8 - (relevant_row.0 - count)];
								if pixel_value != TilePixel::Zero {
									pixel = (pixel_value, relevant_row.2 as usize, relevant_row.3, relevant_row.4 as usize);
								}
							}
						}
						if  !memory_bus.ppu_memory.bg_win_enable || (!pixel.2 && (!memory_bus.is_cgb || !tile_attrs.unwrap().bg_oam_priority)) || bgwin_is_a_zero_pixel {
							match pixel.0 {
								TilePixel::Zero =>	{}
								TilePixel::One =>	{viewport_pixel.clone_from_slice(&Ppu::palette_translation(&if memory_bus.is_cgb {memory_bus.ppu_memory.cgb_obj_palettes[pixel.3]} else {memory_bus.ppu_memory.obj_palettes[pixel.1]}[0]))}
								TilePixel::Two =>	{viewport_pixel.clone_from_slice(&Ppu::palette_translation(&if memory_bus.is_cgb {memory_bus.ppu_memory.cgb_obj_palettes[pixel.3]} else {memory_bus.ppu_memory.obj_palettes[pixel.1]}[1]))}
								TilePixel::Three =>	{viewport_pixel.clone_from_slice(&Ppu::palette_translation(&if memory_bus.is_cgb {memory_bus.ppu_memory.cgb_obj_palettes[pixel.3]} else {memory_bus.ppu_memory.obj_palettes[pixel.1]}[2]))}
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
			memory_bus.ppu_memory.oam_dma_is_active = false;
			self.oam_dma_count = 0;
		} else {
			self.oam_dma_count += 1;
		}
	}

	fn tick_vram_dma(&mut self, memory_bus: &mut MemoryBus) {
		memory_bus.write_byte(0x8000 | (memory_bus.ppu_memory.vram_dma_dst_regs & 0x1FF0) | self.vram_dma_count, memory_bus.read_byte((memory_bus.ppu_memory.vram_dma_src_regs  & 0xFFF0) | self.vram_dma_count));
		if self.vram_dma_count == 0x0F {
			self.vram_dma_count = 0x00;
			memory_bus.ppu_memory.vram_dma_stat = memory_bus.ppu_memory.vram_dma_stat.overflowing_sub(0x01).0;
			memory_bus.ppu_memory.vram_dma_dst_regs = (memory_bus.ppu_memory.vram_dma_dst_regs + 0x0010) & 0x1FF0;
			memory_bus.ppu_memory.vram_dma_src_regs =  memory_bus.ppu_memory.vram_dma_src_regs.overflowing_add(0x0010).0;
			if memory_bus.ppu_memory.vram_dma_stat == 0xFF {memory_bus.ppu_memory.vram_dma_is_active = false}
		} else {self.vram_dma_count += 1}
	}

	pub fn tick(&mut self, memory_bus: &mut MemoryBus, framebuffer: &mut [u8]) -> (bool, bool) {
		let mut ppu_doing_vram_dma_transfer = false;
		if memory_bus.ppu_memory.oam_dma_is_active {
			self.tick_oam_dma(memory_bus);
			if memory_bus.is_double_speed {
				self.tick_oam_dma(memory_bus);
			}
		}
		if memory_bus.is_cgb && memory_bus.ppu_memory.vram_dma_is_active {
			if !memory_bus.ppu_memory.vram_dma_is_hblank_mode {
				self.tick_vram_dma(memory_bus);
				ppu_doing_vram_dma_transfer = true;
			} else if let PPUModes::HBlank(_, count) = self.ppu_mode {
				if count >= 360 {
					self.tick_vram_dma(memory_bus);
					ppu_doing_vram_dma_transfer = true;
				}
			}
		} else {self.vram_dma_count = 0x00}
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
		(frame_completed, ppu_doing_vram_dma_transfer)
	}
}
