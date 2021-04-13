use crate::controllers::file_parser;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct TimePlayed {
    hours: u16,
    minutes: u8,
    seconds: u8,
}

impl TimePlayed {
    // Hours -> u16
    const HOURS_OFFSET: usize = 0x0;

    // Minutes -> u8
    const MINUTES_OFFSET: usize = 0x2;

    // Seconds -> u8
    const SECONDS_OFFSET: usize = 0x3;

    pub fn new(b: Vec<u8>) -> TimePlayed {
        let mut time_played = TimePlayed::default();

        time_played.hours = file_parser::get_u16_little_buffer(&b, TimePlayed::HOURS_OFFSET);
        time_played.minutes = file_parser::get_u8_buffer(&b, TimePlayed::MINUTES_OFFSET);
        time_played.seconds = file_parser::get_u8_buffer(&b, TimePlayed::SECONDS_OFFSET);

        time_played
    }
}

impl fmt::Display for TimePlayed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TimePlayed (hours: {}, minutes: {}, seconds: {})",
            self.hours, self.minutes, self.seconds
        )
    }
}
