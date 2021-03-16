mod controllers;
mod models;

use models::file_save::FileSave;
use std::fs::File;

fn main() {
    let mut f =
        File::open("../../testfiles/PokemonLatestSave.srm").expect("Couldn't find the file save");
    let test = FileSave::new(&mut f);
    let recent = test.recent.unwrap();
    println!("{}", recent);
    println!("{}", recent.trainer);
    println!("{}", recent.trainer.time_played);
}
