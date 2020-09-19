use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Monster {
    hd: Option<Vec<u8>>,
    hd_mod: Option<i8>,
    treasure: Option<String>,
    treasure_mod: Option<String>,
    alignment: Option<Vec<String>>,
    powers: Option<Vec<String>>,
    personality: Option<Vec<String>>,
}

fn main() {
    let file = std::fs::read_to_string("data.toml").unwrap();
    let monsters: HashMap<String, Monster> = toml::from_str(&file).unwrap();
    println!("{:#?}", monsters);
}