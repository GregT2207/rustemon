mod data;
mod pokemath;
mod pokemon;

use colored::Colorize;
use data::{pokemon as pokemon_data, pokemon_index};
use pokemon::Pokemon;
use rand::Rng;
use std::collections::HashMap;
use std::io;

const USER_POKEMON_MAX: usize = 4;
const ENEMY_POKEMON_MAX: usize = 2;

fn main() {
    print_pokeball_message("Prepare for battle!");

    let pokemon_data = pokemon_data();

    let mut user_pokemon = build_user_team(&pokemon_data);
    let mut enemy_pokemon = build_enemy_team(&pokemon_data);

    // match squirtle.attack(0, &mut charmander) {
    //     Ok(true) => println!("Squirtle hit Charmander!"),
    //     Ok(false) => println!("Oh no! Squirtle missed..."),
    //     Err(err) => eprintln!("Attack failed: {}", err),
    // }
}

fn print_pokeball_message(message: &str) {
    println!("");
    println!("{}", "          ██████████          ".red());
    println!("{}", "      ██████████████████      ".red());
    println!("{}", "    ██████████████████████    ".red());
    println!("{}", "  ██████████████████████████  ".red());
    println!("{}", "  ████████ Rustémon ████████  ".red());
    println!("{}", "██████████████████████████████".red());
    println!("{}", "████████████      ████████████".red());
    println!("{}", "              ⚪               ".white());
    println!("{}", "████████████      ████████████".white());
    println!("{}", "██████████████████████████████".white());
    println!("{}", format!("████ {:^20} ████", message).white());
    println!("{}", "  ██████████████████████████  ".white());
    println!("{}", "    ██████████████████████    ".white());
    println!("{}", "      ██████████████████      ".white());
    println!("{}", "          ██████████          ".white());
    println!("");
}

fn build_user_team(pokemon_data: &HashMap<u32, Pokemon>) -> Vec<Pokemon> {
    let mut user_pokemon: Vec<Pokemon> = Vec::new();

    let pokemon_index = pokemon_index();
    let mut sorted_pokemon_index: Vec<_> = pokemon_index.iter().collect();
    sorted_pokemon_index.sort_by_key(|(_name, index)| *index);

    println!("Choose a Rustémon! (Enter a number)");
    for (name, id) in &sorted_pokemon_index {
        println!("{}: {}", id, name.to_uppercase())
    }

    while user_pokemon.len() < USER_POKEMON_MAX {
        let mut pokemon_number = String::new();
        if let Err(err) = io::stdin().read_line(&mut pokemon_number) {
            eprintln!("Couldn't read Rustémon choice: {}", err)
        }

        match pokemon_number.trim().parse::<u32>() {
            Ok(pokemon_number) => {
                match pokemon_data.get(&pokemon_number) {
                    Some(new_pokemon) => {
                        user_pokemon.push(new_pokemon.clone());
                        println!(
                            "Nice choice! Added {} to your team. Choose another...",
                            new_pokemon.name
                        );
                    }
                    None => eprintln!("Rustémon number {} doesn't exist!", pokemon_number),
                };
            }
            Err(err) => {
                eprintln!("Please enter a valid number: {}", err);
            }
        }
    }

    user_pokemon
}

fn build_enemy_team(pokemon_data: &HashMap<u32, Pokemon>) -> Vec<Pokemon> {
    let mut enemy_pokemon: Vec<Pokemon> = Vec::new();

    let random_nums: Vec<u32> = (0..ENEMY_POKEMON_MAX)
        .map(|_| rand::rng().random_range(1..=pokemon_data.len() as u32))
        .collect();

    for random_num in random_nums {
        if let Some(pokemon) = pokemon_data.get(&random_num) {
            enemy_pokemon.push(pokemon.clone());
        }
    }

    enemy_pokemon
}
