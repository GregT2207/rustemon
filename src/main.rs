mod data;
mod pokemath;
mod pokemon;

use data::pokemon as pokemon_data;
use pokemon::Pokemon;
use std::io;

fn main() {
    println!("Welcome to Rustémon!");
    println!("=========================");

    let pokemon_data = pokemon_data();

    let mut pokemon: Vec<Pokemon> = Vec::new();
    while pokemon.len() < 4 {
        println!(
            "Choose a Rustémon! ({})",
            pokemon_data
                .values()
                .map(|p| p.name.clone())
                .collect::<Vec<_>>()
                .join(" | ")
        );

        let mut pokemon_name = String::new();
        if let Err(err) = io::stdin().read_line(&mut pokemon_name) {
            eprintln!("Couldn't read Rustémon name: {}", err)
        }

        match pokemon_data.get(pokemon_name.to_lowercase().trim()) {
            Some(new_pokemon) => pokemon.push(new_pokemon.clone()),
            None => eprintln!("Rustémon {} doesn't exist!", pokemon_name.trim()),
        };
    }

    // match squirtle.attack(0, &mut charmander) {
    //     Ok(true) => println!("Squirtle hit Charmander!"),
    //     Ok(false) => println!("Oh no! Squirtle missed..."),
    //     Err(err) => eprintln!("Attack failed: {}", err),
    // }

    // match charmander.attack(1, &mut squirtle) {
    //     Ok(true) => println!("Charmander hit Squirtle!"),
    //     Ok(false) => println!("Oh no! Charmander missed..."),
    //     Err(err) => eprintln!("Attack failed: {}", err),
    // }
}
