use super::inventory::Inventory;
use super::trainer::Trainer;
use crate::controllers::file_parser;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct GameSave {
    pub trainer: Trainer,
    pub inventory: Inventory,
    pub rival_name: String,
    pub checksum: u16,
    pub save_index: u32,
}

impl GameSave {
    const NBR_SECTION: usize = 14;
    const SECTION_SIZE: usize = 0x1000;

    // ID -> u16
    const ID_OFFSET: usize = 0xFF4;

    // Data
    const DATA_OFFSET: usize = 0x0;
    const DATA_SIZE: usize = 0xF80;

    // Rival Name
    const RIVAL_NAME_OFFSET: usize = 0xBCC;
    const RIVAL_NAME_SIZE: usize = 0x8;

    // Save Index -> u32
    const SAVEINDEX_OFFSET: usize = 0xFFC;

    // Checksum -> u16
    const CHECKSUM_OFFSET: usize = 0x1000;

    pub fn new(b: Vec<u8>) -> GameSave {
        let mut game_save = GameSave::default();

        for i in 0..GameSave::NBR_SECTION {
            let offset = i * GameSave::SECTION_SIZE;
            let buffer = file_parser::get_block_buffer(&b, offset, GameSave::SECTION_SIZE);

            // ID
            let id = file_parser::get_u16_little_buffer(&buffer, GameSave::ID_OFFSET);

            // Data
            let data =
                file_parser::get_block_buffer(&buffer, GameSave::DATA_OFFSET, GameSave::DATA_SIZE);

            // Checksum
            game_save.checksum = file_parser::get_u16_little_buffer(&b, GameSave::CHECKSUM_OFFSET);

            // Parse data depending on id
            if id == 0 {
                game_save.trainer = Trainer::new(data);
            } else if id == 1 {
                game_save.inventory = Inventory::new(data, game_save.trainer.security_key);
            } else if id == 4 {
                game_save.rival_name = file_parser::get_string_buffer(
                    &buffer,
                    GameSave::RIVAL_NAME_OFFSET,
                    GameSave::RIVAL_NAME_SIZE,
                );
            }

            // Save Index
            if i == 0 {
                game_save.save_index =
                    file_parser::get_u32_little_buffer(&buffer, GameSave::SAVEINDEX_OFFSET);
            }
        }

        game_save
    }
}

impl fmt::Display for GameSave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "GameSave (trainer: Trainer, rival_name: {}, save_index: {})",
            self.rival_name, self.save_index
        )
    }
}
