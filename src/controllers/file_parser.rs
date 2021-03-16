use crate::models::character_set::CharacterSet;
use byteorder::{BigEndian, ByteOrder, LittleEndian};
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

static mut CHARACTERSET: CharacterSet = CharacterSet { map: None };

pub fn get_block_file(f: &mut File, offset: u64, size: usize) -> Vec<u8> {
	let mut buffer = vec![Default::default(); size];

	f.seek(SeekFrom::Start(offset)).expect("Couldn't seek");
	f.read(&mut buffer).expect("Couldn't err the whole block");

	buffer
}

pub fn get_block_buffer(b: &Vec<u8>, offset: usize, size: usize) -> Vec<u8> {
	let slice = &b[offset..(offset + size)];
	let mut x = vec![0; size];
	x[..size].clone_from_slice(slice);
	x
}

pub fn get_string_buffer(b: &Vec<u8>, offset: usize, size: usize) -> String {
	let character_set = unsafe { CHARACTERSET.get() };
	let buffer = get_block_buffer(b, offset, size);
	let mut to_return = String::new();

	for num in buffer {
		if num == 255 {
			break;
		}
		to_return.push_str(character_set.get(&num).unwrap_or(&String::from(".?.")));
	}

	to_return
}

pub fn get_u8_buffer(b: &Vec<u8>, offset: usize) -> u8 {
	b[offset]
}

pub fn get_u16_little_buffer(b: &Vec<u8>, offset: usize) -> u16 {
	LittleEndian::read_u16(&b[offset..(offset + 2)])
}

pub fn get_u16_big_buffer(b: &Vec<u8>, offset: usize) -> u16 {
	BigEndian::read_u16(&b[offset..(offset + 2)])
}

pub fn get_u32_little_buffer(b: &Vec<u8>, offset: usize) -> u32 {
	LittleEndian::read_u32(&b[offset..(offset + 4)])
}

pub fn get_u32_big_buffer(b: &Vec<u8>, offset: usize) -> u32 {
	BigEndian::read_u32(&b[offset..(offset + 4)])
}
