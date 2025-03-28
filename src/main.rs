mod pokemath;
mod pokemon;

use pokemath::Percentage;
use pokemon::{Pokemon, Pokemove, Poketype};

fn main() {
    let scratch = Pokemove {
        name: String::from("Scratch"),
        poketype: Poketype::Normal,
        power: 20,
        accuracy: Percentage::new(100),
    };

    let bubble_gun = Pokemove {
        name: String::from("Bubble Gun"),
        poketype: Poketype::Water,
        power: 85,
        accuracy: Percentage::new(95),
    };

    let inferno = Pokemove {
        name: String::from("Inferno"),
        poketype: Poketype::Fire,
        power: 90,
        accuracy: Percentage::new(80),
    };

    let mut squirtle = Pokemon {
        name: String::from("Squirtle"),
        poketype: Poketype::Water,
        pokemoves: [Some(scratch), Some(bubble_gun), None, None],
        level: Percentage::new(10),
        accuracy: Percentage::new(95),
        max_hp: 160,
        current_hp: 160,
    };

    let mut charmander = Pokemon {
        name: String::from("Charmander"),
        poketype: Poketype::Fire,
        pokemoves: [None, Some(inferno), None, None],
        level: Percentage::new(10),
        accuracy: Percentage::new(95),
        max_hp: 160,
        current_hp: 160,
    };

    println!("Welcome to Rustémon!");

    match squirtle.attack(0, &mut charmander) {
        Ok(true) => println!("Squirtle hit Charmander!"),
        Ok(false) => println!("Oh no! Squirtle missed..."),
        Err(err) => eprintln!("Attack failed: {}", err),
    }

    match charmander.attack(1, &mut squirtle) {
        Ok(true) => println!("Charmander hit Squirtle!"),
        Ok(false) => println!("Oh no! Charmander missed..."),
        Err(err) => eprintln!("Attack failed: {}", err),
    }

    println!("Thanks for playing!");
}
