use crate::pokemath::Percentage;
use crate::pokemon::{Pokemon, Pokemove, Poketype};
use std::collections::HashMap;
use std::rc::Rc;

fn pokemove_index() -> HashMap<&'static str, u32> {
    HashMap::from([
        ("tackle", 1),
        ("quick_attack", 2),
        ("ember", 3),
        ("flamethrower", 4),
        ("bubble", 5),
        ("water_gun", 6),
        ("vine_whip", 7),
        ("leaf_blade", 8),
        ("thunder_shock", 9),
        ("thunderbolt", 10),
        ("frost_breath", 11),
        ("ice_beam", 12),
        ("karate_chop", 13),
        ("close_combat", 14),
        ("poison_sting", 15),
        ("sludge_bomb", 16),
        ("mud_slap", 17),
        ("earthquake", 18),
        ("gust", 19),
        ("wing_attack", 20),
        ("confusion", 21),
        ("psybeam", 22),
        ("bug_bite", 23),
        ("x_scissor", 24),
        ("rock_throw", 25),
        ("stone_edge", 26),
        ("lick", 27),
        ("shadow_ball", 28),
        ("dragon_breath", 29),
        ("dragon_claw", 30),
        ("bite", 31),
        ("crunch", 32),
        ("metal_claw", 33),
        ("iron_tail", 34),
        ("fairy_wind", 35),
        ("moonblast", 36),
    ])
}

