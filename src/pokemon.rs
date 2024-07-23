// src/pokemon.rs

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Pokemon {
    pub id: u32,
    pub name: String,
    pub pokemon_type: String,
    pub pokemon_id: u32,

}
