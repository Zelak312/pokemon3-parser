use crate::controllers::cipher::Cipher;
use crate::controllers::file_parser;

#[derive(Clone)]
pub struct Inventory {
	pub team_size: u32,
	pub money: u32,
	pub coins: u16,
}

impl Inventory {
	// Team -> u32
	const TEAM_SIZE_OFFSET: usize = 0x34;

	// Money -> u32
	const MONEY_OFFSET: usize = 0x290;

	// Coins -> u16
	const COINS_OFFSET: usize = 0x294;

	pub fn default() -> Inventory {
		Inventory {
			team_size: 0,
			money: 0,
			coins: 0,
		}
	}

	pub fn new(b: Vec<u8>, key: u32) -> Inventory {
		let mut inventory = Inventory::default();

		inventory.team_size = file_parser::get_u32_little_buffer(&b, Inventory::TEAM_SIZE_OFFSET);
		inventory.money = Cipher::run(
			key,
			file_parser::get_u32_little_buffer(&b, Inventory::MONEY_OFFSET),
		);
		inventory.coins = Cipher::run(
			key,
			file_parser::get_u16_little_buffer(&b, Inventory::COINS_OFFSET) as u32,
		) as u16;

		inventory
	}
}

// impl fmt::Display for GameSave {
// 	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
// 		write!(
// 			f,
// 			"GameSave (trainer: Trainer, rival_name: {}, save_index: {})",
// 			self.rival_name, self.save_index
// 		)
// 	}
// }
