pub mod tileset;

use std::fs;
use serde_json::from_str;

fn main() {
  let file_path = String::from("./sample-data/tileset-file/tileset.json");
  let tileset_file_result = fs::read_to_string(file_path);
  let tileset_file_text = match tileset_file_result {
    Ok(file) => file,
    Err(err) => panic!("Problem opening the file {err:?}")
  };


  let json_tileset_result = from_str::<tileset::Tileset>(&tileset_file_text);
  let json_tileset = match json_tileset_result {
    Ok(tileset) => tileset,
    Err(err) => panic!("Read tileset.json Error {err:?}")
  };

  let tileset_version = json_tileset.asset.version;

  println!("Tileset version: {}", tileset_version);
  println!("Finish.");
}