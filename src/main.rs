mod data;
mod pokemath;
mod pokemon;
mod trainer;

use colored::Colorize;
use data::pokemon as pokemon_data;
use pokemon::{Pokemon, Pokemove, PoketypeColor};
use rand::Rng;
use std::collections::HashMap;
use std::io::{self, Write, stdout};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use trainer::{Trainer, TrainerKind};

const USER_POKEMON_MAX: usize = 4;
const ENEMY_POKEMON_MAX: usize = 2;

fn main() {
    print!("\n\n\n\n\n\n");
    print_pokeball_message("Prepare for battle!");

    let pokemon_data = pokemon_data();

    let mut user = Trainer::new(TrainerKind::User, "Red", build_user_team(&pokemon_data));
    let mut enemy = Trainer::new(TrainerKind::Enemy, "Blue", build_enemy_team(&pokemon_data));

    loop {
        if let Some(user_current_pokemon) = user.current_pokemon() {
            user.attack(user_move(user_current_pokemon).as_ref(), &mut enemy);
        }

        if let Some(enemy_current_pokemon) = enemy.current_pokemon() {
            enemy.attack(enemy_move(enemy_current_pokemon).as_ref(), &mut user);
        }

        if false {
            break;
        }
    }
}

fn sleep_print() {
    let delay = Duration::from_millis(10);

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

    let mut sorted_pokemon_available: Vec<_> = pokemon_data.iter().collect();
    sorted_pokemon_available.sort_by_key(|entry| *entry.0);

    println!("Choose a Rustémon! (Enter a number)");
    for (id, pokemon) in sorted_pokemon_available {
        sleep(Duration::from_millis(20));
        println!("{}: {}", id, pokemon.colored_name())
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

fn user_move(pokemon: &Pokemon) -> Arc<Pokemove> {
    sleep_print();
    println!(
        "What move should {} use? (Enter a number)",
        pokemon.colored_name()
    );

    for (index, pokemove) in pokemon.pokemoves.iter().enumerate() {
        if let Some(pokemove) = pokemove {
            println!("{}: {}", index + 1, pokemove.name);
        }
    }

    loop {
        let mut pokemove_number = String::new();
        if let Err(err) = io::stdin().read_line(&mut pokemove_number) {
            eprintln!("Couldn't read move choice: {}", err);
            continue;
        }

        let pokemove_number = match pokemove_number.trim().parse::<usize>() {
            Ok(n) => n,
            Err(err) => {
                eprintln!("Please enter a valid number: {}", err);
                continue;
            }
        };

        let pokemove = match pokemon.pokemoves.get(pokemove_number - 1) {
            Some(pokemove) => pokemove,
            None => {
                eprintln!("Move number {} doesn't exist", pokemove_number);
                continue;
            }
        };

        match pokemove {
            Some(pokemove) => return pokemove.clone(),
            None => eprintln!("Move number {} doesn't exist", pokemove_number),
        }
    }
}

fn enemy_move(pokemon: &Pokemon) -> Arc<Pokemove> {
    loop {
        let random_num = rand::rng().random_range(0..4);
        if let Some(pokemove) = pokemon.pokemoves[random_num].clone() {
            return pokemove;
        }
    }
}
