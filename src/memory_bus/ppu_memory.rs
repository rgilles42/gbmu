#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
	RGBColour(u8, u8, u8)
}

#[derive(Copy, Clone)]
pub struct CGBTileAttr {
	pub bg_oam_priority: bool,
	pub vertical_flip: bool,
	pub horizontal_flip: bool,
	pub is_from_bank1: bool,
	pub bg_palette_index: u8
}

#[derive(Copy, Clone)]
pub struct OAMObject {
	pub tile_id: u8,
	pub pos_x: u8,
	pub pos_y: u8,
	pub palette_number: u8,
	pub is_from_bank1: bool,
	pub is_using_obp1: bool,
	pub is_x_flipped: bool,
	pub is_y_flipped: bool,
	pub is_under_bg_win: bool,
}

impl OAMObject {
	fn new() -> Self {
		OAMObject {
			tile_id: 0xFF,
			pos_x: 0xFF,
			pos_y: 0xFF,
			palette_number: 0x07,
			is_from_bank1: true,
			is_using_obp1: true,
			is_x_flipped: true,
			is_y_flipped: true,
			is_under_bg_win: true 
		}
	}
}

pub struct PPUMemory {
	pub is_vram_locked: bool,
	pub is_oam_locked: bool,

	video_ram: [u8; 0x2000],
	video_ram2: [u8; 0x2000],
	oam: [u8; 0x00A0],
	lcdc_ram: u8,							// 0xFF40
	pub scy_ram: u8,						// 0xFF42
	pub scx_ram: u8,						// 0xFF43
	pub ly_ram: u8,							// 0xFF44
	pub lyc_ram: u8,						// 0xFF45
	pub oam_dma_reg: u8,					// 0xFF46
	bgp_ram: u8,							// 0xFF47
	obp_ram: [u8; 2],						// 0xFF48 - 0xFF49
	pub wy_ram: u8,							// 0xFF4A
	pub wx_ram: u8,							// 0xFF4B

	pub tiles: [[Tile; 0x80]; 3],			// 0x8000 - 0x97FF
	pub tiles2: [[Tile; 0x80]; 3],			// idem
	pub bg_tilemap0: [[u8; 0x20]; 0x20],	// 0x9800 - 0x9BFF
	pub bg_tilemap0_attr: [[CGBTileAttr; 0x20];0x20],// idem
	pub bg_tilemap1: [[u8; 0x20]; 0x20],	// 0x9C00 - 0x9FFF
	pub bg_tilemap1_attr: [[CGBTileAttr; 0x20];0x20],// idem
	pub objects: [OAMObject; 0x28],			// 0xFE00 - 0xFE9F
	pub lcd_enable: bool,					// 0xFF40 & (1 << 7)
	pub win_using_secondary_tilemap: bool,	// 0xFF40 & (1 << 6)
	pub win_enable: bool,					// 0xFF40 & (1 << 5)
	pub using_fully_common_bg_tileset: bool,// 0xFF40 & (1 << 4)
	pub bg_using_secondary_tilemap: bool,	// 0xFF40 & (1 << 3)
	pub double_heigth_obj: bool,			// 0xFF40 & (1 << 2)
	pub obj_enable: bool,					// 0xFF40 & (1 << 1)
	pub bg_win_enable: bool,				// 0xFF40 & (1 << 0)
	pub lyc_interrupt_enable: bool,			// 0xFF41 & (1 << 6)
	pub ppu_mode_2_interrupt_enable: bool,	// 0xFF41 & (1 << 5)
	pub ppu_mode_1_interrupt_enable: bool,	// 0xFF41 & (1 << 4)
	pub ppu_mode_0_interrupt_enable: bool,	// 0xFF41 & (1 << 3)
	pub lyc_match_flag: bool,				// 0xFF41 & (1 << 2)
	pub ppu_mode_id: u8,					// 0xFF41 & 0x03
	pub bg_palette: [PixelColour; 4],		// 0xFF47
	pub cgb_bg_palettes: [[PixelColour; 4]; 8],
	pub cgb_bg_palette_autoincr: bool,
	pub cgb_bg_palette_addr: u8,
	pub obj_palettes: [[PixelColour; 3]; 2],	// 0xFF48 - 0xFF49
	pub cgb_obj_palettes: [[PixelColour; 3]; 8],
	pub cgb_obj_palette_autoincr: bool,
	pub cgb_obj_palette_addr: u8
}

