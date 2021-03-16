pub struct Cipher {}

impl Cipher {
	pub fn run(key: u32, data: u32) -> u32 {
		key ^ data
	}

	pub fn runSwitch(key: u32, data: u32, high: Option<bool>) -> u32 {
		let bool_high = high.unwrap_or(false);
		if !bool_high {
			(key >> 16) ^ (data & 0xffff)
		} else {
			(key & 0xffff) ^ (data & 0xffff)
		}
	}

	pub fn runSection(key: u32, data: u32, section: u32) -> u32 {
		((key >> (data * section)) & 0xff) ^ (data & 0xff)
	}
}
