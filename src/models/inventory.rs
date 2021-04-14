use crate::controllers::{cipher::Cipher, file_parser};
use serde::{Deserialize, Serialize};

use super::item::Item;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub team_size: u32,
    pub money: u32,
    pub coins: u16,

    pub pc_items: Vec<Item>,
    pub pocket_items: Vec<Item>,
    pub key_items: Vec<Item>,
    pub poke_balls: Vec<Item>,
    pub tm_case: Vec<Item>,
    pub berry_items: Vec<Item>,
}

impl Inventory {
    // Team -> u32
    const TEAM_SIZE_OFFSET: usize = 0x34;

    // Money -> u32
    const MONEY_OFFSET: usize = 0x290;

    // Coins -> u16
    const COINS_OFFSET: usize = 0x294;

    // Items
    // Pc Items
    const PC_ITEMS_OFFSET: usize = 0x0298;
    const PC_ITEMS_SIZE: usize = 120;
    const PC_ITEMS_COUNT: usize = 30;

    // Pocket Items
    const POCKET_ITEMS_OFFSET: usize = 0x0310;
    const POCKET_ITEMS_SIZE: usize = 168;
    const POCKET_ITEMS_COUNT: usize = 42;

    // Key Items
    const KEY_ITEMS_OFFSET: usize = 0x03B8;
    const KEY_ITEMS_SIZE: usize = 120;
    const KEY_ITEMS_COUNT: usize = 30;

    // Poke Balls
    // TODO: make a pokeball class
    const POKE_BALLS_OFFSET: usize = 0x0430;
    const POKE_BALLS_SIZE: usize = 52;
    const POKE_BALLS_COUNT: usize = 13;

    // Tm Case
    const TM_CASE_OFFSET: usize = 0x0464;
    const TM_CASE_SIZE: usize = 232;
    const TM_CASE_COUNT: usize = 58;

    // Berry Items
    const BERRY_ITEMS_OFFSET: usize = 0x054C;
    const BERRY_ITEMS_SIZE: usize = 172;
    const BERRY_ITEMS_COUNT: usize = 43;

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

        inventory.pc_items = Inventory::get_items(
            &b,
            key,
            Inventory::PC_ITEMS_OFFSET,
            Inventory::PC_ITEMS_SIZE,
            Inventory::PC_ITEMS_COUNT,
            Some(true),
        );

        inventory.pocket_items = Inventory::get_items(
            &b,
            key,
            Inventory::POCKET_ITEMS_OFFSET,
            Inventory::POCKET_ITEMS_SIZE,
            Inventory::POCKET_ITEMS_COUNT,
            None,
        );

        inventory.key_items = Inventory::get_items(
            &b,
            key,
            Inventory::KEY_ITEMS_OFFSET,
            Inventory::KEY_ITEMS_SIZE,
            Inventory::KEY_ITEMS_COUNT,
            None,
        );

        inventory.poke_balls = Inventory::get_items(
            &b,
            key,
            Inventory::POKE_BALLS_OFFSET,
            Inventory::POKE_BALLS_SIZE,
            Inventory::POKE_BALLS_COUNT,
            None,
        );

        inventory.tm_case = Inventory::get_items(
            &b,
            key,
            Inventory::TM_CASE_OFFSET,
            Inventory::TM_CASE_SIZE,
            Inventory::TM_CASE_COUNT,
            None,
        );

        inventory.berry_items = Inventory::get_items(
            &b,
            key,
            Inventory::BERRY_ITEMS_OFFSET,
            Inventory::BERRY_ITEMS_SIZE,
            Inventory::BERRY_ITEMS_COUNT,
            None,
        );

        inventory
    }

    pub fn get_items(
        b: &Vec<u8>,
        key: u32,
        offset: usize,
        size: usize,
        count: usize,
        cipher: Option<bool>,
    ) -> Vec<Item> {
        let no_cipher = cipher.unwrap_or(false);
        let mut items = Vec::new();
        let data_block = file_parser::get_block_buffer(&b, offset, size);

        for i in 0..count {
            let base_offset = i * 0x4; // Block offset
            let item_id = file_parser::get_u16_little_buffer(&data_block, base_offset);
            let mut item_quant = file_parser::get_u16_little_buffer(&data_block, base_offset + 0x2);

            if !no_cipher {
                item_quant = Cipher::run_switch(key, item_quant as u32, Some(true)) as u16;
            }

            if item_id == 0 {
                // Not sure of this
                break;
            }

            items.push(Item::new(item_id, item_quant));
        }

        items
    }
}
