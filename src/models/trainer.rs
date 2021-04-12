use super::time_played::TimePlayed;
use crate::controllers::file_parser;
use std::fmt;

#[derive(Debug, Clone)]
pub enum TrainerGender {
    Male,
    Female,
}

impl TrainerGender {
    pub fn from_u8(value: u8) -> TrainerGender {
        match value {
            0 => TrainerGender::Male,
            1 => TrainerGender::Female,
            _ => TrainerGender::Male,
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            TrainerGender::Male => "Male",
            TrainerGender::Female => "Female",
        }
    }
}

impl Default for TrainerGender {
    fn default() -> Self {
        TrainerGender::Male
    }
}

#[derive(Default, Clone)]
pub struct Trainer {
    pub id: u32,
    pub name: String,
    pub gender: TrainerGender, // u8
    pub time_played: TimePlayed,
    pub game_code: u32,
    pub security_key: u32,
}

impl Trainer {
    // Id -> u32
    const ID_OFFSET: usize = 0xA;

    // Name -> string
    const NAME_OFFSET: usize = 0x0;
    const NAME_SIZE: usize = 0x7;

    // Gender -> u8
    const GENDER_OFFSET: usize = 0x8;

    // Time Played -> time
    const TIME_PLAYED_OFFSET: usize = 0xE;
    const TIME_PLAYED_SIZE: usize = 0x5;

    // Game Code -> u32
    const GAME_CODE_OFFSET: usize = 0xAC;

    // Security Key -> u32
    const SECURITY_KEY_OFFSET: usize = 0xAF8;

    pub fn new(b: Vec<u8>) -> Trainer {
        let mut trainer = Trainer::default();

        trainer.id = file_parser::get_u32_little_buffer(&b, Trainer::ID_OFFSET);
        trainer.name = file_parser::get_string_buffer(&b, Trainer::NAME_OFFSET, Trainer::NAME_SIZE);
        trainer.gender =
            TrainerGender::from_u8(file_parser::get_u8_buffer(&b, Trainer::GENDER_OFFSET));
        trainer.time_played = TimePlayed::new(file_parser::get_block_buffer(
            &b,
            Trainer::TIME_PLAYED_OFFSET,
            Trainer::TIME_PLAYED_SIZE,
        ));
        trainer.game_code = file_parser::get_u32_little_buffer(&b, Trainer::GAME_CODE_OFFSET);
        trainer.security_key = file_parser::get_u32_little_buffer(&b, Trainer::SECURITY_KEY_OFFSET);
        trainer
    }
}

impl fmt::Display for Trainer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
			f,
			"Trainer (id: {}, name: {}, gender: {}, time_played: TimePlayed, game_code: {}, security_key: {})",
			self.id, self.name, self.gender.to_string(), self.game_code, self.security_key
		)
    }
}
