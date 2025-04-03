mod data;
mod pokemath;
mod pokemon;

use colored::Colorize;
use core::time;
use data::{pokemon as pokemon_data, pokemon_index};
use pokemon::{Pokemon, PoketypeColor};
use rand::Rng;
use std::collections::HashMap;
use std::io::{self, Write, stdout};
use std::thread::sleep;

const USER_POKEMON_MAX: usize = 4;
const ENEMY_POKEMON_MAX: usize = 2;

fn main() {
    print!("\n\n\n\n\n\n");
    print_pokeball_message("Prepare for battle!");

    let pokemon_data = pokemon_data();

    let mut user_pokemon = build_user_team(&pokemon_data);
    let mut enemy_pokemon = build_enemy_team(&pokemon_data);

    let user_current_pokemon = &mut user_pokemon[0];
    let enemy_current_pokemon = &mut enemy_pokemon[0];

    sleep_print();
    println!(
        "Enemy trainer sent out {}!",
        enemy_current_pokemon.colored_name()
    );
    sleep_print();
    println!("You sent out {}!", user_current_pokemon.colored_name());

    loop {
        // User selects a move
        sleep_print();
        println!(
            "What move should {} use? (Enter a number)",
            user_current_pokemon.colored_name()
        );
        for (index, pokemove) in user_current_pokemon.pokemoves.iter().enumerate() {
            if let Some(pokemove) = pokemove {
                println!("{}: {}", index + 1, pokemove.name)
            }
        }

        let mut pokemove_number = String::new();
        if let Err(err) = io::stdin().read_line(&mut pokemove_number) {
            eprintln!("Couldn't read move choice: {}", err);
            continue;
        }

        // User attacks
        match pokemove_number.trim().parse::<u8>() {
            Ok(pokemove_number) => {
                sleep_print();

                if !try_attack(
                    user_current_pokemon,
                    enemy_current_pokemon,
                    pokemove_number - 1,
                ) {
                    continue;
                }
            }
            Err(err) => {
                eprintln!("Please enter a valid number: {}", err);
                continue;
            }
        }

        // Enemy attacks
        let move_index = loop {
            let random_num = rand::rng().random_range(0..4);
            if enemy_current_pokemon.pokemoves[random_num].is_some() {
                break random_num as u8;
            }
        };

        sleep_print();
        if !try_attack(enemy_current_pokemon, user_current_pokemon, move_index) {
            continue;
        }
    }
}

fn sleep_print() {
    let delay = time::Duration::from_millis(10);

    println!("\n>");
    for i in 2..102 {
        print!("\x1B[1A\x1B[2K");
        stdout().flush().unwrap();

        println!("{}>", "-".repeat(i));
        stdout().flush().unwrap();

        sleep(delay);
    }
    println!("");
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
        let &pokemon = pokemon().
        println!("{}: {}", id, name)
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
                            "Nice choice! Added {} to your team{}",
                            new_pokemon.colored_name(),
                            if user_pokemon.len() < USER_POKEMON_MAX {
                                ". Choose another..."
                            } else {
                                ""
                            }
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

fn try_attack(attacker: &mut Pokemon, target: &mut Pokemon, move_index: u8) -> bool {
    let attacking_move = attacker.pokemove_by_index(move_index).unwrap();

    let attack_successful = match attacker.attack(&attacking_move, target) {
        Ok(true) => {
            println!(
                "{} used {} on {} ({} / {})!",
                attacker.colored_name(),
                attacking_move.colored_name(),
                target.colored_name(),
                target.current_hp,
                target.max_hp
            );
            true
        }
        Ok(false) => {
            println!("Oh no! {} missed...", attacker.colored_name());
            true
        }
        Err(err) => {
            eprintln!("Attack failed: {}", err);
            false
        }
    };

    attack_successful
}
