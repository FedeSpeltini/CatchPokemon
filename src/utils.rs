// src/utils.rs

use std::fs;
use serde_json;
use crate::pokemon::Pokemon;

const FILENAME: &str = "captured_pokemon.json";

pub fn capture_pokemon(pokemon: &Pokemon) -> bool {
    let mut captured_pokemon = load_captured_pokemon();
    if !captured_pokemon.iter().any(|p| p.name == pokemon.name) {
        captured_pokemon.push(pokemon.clone()); // Clona pokemon aquí
        save_captured_pokemon(&captured_pokemon);
        true
    } else {
        false
    }
}

pub fn load_captured_pokemon() -> Vec<Pokemon> {
    match fs::read_to_string(FILENAME) {
        Ok(contents) => {
            serde_json::from_str(&contents).unwrap_or_default()
        },
        Err(_) => Vec::new(),
    }
}

pub fn save_captured_pokemon(pokemon_list: &[Pokemon]) {
    let json_data = serde_json::to_string_pretty(pokemon_list).unwrap();
    fs::write(FILENAME, json_data).unwrap();
}
