use std::{fs, time::{SystemTime, Duration, UNIX_EPOCH}};
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MapperType {
	None, MBC1, MBC2, MBC3, MBC5
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ROMType {
	X2_32KiB, X4_64KiB, X8_128KiB, X16_256KiB, X32_512KiB, X64_1MiB, X128_2MiB, X256_4MiB, X512_8MiB
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RAMType {
	None, X1_8KiB, X4_32KiB, X8_64KiB, X16_128KiB
}

pub struct Cartridge {
	path: String,
	mapper_type: MapperType,
	rom_type: ROMType,
	rom_banks: Vec<[u8; 0x4000]>,
	current_2d_rom_bank: usize,
	ram_type: RAMType,
	ram_banks: Vec<[u8; 0x2000]>,
	ram_enable: bool,
	current_ram_bank: usize,
	has_battery: bool,
	mbc1_banking_mode: bool,
	mbc1_current_rom_banks_upper_bytes: usize,
	mbc3_has_rtc: bool,
	mbc3_rtc_last_update_timestamp: SystemTime,
	mbc3_rtc_registers: [Duration; 2],
	mbc3_rtc_latch_prev_value: u8,
	mbc3_rtc_is_latched: bool,
	mbc3_rtc_is_halted: bool,
	mbc5_9th_rom_bank_bit: usize
}

impl Drop for Cartridge {
    fn drop(&mut self) {
        if self.has_battery {
			let mut sav_contents = Vec::new();
			if self.ram_type != RAMType::None {
				if self.mapper_type == MapperType::MBC2 {
					sav_contents.extend_from_slice(&self.ram_banks[0][0x00..=0x01FF])
				} else {
					for bank in &self.ram_banks {
						sav_contents.extend_from_slice(bank)
					}
				}
			}
			if self.mbc3_has_rtc {
				sav_contents.push(((self.mbc3_rtc_registers[0]).as_secs() % 60) as u8);
				sav_contents.push((((self.mbc3_rtc_registers[0]).as_secs() / 60) % 60) as u8);
				sav_contents.push((((self.mbc3_rtc_registers[0]).as_secs() / 3600) % 24) as u8);
				sav_contents.push((((self.mbc3_rtc_registers[0]).as_secs() / 86400) % 256) as u8);
				let nb_256days_periods_elapsed = (self.mbc3_rtc_registers[0]).as_secs() / 22118400;
				sav_contents.push((nb_256days_periods_elapsed % 2) as u8 | (self.mbc3_rtc_is_halted as u8) << 6 | if nb_256days_periods_elapsed >= 2 {1} else {0} << 7);
				sav_contents.push(((self.mbc3_rtc_registers[1]).as_secs() % 60) as u8);
				sav_contents.push((((self.mbc3_rtc_registers[1]).as_secs() / 60) % 60) as u8);
				sav_contents.push((((self.mbc3_rtc_registers[1]).as_secs() / 3600) % 24) as u8);
				sav_contents.push((((self.mbc3_rtc_registers[1]).as_secs() / 86400) % 256) as u8);
				let nb_256days_periods_elapsed = (self.mbc3_rtc_registers[1]).as_secs() / 22118400;
				sav_contents.push((nb_256days_periods_elapsed % 2) as u8 | (self.mbc3_rtc_is_halted as u8) << 6 | if nb_256days_periods_elapsed >= 2 {1} else {0} << 7);
				let unix_timestamp = self.mbc3_rtc_last_update_timestamp.duration_since(UNIX_EPOCH).expect("We are before epoch!").as_secs();
				for i in 0..=7 {
					sav_contents.push(((unix_timestamp >> i * 8) & 0xFF) as u8)
				}

			}
			let path = self.path.clone() + ".sav";
			fs::write(path, sav_contents).unwrap_or_default();
		}
    }
}

impl Cartridge {
	pub fn new(rom_path: Option<&str>) -> Self {
		if let Some(path) = rom_path {
			if let Ok(cart) = Cartridge::load_from_path(path) {
				cart
			} else {
				panic!("Unable to open ROM file at {}", path)
			}
		} else {
			Cartridge {
				path: String::new(),
				mapper_type: MapperType::None,
				rom_type: ROMType::X2_32KiB,
				rom_banks: vec![[0xFF; 0x4000]; 2],
				current_2d_rom_bank: 0x01,
				ram_type: RAMType::None,
				ram_banks: Vec::new(),
				ram_enable: false,
				current_ram_bank: 0x00,
				has_battery: false,
				mbc1_banking_mode: false,
				mbc1_current_rom_banks_upper_bytes: 0x00,
				mbc3_has_rtc: false,
				mbc3_rtc_registers: [Duration::from_secs(0); 2],
				mbc3_rtc_last_update_timestamp: SystemTime::UNIX_EPOCH,
				mbc3_rtc_latch_prev_value: 0xFF,
				mbc3_rtc_is_latched: false,
				mbc3_rtc_is_halted: false,
				mbc5_9th_rom_bank_bit: 0x00
			}
		}
	}
	fn load_from_path(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
		let rom_contents = fs::read(path)?;
		let mapper_type = match rom_contents[0x147] {
			0x01..=0x03 =>	MapperType::MBC1,
			0x05 | 0x06 =>	MapperType::MBC2,
			0x0F..=0x13 =>	MapperType::MBC3,
			0x19..=0x1E => MapperType::MBC5,
			_ => MapperType::None
		};
		let mbc3_has_rtc = match rom_contents[0x147] {
			0x0F | 0x10 => true,
			_ => false
		};
		let has_battery = match rom_contents[0x147] {
			0x03 | 0x06 | 0x09 | 0x0D | 0x0F | 0x10 | 0x13 | 0x1B | 0x1E => true,
			_ => false 
		};
		let rom_type = match rom_contents[0x148] {
			0x01 => ROMType::X4_64KiB,
			0x02 => ROMType::X8_128KiB,
			0x03 => ROMType::X16_256KiB,
			0x04 => ROMType::X32_512KiB,
			0x05 => ROMType::X64_1MiB,
			0x06 => ROMType::X128_2MiB,
			0x07 => ROMType::X256_4MiB,
			0x08 => ROMType::X512_8MiB,
			_ 	=> ROMType::X2_32KiB
		};
		let ram_type = match rom_contents[0x149] {
			0x02 => RAMType::X1_8KiB,
			0x03 => RAMType::X4_32KiB,
			0x04 => RAMType::X16_128KiB,
			0x05 => RAMType::X8_64KiB,
			_ => if mapper_type == MapperType::MBC2 {RAMType::X1_8KiB} else {RAMType::None}
		};
		let mut rom_banks = vec![[0xFF; 0x4000]; match rom_type {
				ROMType::X2_32KiB =>	0x02,
				ROMType::X4_64KiB =>	0x04,
				ROMType::X8_128KiB =>	0x08,
				ROMType::X16_256KiB =>	0x10,
				ROMType::X32_512KiB =>	0x20,
				ROMType::X64_1MiB =>	0x40,
				ROMType::X128_2MiB =>	0x80,
				ROMType::X256_4MiB =>	0x100,
				ROMType::X512_8MiB =>	0x200,
			}
		];
		for (i, byte) in rom_contents.iter().enumerate() {
			rom_banks[i / 0x4000][i % 0x4000] = *byte;
		}
		let mut ram_banks = vec![[0x00; 0x2000]; match ram_type {
				RAMType::None => 0,
				RAMType::X1_8KiB => 1,
				RAMType::X4_32KiB => 4,
				RAMType::X8_64KiB => 8,
				RAMType::X16_128KiB => 16,
			}
		];
		let mut mbc3_rtc_registers = [Duration::from_secs(0); 2];
		let mut mbc3_rtc_is_latched = false;
		let mut mbc3_rtc_is_halted = false;
		let mut mbc3_rtc_last_update_timestamp = SystemTime::UNIX_EPOCH;
		if has_battery {
			let ram_path = path.to_owned() + ".sav";
			let ram_contents = fs::read(ram_path).unwrap_or_default();
			for (i, byte) in ram_contents.iter().enumerate() {
				if !mbc3_has_rtc || i < ram_contents.len() - 0x12 {
					ram_banks[i / 0x2000][i % 0x2000] = *byte;
				}
			}
			if mbc3_has_rtc && ram_contents.len() >= 0x12 {
				mbc3_rtc_registers[0] +=  Duration::from_secs(ram_contents[ram_contents.len() - 0x12] as u64);
				mbc3_rtc_registers[0] +=  Duration::from_secs((ram_contents[ram_contents.len() - 0x11] as u64) * 60);
				mbc3_rtc_registers[0] +=  Duration::from_secs((ram_contents[ram_contents.len() - 0x10] as u64) * 3600);
				mbc3_rtc_registers[0] +=  Duration::from_secs((ram_contents[ram_contents.len() - 0x0F] as u64) * 86400);
				mbc3_rtc_registers[0] +=  Duration::from_secs(((ram_contents[ram_contents.len() - 0x0E] & 1 + ram_contents[ram_contents.len() - 0x0E] >> 7)  as u64) * 22118400);
				mbc3_rtc_is_halted = ram_contents[ram_contents.len() - 0x0E] & (1 << 6) != 0;
				mbc3_rtc_registers[1] +=  Duration::from_secs(ram_contents[ram_contents.len() - 0x0D] as u64);
				mbc3_rtc_registers[1] +=  Duration::from_secs((ram_contents[ram_contents.len() - 0x0C] as u64) * 60);
				mbc3_rtc_registers[1] +=  Duration::from_secs((ram_contents[ram_contents.len() - 0x0B] as u64) * 3600);
				mbc3_rtc_registers[1] +=  Duration::from_secs((ram_contents[ram_contents.len() - 0x0A] as u64) * 86400);
				mbc3_rtc_registers[1] +=  Duration::from_secs(((ram_contents[ram_contents.len() - 0x09] & 1 + ram_contents[ram_contents.len() - 0x09] >> 7)  as u64) * 22118400);
				mbc3_rtc_is_latched = mbc3_rtc_registers[0] != mbc3_rtc_registers[1];
				let mut last_timestamp = 0 as u64;
				for i in 0..=7 {
					last_timestamp += (ram_contents[ram_contents.len() - 0x08 + i] as u64) << (8 * i);
				}
				mbc3_rtc_last_update_timestamp += Duration::from_secs(last_timestamp);
			}
		}
		Ok(Cartridge {
			path: path.to_string(),
			mapper_type,
			rom_type,
			rom_banks,
			current_2d_rom_bank: 0x01,
			ram_type,
			ram_banks,
			ram_enable: false,
			current_ram_bank: 0x00,
			has_battery,
			mbc1_banking_mode: false,
			mbc1_current_rom_banks_upper_bytes: 0x00,
			mbc3_has_rtc,
			mbc3_rtc_last_update_timestamp,
			mbc3_rtc_registers,
			mbc3_rtc_latch_prev_value: 0xFF,
			mbc3_rtc_is_latched,
			mbc3_rtc_is_halted,
			mbc5_9th_rom_bank_bit: 0x00
		})
	}
	pub fn read(&self, address: usize) -> u8 {
		match address {
			0x0000..=0x3FFF	=> self.rom_banks[if self.mbc1_banking_mode {self.mbc1_current_rom_banks_upper_bytes << 5} else {0}][address],
			0x4000..=0x7FFF	=> self.rom_banks[self.mbc5_9th_rom_bank_bit << 9 | self.mbc1_current_rom_banks_upper_bytes << 5 | self.current_2d_rom_bank][address - 0x4000],
			0xA000..=0xBFFF	=> if (self.ram_type == RAMType::None && !(self.mbc3_has_rtc && self.current_ram_bank >= 0x08)) || !self.ram_enable	{0xFF}
								else if self.mbc3_has_rtc && self.current_ram_bank >= 0x08 {
									let elapsed_time = if self.mbc3_rtc_is_halted || self.mbc3_rtc_is_latched {Duration::new(0, 0)} else {self.mbc3_rtc_last_update_timestamp.elapsed().expect("Time ran backwards")};
									match self.current_ram_bank {
										0x08 => {((self.mbc3_rtc_registers[1] + elapsed_time).as_secs() % 60) as u8}
										0x09 => {(((self.mbc3_rtc_registers[1] + elapsed_time).as_secs() / 60) % 60) as u8}
										0x0A => {(((self.mbc3_rtc_registers[1] + elapsed_time).as_secs() / 3600) % 24) as u8}
										0x0B => {(((self.mbc3_rtc_registers[1] + elapsed_time).as_secs() / 86400) % 256) as u8}
										_	 => {
											let nb_256days_periods_elapsed = (self.mbc3_rtc_registers[1] + elapsed_time).as_secs() / 22118400;
											(if nb_256days_periods_elapsed >= 2 {1} else {0} as u8) << 7 | (self.mbc3_rtc_is_halted as u8) << 6 | (nb_256days_periods_elapsed % 2) as u8
										}
									}
								}
								else { self.ram_banks
										[
											if self.mapper_type == MapperType::MBC1 && !self.mbc1_banking_mode {0}
											else {self.current_ram_bank}
										]
										[(address - 0xA000) % if self.mapper_type == MapperType::MBC2 {0x0200} else {0x2000}]
								},
			_ => 0
		}
	}
	pub fn write(&mut self, address: usize, data: u8) {
		match self.mapper_type {
			MapperType::None =>
				match address {
					0x0000..=0x7FFF	=> {},
					0xA000..=0xBFFF	=> if let RAMType::None = self.ram_type {}
										else if self.ram_enable {self.ram_banks[self.current_ram_bank][(address - 0xA000)] = data},
					_ => {}
				}
			MapperType::MBC1 =>
				match address {
					0x0000..=0x1FFF => if data & 0x0F == 0x0A {self.ram_enable = true} else {self.ram_enable = false}
					0x2000..=0x3FFF => {
						let mut data = data & 0x1F;
						if data == 0x00 {data = 0x01}
						self.current_2d_rom_bank = (data as usize) & match self.rom_type {
																		ROMType::X2_32KiB	=> 0x01,
																		ROMType::X4_64KiB	=> 0x03,
																		ROMType::X8_128KiB	=> 0x07,
																		ROMType::X16_256KiB	=> 0x0F,
																		_					=> 0x1F,
        															}
						}
					0x4000..=0x5FFF => {
						self.current_ram_bank = data as usize & match self.ram_type {
							RAMType::X4_32KiB => 0x03,
							_ => 0x00
						};
						self.mbc1_current_rom_banks_upper_bytes = data as usize & match self.rom_type {
							ROMType::X128_2MiB => 0x03,
							ROMType::X64_1MiB => 0x01,
							_ => 0x00					
						}
					}
					0x6000..=0x7FFF => {
						if data & 0x01 != 0x00 && (self.ram_type == RAMType::X4_32KiB || self.rom_type == ROMType::X64_1MiB || self.rom_type == ROMType::X128_2MiB) {
							self.mbc1_banking_mode = true
						}
						else {self.mbc1_banking_mode = false}
					}
					0xA000..=0xBFFF	=> if let RAMType::None = self.ram_type {}
										else if self.ram_enable {self.ram_banks[if !self.mbc1_banking_mode {0} else {self.current_ram_bank}][(address - 0xA000)] = data},
					_ => {}
				}
			MapperType::MBC2 => {
				match address {
					0x0000..=0x3FFF => {
						let mut data = data & 0x0F;
						if address & 0x0100 == 0 {if data == 0x0A {self.ram_enable = true} else {self.ram_enable = false}}
						else {
							if data == 0x00 {data = 0x01}
							self.current_2d_rom_bank = (data as usize) & match self.rom_type {
																			ROMType::X2_32KiB	=> 0x01,
																			ROMType::X4_64KiB	=> 0x03,
																			ROMType::X8_128KiB	=> 0x07,
																			_					=> 0x0F,
																		};
						}
					}
					0xA000..=0xBFFF => if self.ram_enable {self.ram_banks[0][(address - 0xA000) % 0x0200] = data}
					_ => {}
				}
			},
			MapperType::MBC3 => {
				match address {
					0x0000..=0x1FFF => if data & 0x0F == 0x0A {self.ram_enable = true} else {self.ram_enable = false}
					0x2000..=0x2FFF => {
						let mut data = data & 0x7F;
						if data == 0x00 {data = 0x01}
						self.current_2d_rom_bank = (data as usize) & match self.rom_type {
																		ROMType::X2_32KiB	=> 0x01,
																		ROMType::X4_64KiB	=> 0x03,
																		ROMType::X8_128KiB	=> 0x07,
																		ROMType::X16_256KiB	=> 0x0F,
																		ROMType::X32_512KiB	=> 0x1F,
																		ROMType::X64_1MiB	=> 0x3F,
																		_					=> 0x7F
        															}
						}
					0x4000..=0x5FFF => {
							if data <= 0x03 {
								self.current_ram_bank = (data as usize) & match self.ram_type {
																			RAMType::None		=> 0x00,
																			RAMType::X1_8KiB	=> 0x01,
																			_					=> 0x03,
																		}
							}
							else if data >= 0x08 && data <= 0x0C && self.mbc3_has_rtc {self.current_ram_bank = data as usize}
						}
					0x6000..=0x7FFF => {
						if !self.mbc3_has_rtc {return}
						let is_latch_started = self.mbc3_rtc_latch_prev_value == 0x00;
						self.mbc3_rtc_latch_prev_value = data;
						if is_latch_started && data == 0x01 {
							if !self.mbc3_rtc_is_halted {
								self.mbc3_rtc_registers[0] += self.mbc3_rtc_last_update_timestamp.elapsed().expect("Time ran backwards")
							};
							self.mbc3_rtc_last_update_timestamp = SystemTime::now();
							self.mbc3_rtc_registers[1] = self.mbc3_rtc_registers[0];
							self.mbc3_rtc_is_latched = !self.mbc3_rtc_is_latched
						}
					}
					0xA000..=0xBFFF => {
						if self.ram_enable {
							if self.ram_type != RAMType::None && self.current_ram_bank < 0x08  {
								self.ram_banks[self.current_ram_bank][address - 0xA000] = data
							}
							else if self.mbc3_has_rtc && self.current_ram_bank >= 0x08 {
								if !self.mbc3_rtc_is_halted {
									self.mbc3_rtc_registers[0] += self.mbc3_rtc_last_update_timestamp.elapsed().expect("Time ran backwards!");
								}
								self.mbc3_rtc_last_update_timestamp = SystemTime::now();
								match self.current_ram_bank {
									0x08 => {
										self.mbc3_rtc_registers[0] -= Duration::from_secs(self.mbc3_rtc_registers[0].as_secs() % 60);
										self.mbc3_rtc_registers[0] += Duration::from_secs(data as u64);
									}
									0x09 => {
										self.mbc3_rtc_registers[0] -= Duration::from_secs((self.mbc3_rtc_registers[0].as_secs() / 60) % 60);
										self.mbc3_rtc_registers[0] += Duration::from_secs(data as u64 * 60);
									}
									0x0A => {
										self.mbc3_rtc_registers[0] -= Duration::from_secs((self.mbc3_rtc_registers[0].as_secs() / 3600) % 24);
										self.mbc3_rtc_registers[0] += Duration::from_secs(data as u64 * 3600);
									}
									0x0B => {
										self.mbc3_rtc_registers[0] -= Duration::from_secs((self.mbc3_rtc_registers[0].as_secs() / 86400) % 256);
										self.mbc3_rtc_registers[0] += Duration::from_secs(data as u64 * 86400);
									}
									_ => {
										let is_carry = data & (1 << 7) != 0;
										let is_halt = data & (1 << 6) != 0;
										let data = data & 1;
										self.mbc3_rtc_registers[0] = Duration::from_secs(self.mbc3_rtc_registers[0].as_secs() % 44236800);
										self.mbc3_rtc_registers[0] += Duration::from_secs(if is_carry {1} else {0} * 44236800);
										self.mbc3_rtc_registers[0] -= Duration::from_secs((self.mbc3_rtc_registers[0].as_secs() / 22118400) % 2);
										self.mbc3_rtc_registers[0] += Duration::from_secs(data as u64 * 22118400);
										if is_halt {self.mbc3_rtc_is_halted = true} else {self.mbc3_rtc_is_halted = false}
									}
								}
								if !self.mbc3_rtc_is_latched {
									self.mbc3_rtc_registers[1] = self.mbc3_rtc_registers[0];
								}

							}
						}
					}
					_ => {}
				}
			}
			MapperType::MBC5 => {
				match address {
					0x0000..=0x1FFF => if data & 0x0F == 0x0A {self.ram_enable = true} else {self.ram_enable = false}
					0x2000..=0x2FFF => {
						self.current_2d_rom_bank = (data as usize) & match self.rom_type {
																		ROMType::X2_32KiB	=> 0x01,
																		ROMType::X4_64KiB	=> 0x03,
																		ROMType::X8_128KiB	=> 0x07,
																		ROMType::X16_256KiB	=> 0x0F,
																		ROMType::X32_512KiB	=> 0x1F,
																		ROMType::X64_1MiB	=> 0x3F,
																		ROMType::X128_2MiB	=> 0x7F,
																		_					=> 0xFF
        															}
						}
					0x3000..=0x3FFF => {
						self.mbc5_9th_rom_bank_bit = if data & 0x01 != 0 && self.rom_type == ROMType::X512_8MiB {1} else {0} 
					}
					0x4000..=0x5FFF => {
						if data <= 0x0F {
							self.current_ram_bank = (data as usize) & match self.ram_type {
																		RAMType::None		=> 0x00,
																		RAMType::X1_8KiB	=> 0x01,
																		RAMType::X4_32KiB	=> 0x03,
																		RAMType::X8_64KiB	=> 0x07,
																		RAMType::X16_128KiB	=> 0x0F
																	}
						}
					}
					0xA000..=0xBFFF => if self.ram_type != RAMType::None && self.ram_enable {self.ram_banks[self.current_ram_bank][address - 0xA000] = data}
					_ => {}
				}
			},
		}
	}
	pub fn _debug_insert_cart_logo(&mut self) {
		let logo_data : [u8; 48] = [
			0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 
			0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 
			0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 
			0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E
		];
		for (i, byte) in logo_data.iter().enumerate() {
			self.rom_banks[0][0x104 + i] = *byte;
		}
		self.rom_banks[0][0x134] = 0xE7;
		for i in 0x135..0x14E {
			self.rom_banks[0][i] = 0x00;
		}
	}
}