pub fn pokemoves() -> HashMap<u32, Rc<Pokemove>> {
    HashMap::from([
        (
            1,
            Rc::new(Pokemove {
                name: String::from("Tackle"),
                poketype: Poketype::Normal,
                power: 40,
                accuracy: Percentage::new(95),
            }),
        ),
        (
            2,
            Rc::new(Pokemove {
                name: String::from("Quick Attack"),
                poketype: Poketype::Normal,
                power: 40,
                accuracy: Percentage::new(100),
            }),
        ),
        (
            3,
            Rc::new(Pokemove {
                name: String::from("Ember"),
                poketype: Poketype::Fire,
                power: 40,
                accuracy: Percentage::new(95),
            }),
        ),
        (
            4,
            Rc::new(Pokemove {
                name: String::from("Flamethrower"),
                poketype: Poketype::Fire,
                power: 90,
                accuracy: Percentage::new(85),
            }),
        ),
        (
            5,
            Rc::new(Pokemove {
                name: String::from("Bubble"),
                poketype: Poketype::Water,
                power: 40,
                accuracy: Percentage::new(100),
            }),
        ),
        (
            6,
            Rc::new(Pokemove {
                name: String::from("Water Gun"),
                poketype: Poketype::Water,
                power: 40,
                accuracy: Percentage::new(95),
            }),
        ),
        (
            7,
            Rc::new(Pokemove {
                name: String::from("Vine Whip"),
                poketype: Poketype::Grass,
                power: 45,
                accuracy: Percentage::new(100),
            }),
        ),
        (
            8,
            Rc::new(Pokemove {
                name: String::from("Leaf Blade"),
                poketype: Poketype::Grass,
                power: 70,
                accuracy: Percentage::new(90),
            }),
        ),
        (
            9,
            Rc::new(Pokemove {
                name: String::from("Thunder Shock"),
                poketype: Poketype::Electric,
                power: 40,
                accuracy: Percentage::new(100),
            }),
        ),
        (
            10,
            Rc::new(Pokemove {
                name: String::from("Thunderbolt"),
                poketype: Poketype::Electric,
                power: 90,
                accuracy: Percentage::new(85),
            }),
        ),
        (
            11,
            Rc::new(Pokemove {
                name: String::from("Frost Breath"),
                poketype: Poketype::Ice,
                power: 40,
                accuracy: Percentage::new(95),
            }),
        ),
        (
            12,
            Rc::new(Pokemove {
                name: String::from("Ice Beam"),
                poketype: Poketype::Ice,
                power: 90,
                accuracy: Percentage::new(90),
            }),
        ),
        (
            13,
            Rc::new(Pokemove {
                name: String::from("Karate Chop"),
                poketype: Poketype::Fighting,
                power: 50,
                accuracy: Percentage::new(100),
            }),
        ),
        (
            14,
            Rc::new(Pokemove {
                name: String::from("Close Combat"),
                poketype: Poketype::Fighting,
                power: 120,
                accuracy: Percentage::new(80),
            }),
        ),
        (
            15,
            Rc::new(Pokemove {
                name: String::from("Poison Sting"),
                poketype: Poketype::Poison,
                power: 15,
                accuracy: Percentage::new(100),
            }),
        ),
        (
            16,
            Rc::new(Pokemove {
                name: String::from("Sludge Bomb"),
                poketype: Poketype::Poison,
                power: 90,
                accuracy: Percentage::new(85),
            }),
        ),
        (
            17,
            Rc::new(Pokemove {
                name: String::from("Mud-Slap"),
                poketype: Poketype::Ground,
                power: 20,
                accuracy: Percentage::new(95),
            }),
        ),
        (
            18,
            Rc::new(Pokemove {
                name: String::from("Earthquake"),
                poketype: Poketype::Ground,
                power: 100,
                accuracy: Percentage::new(85),
            }),
        ),
        (
            19,
            Rc::new(Pokemove {
                name: String::from("Gust"),
                poketype: Poketype::Flying,
                power: 40,
                accuracy: Percentage::new(100),
            }),
        ),
        (
            20,
            Rc::new(Pokemove {
                name: String::from("Wing Attack"),
                poketype: Poketype::Flying,
                power: 60,
                accuracy: Percentage::new(95),
            }),
        ),
        (
            21,
            Rc::new(Pokemove {
                name: String::from("Confusion"),
                poketype: Poketype::Psychic,
                power: 50,
                accuracy: Percentage::new(100),
            }),
        ),
        (
            22,
            Rc::new(Pokemove {
                name: String::from("Psybeam"),
                poketype: Poketype::Psychic,
                power: 65,
                accuracy: Percentage::new(95),
            }),
        ),
        (
            23,
            Rc::new(Pokemove {
                name: String::from("Bug Bite"),
                poketype: Poketype::Bug,
                power: 60,
                accuracy: Percentage::new(100),
            }),
        ),
        (
            24,
            Rc::new(Pokemove {
                name: String::from("X-Scissor"),
                poketype: Poketype::Bug,
                power: 80,
                accuracy: Percentage::new(90),
            }),
        ),
        (
            25,
            Rc::new(Pokemove {
                name: String::from("Rock Throw"),
                poketype: Poketype::Rock,
                power: 50,
                accuracy: Percentage::new(90),
            }),
        ),
        (
            26,
            Rc::new(Pokemove {
                name: String::from("Stone Edge"),
                poketype: Poketype::Rock,
                power: 100,
                accuracy: Percentage::new(80),
            }),
        ),
        (
            31,
            Rc::new(Pokemove {
                name: String::from("Bite"),
                poketype: Poketype::Dark,
                power: 60,
                accuracy: Percentage::new(95),
            }),
        ),
        (
            32,
            Rc::new(Pokemove {
                name: String::from("Crunch"),
                poketype: Poketype::Dark,
                power: 80,
                accuracy: Percentage::new(90),
            }),
        ),
        (
            33,
            Rc::new(Pokemove {
                name: String::from("Metal Claw"),
                poketype: Poketype::Steel,
                power: 50,
                accuracy: Percentage::new(95),
            }),
        ),
        (
            34,
            Rc::new(Pokemove {
                name: String::from("Iron Tail"),
                poketype: Poketype::Steel,
                power: 100,
                accuracy: Percentage::new(75),
            }),
        ),
        (
            35,
            Rc::new(Pokemove {
                name: String::from("Fairy Wind"),
                poketype: Poketype::Fairy,
                power: 40,
                accuracy: Percentage::new(100),
            }),
        ),
        (
            36,
            Rc::new(Pokemove {
                name: String::from("Moonblast"),
                poketype: Poketype::Fairy,
                power: 95,
                accuracy: Percentage::new(90),
            }),
        ),
        (
            27,
            Rc::new(Pokemove {
                name: String::from("Lick"),
                poketype: Poketype::Ghost,
                power: 30,
                accuracy: Percentage::new(100),
            }),
        ),
        (
            28,
            Rc::new(Pokemove {
                name: String::from("Shadow Ball"),
                poketype: Poketype::Ghost,
                power: 80,
                accuracy: Percentage::new(100),
            }),
        ),
        (
            29,
            Rc::new(Pokemove {
                name: String::from("Dragon Breath"),
                poketype: Poketype::Dragon,
                power: 60,
                accuracy: Percentage::new(100),
            }),
        ),
        (
            30,
            Rc::new(Pokemove {
                name: String::from("Dragon Claw"),
                poketype: Poketype::Dragon,
                power: 80,
                accuracy: Percentage::new(100),
            }),
        ),
    ])
}

