// src/main.rs

mod api;
mod pokemon;
mod utils;

use std::io;
use tokio::runtime::Runtime;
use crate::pokemon::Pokemon;

fn main() {
    println!("Bienvenido a la aplicación de captura de Pokémon!");

    let mut rt = Runtime::new().unwrap();
    loop {
        println!("\nMenu:");
        println!("1. Capturar Pokémon");
        println!("2. Ver Pokémon Capturados");
        println!("3. Salir");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error al leer la entrada.");

        match input.trim() {
            "1" => {
                let pokemon_result = rt.block_on(api::get_random_pokemon()).expect("Error al obtener un Pokémon.");
                match pokemon_result {
                    Some(pokemon) => {
                        println!("¡Has encontrado a {}!", pokemon.name);
                        let captured = utils::capture_pokemon(&pokemon);
                        if captured {
                            println!("¡Has capturado a {}!", pokemon.name);
                        } else {
                            println!("¡{} se ha escapado!", pokemon.name);
                        }
                    },
                    None => {
                        println!("No se encontró ningún Pokémon esta vez. Intenta de nuevo.");
                    }
                }
            },
            "2" => {
                // Lógica para ver Pokémon capturados
                let captured_pokemon = utils::load_captured_pokemon();
                if captured_pokemon.is_empty() {
                    println!("No has capturado ningún Pokémon aún.");
                } else {
                    println!("Pokémon Capturados:");
                    for pokemon in captured_pokemon {
                        println!("ID: {}, Nombre: {}, Tipo: {}, ID Pokémon: {}", pokemon.id, pokemon.name, pokemon.pokemon_type, pokemon.pokemon_id);
                    }
                }
            },
            "3" => {
                println!("Saliendo del programa.");
                break;
            },
            _ => {
                println!("Opción no válida. Por favor, elige una opción válida.");
            }
        }
    }
}
