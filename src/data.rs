use crate::pokemath::Percentage;
use crate::pokemon::{Pokemon, Pokemove, Poketype};
use std::collections::HashMap;
use std::sync::Arc;

pub fn pokemoves() -> HashMap<String, Arc<Pokemove>> {
    let mut pokemoves = HashMap::new();

    pokemoves.insert(
        String::from("scratch"),
        Arc::new(Pokemove {
            name: String::from("Scratch"),
            poketype: Poketype::Normal,
            power: 20,
            accuracy: Percentage::new(100),
        }),
    );

    pokemoves.insert(
        String::from("bubble_gun"),
        Arc::new(Pokemove {
            name: String::from("Bubble Gun"),
            poketype: Poketype::Water,
            power: 85,
            accuracy: Percentage::new(95),
        }),
    );

    pokemoves.insert(
        String::from("inferno"),
        Arc::new(Pokemove {
            name: String::from("Inferno"),
            poketype: Poketype::Fire,
            power: 90,
            accuracy: Percentage::new(80),
        }),
    );

    pokemoves
}

pub fn pokemon() -> HashMap<String, Pokemon> {
    let pokemoves = pokemoves();

    let mut pokemon = HashMap::new();

    pokemon.insert(
        String::from("squirtle"),
        Pokemon {
            name: String::from("Squirtle"),
            poketype: Poketype::Water,
            pokemoves: [
                Some(pokemoves.get("scratch").unwrap().clone()),
                Some(pokemoves.get("bubble_gun").unwrap().clone()),
                None,
                None,
            ],
            level: Percentage::new(10),
            accuracy: Percentage::new(95),
            max_hp: 160,
            current_hp: 160,
        },
    );

    pokemon.insert(
        String::from("charmander"),
        Pokemon {
            name: String::from("Charmander"),
            poketype: Poketype::Fire,
            pokemoves: [
                Some(pokemoves.get("scratch").unwrap().clone()),
                Some(pokemoves.get("inferno").unwrap().clone()),
                None,
                None,
            ],
            level: Percentage::new(10),
            accuracy: Percentage::new(95),
            max_hp: 160,
            current_hp: 160,
        },
    );

    pokemon
}