pub fn pokemon_index() -> HashMap<&'static str, u32> {
    HashMap::from([
        ("squirtle", 1),
        ("charmander", 2),
        ("bulbasaur", 3),
        ("ivysaur", 4),
        ("venusaur", 5),
        ("charmeleon", 6),
        ("charizard", 7),
        ("wartortle", 8),
        ("blastoise", 9),
        ("pikachu", 10),
        ("raichu", 11),
        ("jigglypuff", 12),
        ("wigglytuff", 13),
        ("diglett", 14),
        ("dugtrio", 15),
        ("meowth", 16),
        ("persian", 17),
        ("psyduck", 18),
        ("golduck", 19),
        ("machop", 20),
        ("machoke", 21),
        ("machamp", 22),
        ("abra", 23),
        ("kadabra", 24),
        ("alakazam", 25),
        ("gastly", 26),
        ("haunter", 27),
        ("gengar", 28),
        ("scyther", 29),
        ("scizor", 30),
        ("magmar", 31),
        ("electabuzz", 32),
        ("electivire", 33),
        ("magikarp", 34),
        ("gyarados", 35),
        ("lapras", 36),
        ("eevee", 37),
        ("vaporeon", 38),
        ("jolteon", 39),
        ("flareon", 40),
        ("espeon", 41),
        ("umbreon", 42),
        ("leafeon", 43),
        ("glaceon", 44),
        ("sylveon", 45),
        ("snorlax", 46),
        ("mew", 47),
        ("dratini", 48),
        ("dragonair", 49),
        ("dragonite", 50),
        ("geodude", 51),
        ("graveler", 52),
        ("golem", 53),
        ("onix", 54),
        ("steelix", 55),
    ])
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
                            .get(&pokemove_index().get("tackle").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("water_gun").unwrap())
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
                            .get(&pokemove_index().get("tackle").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("ember").unwrap())
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
            pokemon_index().get("bulbasaur").unwrap().clone(),
            Pokemon {
                name: String::from("Bulbasaur"),
                poketype: Poketype::Grass,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("vine_whip").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("leaf_blade").unwrap())
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
            pokemon_index().get("ivysaur").unwrap().clone(),
            Pokemon {
                name: String::from("Ivysaur"),
                poketype: Poketype::Grass,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("vine_whip").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("leaf_blade").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("tackle").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    None,
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("venusaur").unwrap().clone(),
            Pokemon {
                name: String::from("Venusaur"),
                poketype: Poketype::Grass,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("vine_whip").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("leaf_blade").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("tackle").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("sludge_bomb").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("charmeleon").unwrap().clone(),
            Pokemon {
                name: String::from("Charmeleon"),
                poketype: Poketype::Fire,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("ember").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("flamethrower").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("tackle").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    None,
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("charizard").unwrap().clone(),
            Pokemon {
                name: String::from("Charizard"),
                poketype: Poketype::Fire,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("flamethrower").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("wing_attack").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("gust").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("tackle").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("wartortle").unwrap().clone(),
            Pokemon {
                name: String::from("Wartortle"),
                poketype: Poketype::Water,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("bubble").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("water_gun").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("tackle").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    None,
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("blastoise").unwrap().clone(),
            Pokemon {
                name: String::from("Blastoise"),
                poketype: Poketype::Water,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("water_gun").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("bubble").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("tackle").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("ice_beam").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("pikachu").unwrap().clone(),
            Pokemon {
                name: String::from("Pikachu"),
                poketype: Poketype::Electric,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("thunder_shock").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("quick_attack").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("tackle").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    None,
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("raichu").unwrap().clone(),
            Pokemon {
                name: String::from("Raichu"),
                poketype: Poketype::Electric,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("thunderbolt").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("quick_attack").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("tackle").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("thunder_shock").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("jigglypuff").unwrap().clone(),
            Pokemon {
                name: String::from("Jigglypuff"),
                poketype: Poketype::Normal,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("tackle").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("fairy_wind").unwrap())
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
            pokemon_index().get("wigglytuff").unwrap().clone(),
            Pokemon {
                name: String::from("Wigglytuff"),
                poketype: Poketype::Normal,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("tackle").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("fairy_wind").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("moonblast").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    None,
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("diglett").unwrap().clone(),
            Pokemon {
                name: String::from("Diglett"),
                poketype: Poketype::Ground,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("mud_slap").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("tackle").unwrap())
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
            pokemon_index().get("dugtrio").unwrap().clone(),
            Pokemon {
                name: String::from("Dugtrio"),
                poketype: Poketype::Ground,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("mud_slap").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("earthquake").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("tackle").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    None,
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("meowth").unwrap().clone(),
            Pokemon {
                name: String::from("Meowth"),
                poketype: Poketype::Normal,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("tackle").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("quick_attack").unwrap())
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
            pokemon_index().get("persian").unwrap().clone(),
            Pokemon {
                name: String::from("Persian"),
                poketype: Poketype::Normal,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("tackle").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("quick_attack").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("bite").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    None,
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("psyduck").unwrap().clone(),
            Pokemon {
                name: String::from("Psyduck"),
                poketype: Poketype::Water,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("water_gun").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("confusion").unwrap())
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
            pokemon_index().get("golduck").unwrap().clone(),
            Pokemon {
                name: String::from("Golduck"),
                poketype: Poketype::Water,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("water_gun").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("confusion").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("psybeam").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    None,
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("machop").unwrap().clone(),
            Pokemon {
                name: String::from("Machop"),
                poketype: Poketype::Fighting,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("karate_chop").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("tackle").unwrap())
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
            pokemon_index().get("machoke").unwrap().clone(),
            Pokemon {
                name: String::from("Machoke"),
                poketype: Poketype::Fighting,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("karate_chop").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("close_combat").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("tackle").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    None,
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("machamp").unwrap().clone(),
            Pokemon {
                name: String::from("Machamp"),
                poketype: Poketype::Fighting,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("karate_chop").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("close_combat").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("tackle").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("earthquake").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("abra").unwrap().clone(),
            Pokemon {
                name: String::from("Abra"),
                poketype: Poketype::Psychic,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("confusion").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    None,
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
            pokemon_index().get("kadabra").unwrap().clone(),
            Pokemon {
                name: String::from("Kadabra"),
                poketype: Poketype::Psychic,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("confusion").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("psybeam").unwrap())
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
            pokemon_index().get("alakazam").unwrap().clone(),
            Pokemon {
                name: String::from("Alakazam"),
                poketype: Poketype::Psychic,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("confusion").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("psybeam").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("shadow_ball").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    None,
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("gastly").unwrap().clone(),
            Pokemon {
                name: String::from("Gastly"),
                poketype: Poketype::Ghost,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("lick").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("shadow_ball").unwrap())
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
            pokemon_index().get("haunter").unwrap().clone(),
            Pokemon {
                name: String::from("Haunter"),
                poketype: Poketype::Ghost,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("lick").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("shadow_ball").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("poison_sting").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    None,
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("gengar").unwrap().clone(),
            Pokemon {
                name: String::from("Gengar"),
                poketype: Poketype::Ghost,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("lick").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("shadow_ball").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("poison_sting").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("sludge_bomb").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("scyther").unwrap().clone(),
            Pokemon {
                name: String::from("Scyther"),
                poketype: Poketype::Bug,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("bug_bite").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("x_scissor").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("wing_attack").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    None,
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("scizor").unwrap().clone(),
            Pokemon {
                name: String::from("Scizor"),
                poketype: Poketype::Bug,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("bug_bite").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("x_scissor").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("metal_claw").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    None,
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("magmar").unwrap().clone(),
            Pokemon {
                name: String::from("Magmar"),
                poketype: Poketype::Fire,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("ember").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("flamethrower").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("karate_chop").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    None,
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("electabuzz").unwrap().clone(),
            Pokemon {
                name: String::from("Electabuzz"),
                poketype: Poketype::Electric,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("thunder_shock").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("thunderbolt").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("karate_chop").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    None,
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("electivire").unwrap().clone(),
            Pokemon {
                name: String::from("Electivire"),
                poketype: Poketype::Electric,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("thunderbolt").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("karate_chop").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("earthquake").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    None,
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("magikarp").unwrap().clone(),
            Pokemon {
                name: String::from("Magikarp"),
                poketype: Poketype::Water,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("tackle").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    None,
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
            pokemon_index().get("gyarados").unwrap().clone(),
            Pokemon {
                name: String::from("Gyarados"),
                poketype: Poketype::Water,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("bite").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("water_gun").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("dragon_breath").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("ice_beam").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("lapras").unwrap().clone(),
            Pokemon {
                name: String::from("Lapras"),
                poketype: Poketype::Water,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("ice_beam").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("water_gun").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("confusion").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("tackle").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("eevee").unwrap().clone(),
            Pokemon {
                name: String::from("Eevee"),
                poketype: Poketype::Normal,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("tackle").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("quick_attack").unwrap())
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
            pokemon_index().get("vaporeon").unwrap().clone(),
            Pokemon {
                name: String::from("Vaporeon"),
                poketype: Poketype::Water,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("water_gun").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("ice_beam").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("tackle").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    None,
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("jolteon").unwrap().clone(),
            Pokemon {
                name: String::from("Jolteon"),
                poketype: Poketype::Electric,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("thunder_shock").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("thunderbolt").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("quick_attack").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    None,
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("flareon").unwrap().clone(),
            Pokemon {
                name: String::from("Flareon"),
                poketype: Poketype::Fire,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("ember").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("flamethrower").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("bite").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    None,
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("espeon").unwrap().clone(),
            Pokemon {
                name: String::from("Espeon"),
                poketype: Poketype::Psychic,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("confusion").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("psybeam").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("quick_attack").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    None,
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("umbreon").unwrap().clone(),
            Pokemon {
                name: String::from("Umbreon"),
                poketype: Poketype::Dark,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("bite").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("crunch").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("quick_attack").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    None,
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("leafeon").unwrap().clone(),
            Pokemon {
                name: String::from("Leafeon"),
                poketype: Poketype::Grass,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("leaf_blade").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("vine_whip").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("quick_attack").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    None,
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("glaceon").unwrap().clone(),
            Pokemon {
                name: String::from("Glaceon"),
                poketype: Poketype::Ice,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("ice_beam").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("frost_breath").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("quick_attack").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    None,
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("sylveon").unwrap().clone(),
            Pokemon {
                name: String::from("Sylveon"),
                poketype: Poketype::Fairy,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("fairy_wind").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("moonblast").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("tackle").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    None,
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("snorlax").unwrap().clone(),
            Pokemon {
                name: String::from("Snorlax"),
                poketype: Poketype::Normal,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("tackle").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("lick").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("earthquake").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("crunch").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("mew").unwrap().clone(),
            Pokemon {
                name: String::from("Mew"),
                poketype: Poketype::Psychic,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("confusion").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("psybeam").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("shadow_ball").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("earthquake").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("dratini").unwrap().clone(),
            Pokemon {
                name: String::from("Dratini"),
                poketype: Poketype::Dragon,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("dragon_breath").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("tackle").unwrap())
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
            pokemon_index().get("dragonair").unwrap().clone(),
            Pokemon {
                name: String::from("Dragonair"),
                poketype: Poketype::Dragon,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("dragon_breath").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("dragon_claw").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("tackle").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    None,
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("dragonite").unwrap().clone(),
            Pokemon {
                name: String::from("Dragonite"),
                poketype: Poketype::Dragon,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("dragon_claw").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("wing_attack").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("earthquake").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("ice_beam").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("geodude").unwrap().clone(),
            Pokemon {
                name: String::from("Geodude"),
                poketype: Poketype::Rock,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("rock_throw").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("mud_slap").unwrap())
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
            pokemon_index().get("graveler").unwrap().clone(),
            Pokemon {
                name: String::from("Graveler"),
                poketype: Poketype::Rock,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("rock_throw").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("stone_edge").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("mud_slap").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    None,
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("golem").unwrap().clone(),
            Pokemon {
                name: String::from("Golem"),
                poketype: Poketype::Rock,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("rock_throw").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("stone_edge").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("earthquake").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("mud_slap").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("onix").unwrap().clone(),
            Pokemon {
                name: String::from("Onix"),
                poketype: Poketype::Rock,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("rock_throw").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("mud_slap").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("earthquake").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    None,
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
        (
            pokemon_index().get("steelix").unwrap().clone(),
            Pokemon {
                name: String::from("Steelix"),
                poketype: Poketype::Steel,
                pokemoves: [
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("iron_tail").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("mud_slap").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("crunch").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                    Some(
                        pokemoves
                            .get(&pokemove_index().get("earthquake").unwrap())
                            .unwrap()
                            .clone(),
                    ),
                ],
                level: Percentage::new(10),
                accuracy: Percentage::new(95),
                max_hp: 160,
                current_hp: 160,
            },
        ),
    ])
}
