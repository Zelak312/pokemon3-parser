mod controllers;
mod models;

use models::file_save::FileSave;
use std::fs::File;

fn main() {
    let mut f =
        File::open("../../testfiles/PokemonLatestSave.srm").expect("Couldn't find the file save");
    let test = FileSave::new(&mut f);
    println!("{}", serde_json::to_string_pretty(&test).unwrap());
}
