use crate::pokemath::Percentage;
use crate::sleep_print;
use colored::{ColoredString, Colorize};
use rand::Rng;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Pokemon {
    pub name: String,
    pub poketype: Poketype,
    pub pokemoves: [Option<Rc<Pokemove>>; 4],
    pub level: Percentage,
    pub accuracy: Percentage,
    pub max_hp: u16,
    pub current_hp: u16,
}

impl Pokemon {
    pub fn attack(&mut self, attacking_move: &Pokemove, target: &mut Pokemon) -> bool {
        // Check for successfully landed attack
        if !self.attack_landed_by_accuracy(attacking_move.accuracy.value()) {
            sleep_print();
            println!("Oh no! {} missed...", self.colored_name());
            return false;
        }

        // Calculate and apply damage
        let damage = Self::calculate_damage(&attacking_move, &self, &target);
        target.current_hp = if damage >= target.current_hp {
            0
        } else {
            target.current_hp - damage
        };

        sleep_print();
        println!(
            "{} used {} on {} ({} / {})!",
            self.colored_name(),
            attacking_move.colored_name(),
            target.colored_name(),
            target.current_hp,
            target.max_hp
        );

        // Level up if enough damage is dealt
        if damage > (target.current_hp as f32 * 0.5).round() as u16 {
            self.level_up();
        }

        true
    }

    fn attack_landed_by_accuracy(&self, attacking_move_accuracy: u8) -> bool {
        let total_accuracy =
            (self.accuracy.value() as f32) * (attacking_move_accuracy as f32) / 100.0;

        let roll = rand::rng().random_range(0..101) as f32;

        total_accuracy > roll
    }

    fn level_up(&mut self) -> u8 {
        self.level = Percentage::new(self.level.value() + 1);

        sleep_print();
        println!(
            "{} reached level {}!",
            self.colored_name(),
            self.level.value()
        );

        self.level.value()
    }
}

impl Pokemon {
    fn calculate_damage(attacking_move: &Pokemove, attacker: &Pokemon, target: &Pokemon) -> u16 {
        // Get move base power
        let mut damage = attacking_move.power as f32;

        // Apply type matchup
        damage *= attacking_move
            .poketype
            .effectiveness(&target.poketype)
            .multiplier();

        // Check same-type bonus
        if attacker.poketype == attacking_move.poketype {
            damage *= 1.5;
        }

        // Apply level bonus
        damage *= 1.0 + (attacker.level.value() as f32 / 100.0);

        damage.round() as u16
    }
}

impl PoketypeColor for Pokemon {
    fn name(&self) -> &String {
        &self.name
    }

    fn poketype(&self) -> &Poketype {
        &self.poketype
    }
}

#[derive(Debug)]
pub struct Pokemove {
    pub name: String,
    pub poketype: Poketype,
    pub power: u16,
    pub accuracy: Percentage,
}

impl PoketypeColor for Pokemove {
    fn name(&self) -> &String {
        &self.name
    }

    fn poketype(&self) -> &Poketype {
        &self.poketype
    }
}

enum Effectiveness {
    None,
    Quarter,
    Half,
    Neutral,
    Double,
    Quadruple,
}

