use crate::pokemath::Percentage;
use crate::pokemon::{Pokemon, Pokemove, Poketype};
use std::collections::HashMap;
use std::sync::Arc;

fn pokemove_index() -> HashMap<&'static str, u32> {
    HashMap::from([("scratch", 1), ("bubble_gun", 2), ("inferno", 3)])
}

pub fn pokemoves() -> HashMap<u32, Arc<Pokemove>> {
    HashMap::from([
        (
            1,
            Arc::new(Pokemove {
                name: String::from("Scratch"),
                poketype: Poketype::Normal,
                power: 20,
                accuracy: Percentage::new(100),
            }),
        ),
        (
            2,
            Arc::new(Pokemove {
                name: String::from("Bubble Gun"),
                poketype: Poketype::Water,
                power: 85,
                accuracy: Percentage::new(95),
            }),
        ),
        (
            3,
            Arc::new(Pokemove {
                name: String::from("Inferno"),
                poketype: Poketype::Fire,
                power: 90,
                accuracy: Percentage::new(80),
            }),
        ),
    ])
}

pub fn pokemon_index() -> HashMap<&'static str, u32> {
    HashMap::from([("squirtle", 1), ("charmander", 2)])
}

pub fn pokemon() -> HashMap<u32, Pokemon> {
    let pokemoves = pokemoves();

    HashMap::from([
        (
            pokemon_index().get("squirtle").unwrap().clone(),
            Pokemon {
                name: String::from("Squirtle"),
                poketype: Poketype::Water,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("scratch").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("bubble_gun").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    None,
                    None,
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("charmander").unwrap().clone(),
            Pokemon {
                name: String::from("Charmander"),
                poketype: Poketype::Fire,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("scratch").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("inferno").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    None,
                    None,
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
    ])
}
