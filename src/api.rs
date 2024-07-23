// src/api.rs

use reqwest::Error;
use rand::{self, Rng};
use crate::pokemon::Pokemon;

pub async fn get_random_pokemon() -> Result<Option<Pokemon>, Error> {
    let random_id: u32 = rand::thread_rng().gen_range(1..=150); // Genera un número aleatorio entre 1 y 150 inclusive
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", random_id);


    let response = reqwest::get(&url).await?;
    if response.status().is_success() {
        let pokemon_data: serde_json::Value = response.json().await?;
        let id = random_id; // Id interno
        let name = pokemon_data["name"].as_str().unwrap_or_default().to_owned();
        let pokemon_type = get_pokemon_type(&pokemon_data);
        let pokemon_id = pokemon_data["id"].as_u64().unwrap_or(0) as u32;

        let pokemon = Pokemon {
            id,
            name,
            pokemon_type,
            pokemon_id,
        };

        Ok(Some(pokemon))
    } else {
        Ok(None)
    }
}

fn get_pokemon_type(pokemon_data: &serde_json::Value) -> String {
    // Lógica para obtener el tipo de Pokémon
    // Puedes implementar la lógica según la estructura de la API de Pokémon que estás usando
    // Aquí un ejemplo básico
    pokemon_data["types"][0]["type"]["name"].as_str().unwrap_or_default().to_owned()
}