impl Effectiveness {
    fn multiplier(&self) -> f32 {
        match self {
            Self::None => 0.0,
            Self::Quarter => 0.25,
            Self::Half => 0.5,
            Self::Neutral => 1.0,
            Self::Double => 2.0,
            Self::Quadruple => 4.0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Poketype {
    Normal,
    Fire,
    Water,
    Grass,
    Electric,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Dark,
    Steel,
    Fairy,
    Ghost,
    Dragon,
}

impl Poketype {
    fn effectiveness(&self, other: &Self) -> Effectiveness {
        match self {
            Self::Normal => match other {
                Self::Rock => Effectiveness::Half,
                Self::Ghost => Effectiveness::None,
                Self::Steel => Effectiveness::Half,
                _ => Effectiveness::Neutral,
            },
            Self::Fire => match other {
                Self::Fire => Effectiveness::Half,
                Self::Water => Effectiveness::Half,
                Self::Grass => Effectiveness::Double,
                Self::Ice => Effectiveness::Double,
                Self::Bug => Effectiveness::Double,
                Self::Rock => Effectiveness::Half,
                Self::Dragon => Effectiveness::Half,
                Self::Steel => Effectiveness::Double,
                _ => Effectiveness::Neutral,
            },
            Self::Water => match other {
                Self::Fire => Effectiveness::Double,
                Self::Water => Effectiveness::Half,
                Self::Grass => Effectiveness::Half,
                Self::Electric => Effectiveness::Neutral,
                Self::Ice => Effectiveness::Half,
                Self::Steel => Effectiveness::Half,
                Self::Rock => Effectiveness::Double,
                Self::Dragon => Effectiveness::Half,
                _ => Effectiveness::Neutral,
            },
            Self::Grass => match other {
                Self::Fire => Effectiveness::Half,
                Self::Water => Effectiveness::Half,
                Self::Grass => Effectiveness::Half,
                Self::Electric => Effectiveness::Neutral,
                Self::Ice => Effectiveness::Neutral,
                Self::Poison => Effectiveness::Half,
                Self::Ground => Effectiveness::Double,
                Self::Flying => Effectiveness::Half,
                Self::Bug => Effectiveness::Half,
                Self::Rock => Effectiveness::Double,
                Self::Dragon => Effectiveness::Half,
                Self::Steel => Effectiveness::Half,
                _ => Effectiveness::Neutral,
            },
            Self::Electric => match other {
                Self::Water => Effectiveness::Double,
                Self::Electric => Effectiveness::Half,
                Self::Grass => Effectiveness::Half,
                Self::Ground => Effectiveness::Double,
                Self::Flying => Effectiveness::Double,
                Self::Dragon => Effectiveness::Half,
                _ => Effectiveness::Neutral,
            },
            Self::Ice => match other {
                Self::Fire => Effectiveness::Half,
                Self::Water => Effectiveness::Half,
                Self::Grass => Effectiveness::Double,
                Self::Ice => Effectiveness::Half,
                Self::Ground => Effectiveness::Double,
                Self::Flying => Effectiveness::Double,
                Self::Dragon => Effectiveness::Double,
                _ => Effectiveness::Neutral,
            },
            Self::Fighting => match other {
                Self::Normal => Effectiveness::Double,
                Self::Ice => Effectiveness::Double,
                Self::Rock => Effectiveness::Double,
                Self::Dark => Effectiveness::Double,
                Self::Steel => Effectiveness::Double,
                Self::Poison => Effectiveness::Half,
                Self::Flying => Effectiveness::Half,
                Self::Psychic => Effectiveness::Half,
                Self::Bug => Effectiveness::Half,
                Self::Fairy => Effectiveness::Half,
                Self::Ghost => Effectiveness::None,
                _ => Effectiveness::Neutral,
            },
            Self::Poison => match other {
                Self::Grass => Effectiveness::Double,
                Self::Fairy => Effectiveness::Double,
                Self::Poison => Effectiveness::Half,
                Self::Ground => Effectiveness::Half,
                Self::Rock => Effectiveness::Half,
                Self::Ghost => Effectiveness::Half,
                Self::Steel => Effectiveness::None,
                _ => Effectiveness::Neutral,
            },
            Self::Ground => match other {
                Self::Fire => Effectiveness::Double,
                Self::Electric => Effectiveness::Double,
                Self::Poison => Effectiveness::Double,
                Self::Rock => Effectiveness::Double,
                Self::Steel => Effectiveness::Double,
                Self::Grass => Effectiveness::Half,
                Self::Bug => Effectiveness::Half,
                Self::Flying => Effectiveness::None,
                _ => Effectiveness::Neutral,
            },
            Self::Flying => match other {
                Self::Grass => Effectiveness::Double,
                Self::Fighting => Effectiveness::Double,
                Self::Bug => Effectiveness::Double,
                Self::Electric => Effectiveness::Half,
                Self::Rock => Effectiveness::Half,
                Self::Steel => Effectiveness::Half,
                _ => Effectiveness::Neutral,
            },
            Self::Psychic => match other {
                Self::Fighting => Effectiveness::Double,
                Self::Poison => Effectiveness::Double,
                Self::Psychic => Effectiveness::Half,
                Self::Steel => Effectiveness::Half,
                Self::Dark => Effectiveness::None,
                _ => Effectiveness::Neutral,
            },
            Self::Bug => match other {
                Self::Grass => Effectiveness::Double,
                Self::Psychic => Effectiveness::Double,
                Self::Dark => Effectiveness::Double,
                Self::Fire => Effectiveness::Half,
                Self::Fighting => Effectiveness::Half,
                Self::Poison => Effectiveness::Half,
                Self::Flying => Effectiveness::Half,
                Self::Ghost => Effectiveness::Half,
                Self::Steel => Effectiveness::Half,
                Self::Fairy => Effectiveness::Half,
                _ => Effectiveness::Neutral,
            },
            Self::Rock => match other {
                Self::Fire => Effectiveness::Double,
                Self::Ice => Effectiveness::Double,
                Self::Flying => Effectiveness::Double,
                Self::Bug => Effectiveness::Double,
                Self::Fighting => Effectiveness::Half,
                Self::Ground => Effectiveness::Half,
                Self::Steel => Effectiveness::Half,
                _ => Effectiveness::Neutral,
            },
            Self::Ghost => match other {
                Self::Psychic => Effectiveness::Double,
                Self::Ghost => Effectiveness::Double,
                Self::Dark => Effectiveness::Half,
                Self::Normal => Effectiveness::None,
                _ => Effectiveness::Neutral,
            },
            Self::Dragon => match other {
                Self::Dragon => Effectiveness::Double,
                Self::Steel => Effectiveness::Half,
                Self::Fairy => Effectiveness::None,
                _ => Effectiveness::Neutral,
            },
            Self::Dark => match other {
                Self::Psychic => Effectiveness::Double,
                Self::Ghost => Effectiveness::Double,
                Self::Fighting => Effectiveness::Half,
                Self::Dark => Effectiveness::Half,
                Self::Fairy => Effectiveness::Half,
                _ => Effectiveness::Neutral,
            },
            Self::Steel => match other {
                Self::Ice => Effectiveness::Double,
                Self::Rock => Effectiveness::Double,
                Self::Fairy => Effectiveness::Double,
                Self::Fire => Effectiveness::Half,
                Self::Water => Effectiveness::Half,
                Self::Electric => Effectiveness::Half,
                Self::Steel => Effectiveness::Half,
                _ => Effectiveness::Neutral,
            },
            Self::Fairy => match other {
                Self::Fighting => Effectiveness::Double,
                Self::Dragon => Effectiveness::Double,
                Self::Dark => Effectiveness::Double,
                Self::Fire => Effectiveness::Half,
                Self::Poison => Effectiveness::Half,
                Self::Steel => Effectiveness::Half,
                _ => Effectiveness::Neutral,
            },
        }
    }

    fn color(&self) -> (u8, u8, u8) {
        match self {
            Self::Normal => (168, 167, 122),
            Self::Fire => (238, 129, 48),
            Self::Water => (99, 144, 240),
            Self::Grass => (122, 199, 76),
            Self::Electric => (247, 208, 44),
            Self::Ice => (150, 217, 214),
            Self::Fighting => (194, 46, 40),
            Self::Poison => (163, 62, 161),
            Self::Ground => (226, 191, 101),
            Self::Flying => (169, 143, 243),
            Self::Psychic => (249, 85, 135),
            Self::Bug => (166, 185, 26),
            Self::Rock => (182, 161, 54),
            Self::Ghost => (115, 87, 151),
            Self::Dragon => (111, 53, 252),
            Self::Dark => (112, 87, 70),
            Self::Steel => (183, 183, 206),
            Self::Fairy => (214, 133, 173),
        }
    }
}

pub trait PoketypeColor {
    fn name(&self) -> &String;
    fn poketype(&self) -> &Poketype;

    fn colored_name(&self) -> ColoredString {
        let color = self.poketype().color();
        self.name().truecolor(color.0, color.1, color.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_damage_factors_included_in_calculation() {
        // 90 * 2 * 1.5 * 1.48 = 399.6
        // Rounded to 400

        let attacker_move_power = 90;
        let attacker_type = Poketype::Fire;
        let target_type = Poketype::Grass;
        let attacker_move_type = Poketype::Fire;
        let attacker_level: u8 = 48;

        let attacking_move = Rc::new(Pokemove {
            name: String::from("Flamethrower"),
            poketype: attacker_move_type,
            power: attacker_move_power,
            accuracy: Percentage::new(85),
        });

        let attacker = Pokemon {
            name: String::from("Charmander"),
            poketype: attacker_type,
            pokemoves: [Some(attacking_move.clone()), None, None, None],
            level: Percentage::new(attacker_level),
            accuracy: Percentage::new(95),
            max_hp: 160,
            current_hp: 160,
        };
        let target = Pokemon {
            name: String::from("Bulbasaur"),
            poketype: target_type,
            pokemoves: [None, None, None, None],
            level: Percentage::new(10),
            accuracy: Percentage::new(95),
            max_hp: 160,
            current_hp: 160,
        };

        let damage = Pokemon::calculate_damage(&attacking_move, &attacker, &target);

        assert_eq!(damage, 400);
    }
}
