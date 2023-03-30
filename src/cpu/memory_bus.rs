use std::fmt::Debug;

pub struct MemoryBus {
	pub memory: [u8; 0xFFFF]
}

impl MemoryBus {
	pub fn read_byte(&self, address: u16) -> u8 {
		self.memory[address as usize]
	}
}

impl Debug for MemoryBus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemoryBus")/*.field("memory", &self.memory)*/.finish()
    }
}