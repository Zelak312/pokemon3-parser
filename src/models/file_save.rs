use super::game_save::GameSave;
use crate::controllers::file_parser;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::File;

#[derive(Default, Serialize, Deserialize)]
pub struct FileSave {
    pub blocks: Vec<GameSave>,
    pub recent: Option<GameSave>,
}

impl FileSave {
    const BLOCK_SIZE: u16 = 0xE000;
    const NBR_BLOCK: u8 = 2;

    pub fn new(f: &mut File) -> FileSave {
        let mut save_file = FileSave::default();

        for i in 0..FileSave::NBR_BLOCK {
            let offset = i as u16 * FileSave::BLOCK_SIZE;
            let buffer =
                file_parser::get_block_file(f, offset as u64, FileSave::BLOCK_SIZE as usize);

            let game_save = GameSave::new(buffer);
            save_file.blocks.push(game_save.clone());
            if save_file.recent.is_none() {
                save_file.recent.replace(game_save);
            } else if save_file.recent.as_ref().unwrap().save_index < game_save.save_index {
                save_file.recent.replace(game_save);
            }
        }

        save_file
    }
}

impl fmt::Display for FileSave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.blocks.len())
    }
}