impl PPUMemory {
	pub fn new() -> Self {
		PPUMemory {
			is_vram_locked: false,
			is_oam_locked: false,
			video_ram: [0; 0x2000],
			video_ram2: [0; 0x2000],
			oam: [0xFF; 0xA0],
			lcdc_ram: 0,
			scy_ram: 0,
			scx_ram: 0,
			ly_ram: 0,
			lyc_ram: 0,
			oam_dma_reg: 0x00,
			bgp_ram: 0,
			obp_ram: [0; 2],
			wy_ram: 0,
			wx_ram: 0,
			tiles: [ [[[TilePixel::Zero;8];8];0x80]; 3],
			tiles2: [ [[[TilePixel::Zero;8];8];0x80]; 3],
			bg_tilemap0: [[0; 0x20]; 0x20],
			bg_tilemap0_attr: [[CGBTileAttr { bg_oam_priority: false, vertical_flip: false, horizontal_flip: false, is_from_bank1: false, bg_palette_index: 0x00}; 0x20]; 0x20],
			bg_tilemap1: [[0; 0x20]; 0x20],
			bg_tilemap1_attr: [[CGBTileAttr { bg_oam_priority: false, vertical_flip: false, horizontal_flip: false, is_from_bank1: false, bg_palette_index: 0x00}; 0x20]; 0x20],
			objects: [OAMObject::new(); 0x28],
			lcd_enable: false,
			win_using_secondary_tilemap: false,
			win_enable: false,
			using_fully_common_bg_tileset: false,
			bg_using_secondary_tilemap: false,
			double_heigth_obj: false,
			obj_enable: false,
			bg_win_enable: false,
			lyc_interrupt_enable: false,
			ppu_mode_2_interrupt_enable: false,
			ppu_mode_1_interrupt_enable: false,
			ppu_mode_0_interrupt_enable: false,
			lyc_match_flag: false,
			ppu_mode_id: 0,
			bg_palette: [PixelColour::White, PixelColour::LightGray, PixelColour::DarkGray, PixelColour::Black],
			cgb_bg_palettes: [[PixelColour::RGBColour(0x00, 0x00, 0x00), PixelColour::RGBColour(0x00, 0x00, 0x00), PixelColour::RGBColour(0x00, 0x00, 0x00), PixelColour::RGBColour(0x00, 0x00, 0x00)]; 8],
			cgb_bg_palette_addr: 0x00,
			cgb_bg_palette_autoincr: false,
			obj_palettes: [[PixelColour::LightGray, PixelColour::DarkGray, PixelColour::Black]; 2],
			cgb_obj_palettes: [[PixelColour::RGBColour(0x00, 0x00, 0x00), PixelColour::RGBColour(0x00, 0x00, 0x00), PixelColour::RGBColour(0x00, 0x00, 0x00)]; 8],
			cgb_obj_palette_addr: 0x00,
			cgb_obj_palette_autoincr: false
		}
	}
	fn write_tile(&mut self, floored_even_addr: usize, is_to_bank1: bool) {
		let tile_reg = floored_even_addr / 0x800;
		let tile_index = (floored_even_addr % 0x800) / 16;
		let row_index = (floored_even_addr % 16) / 2;
		for pixel_index in 0..8 {
			let msb;
			let lsb;
			if is_to_bank1 {
				msb = self.video_ram2[floored_even_addr + 1] & (1 << (7 - pixel_index));
				lsb = self.video_ram2[floored_even_addr] & (1 << (7 - pixel_index));
			} else {
				msb = self.video_ram[floored_even_addr + 1] & (1 << (7 - pixel_index));
				lsb = self.video_ram[floored_even_addr] & (1 << (7 - pixel_index));
			}
			let value = match (msb != 0, lsb != 0) {
				(false, false) => TilePixel::Zero,
				(false, true) => TilePixel::One,
				(true, false) => TilePixel::Two,
				(true, true) => TilePixel::Three
			};
			if is_to_bank1 {
				self.tiles2[tile_reg][tile_index][row_index][pixel_index as usize] = value;
			} else {
				self.tiles[tile_reg][tile_index][row_index][pixel_index as usize] = value;
			}
		}
	}
	pub fn write(&mut self, address: usize, data: u8, is_to_bank1: bool) {
		if address < 0xA000 {
			if self.is_vram_locked {return}
			let address = address - 0x8000;
			if is_to_bank1 {
				self.video_ram2[address] = data;
			} else {
				self.video_ram[address] = data;
			}
			if address < 0x1800 {
				self.write_tile(address & 0xFFFE, is_to_bank1)
			}
			else {
				if address < 0x1C00 {
					if is_to_bank1 {
						let attrs = CGBTileAttr {
							bg_oam_priority: data & 0x80 != 0,
							vertical_flip: data & 0x40 != 0,
							horizontal_flip: data & 0x20 != 0,
							is_from_bank1: data & 0x08 != 0,
							bg_palette_index: data & 0x07
						};
						self.bg_tilemap0_attr[(address - 0x1800) / 0x20][(address - 0x1800) % 0x20] = attrs
					} else {
						self.bg_tilemap0[(address - 0x1800) / 0x20][(address - 0x1800) % 0x20] = data
					}
				} else {
					if is_to_bank1 {
						let attrs = CGBTileAttr {
							bg_oam_priority: data & 0x80 != 0,
							vertical_flip: data & 0x40 != 0,
							horizontal_flip: data & 0x20 != 0,
							is_from_bank1: data & 0x08 != 0,
							bg_palette_index: data & 0x07
						};
						self.bg_tilemap1_attr[(address - 0x1C00) / 0x20][(address - 0x1C00) % 0x20] = attrs
					} else {
						self.bg_tilemap1[(address - 0x1C00) / 0x20][(address - 0x1C00) % 0x20] = data
					}
				}
			}
		} else if address < 0xFEA0 {
			if self.is_oam_locked {return}
			let address = address - 0xFE00;
			self.oam[address] = data;
			match address % 4 {
				0 => self.objects[address / 4].pos_y = data,
				1 => self.objects[address / 4].pos_x = data,
				2 => self.objects[address / 4].tile_id = data,
				_ => {
					self.objects[address / 4].is_under_bg_win = (data & (1 << 7)) != 0;
					self.objects[address / 4].is_y_flipped = (data & (1 << 6)) != 0;
					self.objects[address / 4].is_x_flipped = (data & (1 << 5)) != 0;
					self.objects[address / 4].is_using_obp1 = (data & (1 << 4)) != 0;
					self.objects[address / 4].is_from_bank1 = (data & (1 << 3)) != 0;
					self.objects[address / 4].palette_number = data & 0x07;
				}
			}

		} else if address == 0xFF40 {
			self.lcdc_ram = data;
			self.win_using_secondary_tilemap	= (data & (1 << 6)) != 0;
			self.win_enable						= (data & (1 << 5)) != 0;
			self.using_fully_common_bg_tileset	= (data & (1 << 4)) != 0;
			self.bg_using_secondary_tilemap		= (data & (1 << 3)) != 0;
			self.double_heigth_obj				= (data & (1 << 2)) != 0;
			self.obj_enable						= (data & (1 << 1)) != 0;
			self.bg_win_enable					= (data & 1) != 0;
			let new_lcd_enable			= (data & (1 << 7)) != 0;
			if self.lcd_enable != new_lcd_enable {
				if !new_lcd_enable	{
					self.is_oam_locked = false;
					self.is_vram_locked = false;
					self.ppu_mode_id = 0;
					self.ly_ram = 0;
				}	// LCD/PPU is disabled, freeing all access to display memory and setting ly and ppu_mode to 0;
				self.lcd_enable = new_lcd_enable;
			}
		} else if address == 0xFF41 {
			self.lyc_interrupt_enable			= (data & (1 << 6)) != 0;
			self.ppu_mode_2_interrupt_enable	= (data & (1 << 5)) != 0;
			self.ppu_mode_1_interrupt_enable	= (data & (1 << 4)) != 0;
			self.ppu_mode_0_interrupt_enable	= (data & (1 << 3)) != 0;
		} else if address == 0xFF44 && self.lcd_enable {
			self.ly_ram = data
		}
		else if address == 0xFF47 {
			self.bgp_ram = data;
			for i in 0..4 {
				let colour_code = (data >> 2*i) & 0x03;
				self.bg_palette[i] = match colour_code {
					0 => PixelColour::White,
					1 => PixelColour::LightGray,
					2 => PixelColour::DarkGray,
					_ => PixelColour::Black
				}
			}
		} else if address == 0xFF48 || address == 0xFF49{
			self.obp_ram[address % 2] = data;
			for i in 1..4 {
				let colour_code = (data >> 2*i) & 0x03;
				self.obj_palettes[address % 2][i - 1] = match colour_code {
					0 => PixelColour::White,
					1 => PixelColour::LightGray,
					2 => PixelColour::DarkGray,
					_ => PixelColour::Black
				}
			}
		} else if address == 0xFF68 {
			self.cgb_bg_palette_autoincr = data & 0x80 != 0;
			self.cgb_bg_palette_addr = data & 0x3F;
		} else if address == 0xFF69 {
			if !self.is_vram_locked {
				let selected_pixel_colour = &mut self.cgb_bg_palettes[self.cgb_bg_palette_addr as usize / 8][self.cgb_bg_palette_addr as usize % 8 / 2];
				if let PixelColour::RGBColour(r, g, b) = selected_pixel_colour {
					if self.cgb_bg_palette_addr % 2 == 0 {
						*r = (data & 0x1F) * 8;
						*g = (((*g / 8) & 0x18) | ((data & 0xE0) >> 5)) * 8;
					} else {
						*g = (((*g / 8) & 0x07) | ((data & 0x03) << 3)) * 8;
						*b = ((data & 0x7C) >> 2) * 8;
					}
				} else {
					println!("SHOULD NEVER HAPPEN!");
				}
			}
			if self.cgb_bg_palette_autoincr {self.cgb_bg_palette_addr = (self.cgb_bg_palette_addr + 1) & 0x3F}
		} else if address == 0xFF6A {
			self.cgb_obj_palette_autoincr = data & 0x80 != 0;
			self.cgb_obj_palette_addr = data & 0x3F;
		} else if address == 0xFF6B {
			if !self.is_vram_locked && self.cgb_obj_palette_addr % 8 / 2 != 0 {
				let selected_pixel_colour = &mut self.cgb_obj_palettes[self.cgb_obj_palette_addr as usize / 8][self.cgb_obj_palette_addr as usize % 8 / 2 - 1];
				if let PixelColour::RGBColour(r, g, b) = selected_pixel_colour {
					if self.cgb_obj_palette_addr % 2 == 0 {
						*r = (data & 0x1F) * 8;
						*g = (((*g / 8) & 0x18) | ((data & 0xE0) >> 5)) * 8;
					} else {
						*g = (((*g / 8) & 0x07) | ((data & 0x03) << 3)) * 8;
						*b = ((data & 0x7C) >> 2) * 8;
					}
				} else {
					println!("SHOULD NEVER HAPPEN!");
				}
			}
			if self.cgb_obj_palette_autoincr {self.cgb_obj_palette_addr = (self.cgb_obj_palette_addr + 1) & 0x3F}
		}
	}
	pub fn read(&self, address: usize, is_from_bank1: bool) -> u8 {
		if address < 0xA000 {
			if self.is_vram_locked {
				0xFF
			} else {
				let address = address - 0x8000;
				if is_from_bank1 {
					self.video_ram2[address]
				} else {
					self.video_ram[address]
				}
			}
		} else if address < 0xFEA0 {
			if self.is_oam_locked {
				0xFF
			} else {
				let address = address - 0xFE00;
				self.oam[address]
			}
		}
		else if address == 0xFF40 { self.lcdc_ram }
		else if address == 0xFF41 {
			(self.lyc_interrupt_enable as u8) << 6 |
			(self.ppu_mode_2_interrupt_enable as u8) << 5 |
			(self.ppu_mode_1_interrupt_enable as u8) << 4 |
			(self.ppu_mode_0_interrupt_enable as u8) << 3 |
			(self.lyc_match_flag as u8) << 2 |
			(self.ppu_mode_id & 0x03) 
		}
		else if address == 0xFF44 { self.ly_ram }
		else if address == 0xFF47 {	self.bgp_ram }
		else if address == 0xFF48 { self.obp_ram[0] }
		else if address == 0xFF49 { self.obp_ram[1] }
		else if address == 0xFF68 { ((self.cgb_bg_palette_autoincr as u8) << 7) | self.cgb_bg_palette_addr }
		else if address == 0xFF69 {
			if self.is_vram_locked {0xFF}
			else if let PixelColour::RGBColour(r, g, b) = self.cgb_bg_palettes[self.cgb_bg_palette_addr as usize / 8][self.cgb_bg_palette_addr as usize % 8 / 2] {
				if self.cgb_bg_palette_addr % 2 == 0 {
					(r / 8) | ((g / 8) << 5)
				} else {
					((g / 8) >> 3) | ((b / 8) << 2)
				}
			} else {
				println!("SHOULD NEVER HAPPEN!");
				0xFF
			}
		}
		else if address == 0xFF6A { ((self.cgb_obj_palette_autoincr as u8) << 7) | self.cgb_obj_palette_addr }
		else {	// address == 0xFF6B
			if self.is_vram_locked || self.cgb_obj_palette_addr % 8 / 2 == 0 {0xFF}
			else if let PixelColour::RGBColour(r, g, b) = self.cgb_obj_palettes[self.cgb_obj_palette_addr as usize / 8][self.cgb_obj_palette_addr as usize % 8 / 2 - 1] {
				if self.cgb_obj_palette_addr % 2 == 0 {
					(r / 8) | ((g / 8) << 5)
				} else {
					((g / 8) >> 3) | ((b / 8) << 2)
				}
			} else {
				println!("SHOULD NEVER HAPPEN!");
				0xFF
			}
		}
	}
	pub fn get_bg_tile_index(&self, x: u8, y: u8) -> u8{
		if self.bg_using_secondary_tilemap {
			self.bg_tilemap1[y as usize][x as usize]
		} else {
			self.bg_tilemap0[y as usize][x as usize]
		}
	}
	pub fn get_bg_tile_cgb_attr(&self, x: u8, y: u8) -> CGBTileAttr{
		if self.bg_using_secondary_tilemap {
			self.bg_tilemap1_attr[y as usize][x as usize]
		} else {
			self.bg_tilemap0_attr[y as usize][x as usize]
		}
	}
	pub fn get_win_tile_index(&self, x: u8, y: u8) -> u8{
		if self.win_using_secondary_tilemap {
			self.bg_tilemap1[y as usize][x as usize]
		} else {
			self.bg_tilemap0[y as usize][x as usize]
		}
	}
	pub fn get_win_tile_cgb_attr(&self, x: u8, y: u8) -> CGBTileAttr{
		if self.win_using_secondary_tilemap {
			self.bg_tilemap1_attr[y as usize][x as usize]
		} else {
			self.bg_tilemap0_attr[y as usize][x as usize]
		}
	}
	pub fn get_bg_win_tile(&self, mut tile_index: u8, is_from_bank1: bool) -> Tile {
		let tile_reg =
			if tile_index >= 128 {
				tile_index -= 128;
				1
			} else { if self.using_fully_common_bg_tileset {0} else {2} };
		if is_from_bank1 {
			self.tiles2[tile_reg][tile_index as usize]
		} else {
			self.tiles[tile_reg][tile_index as usize]
		}
	}
	pub fn get_obj_row(&self, mut tile_index: u8, line_index: u8, is_from_bank1: bool) -> TileRow {
		if self.double_heigth_obj {
			tile_index = tile_index & 0xFE;
		}
		let tile_reg = if tile_index >= 128 {tile_index -= 128; 1} else {0};
		if self.double_heigth_obj {
			if is_from_bank1 {
				self.tiles2[tile_reg][if line_index < 8 {tile_index} else {tile_index + 1} as usize][line_index as usize % 8]
			} else {
				self.tiles[tile_reg][if line_index < 8 {tile_index} else {tile_index + 1} as usize][line_index as usize % 8]
			}
		} else {
			if is_from_bank1 {
				self.tiles2[tile_reg][tile_index as usize][line_index as usize]
			} else {
				self.tiles[tile_reg][tile_index as usize][line_index as usize]
			}
		}
	}
}