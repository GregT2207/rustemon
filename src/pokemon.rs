use crate::pokemath::Percentage;
use colored::{Color, Colorize};
use colored::{ColoredString, CustomColor};
use rand::Rng;
use std::error::Error;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Pokemon {
    pub name: String,
    pub poketype: Poketype,
    pub pokemoves: [Option<Arc<Pokemove>>; 4],
    pub level: Percentage,
    pub accuracy: Percentage,
    pub max_hp: u16,
    pub current_hp: u16,
}

impl Pokemon {
    pub fn attack(&mut self, move_index: u8, target: &mut Pokemon) -> Result<bool, Box<dyn Error>> {
        let attacking_move = self
            .pokemoves
            .get(move_index as usize)
            .ok_or(format!(
                "Move index {} out of bounds (use 0 - 3)",
                move_index
            ))?
            .as_ref()
            .ok_or(format!("Move {} doesn't exist", move_index + 1))?;

        if !self.attack_landed_by_accuracy(attacking_move.accuracy.value()) {
            return Ok(false);
        }

        target.current_hp -= Self::calculate_damage(&attacking_move, &self, &target);

        self.level_up();

        Ok(true)
    }

    fn attack_landed_by_accuracy(&self, attacking_move_accuracy: u8) -> bool {
        let total_accuracy =
            (self.accuracy.value() as f32) * (attacking_move_accuracy as f32) / 100.0;

        let roll = rand::rng().random_range(0..101) as f32;

        total_accuracy > roll
    }

    fn level_up(&mut self) -> u8 {
        self.level = Percentage::new(self.level.value() + 1);
        self.level.value()
    }

    pub fn colored_name(&self) -> ColoredString {
        let color = self.poketype.color();
        self.name.truecolor(color.0, color.1, color.2)
    }
}

impl Pokemon {
    fn calculate_damage(attacking_move: &Pokemove, attacker: &Pokemon, target: &Pokemon) -> u16 {
        let damage: u16 = attacking_move.power;

        damage
    }
}

#[derive(Debug)]
pub struct Pokemove {
    pub name: String,
    pub poketype: Poketype,
    pub power: u16,
    pub accuracy: Percentage,
}

#[derive(Clone, Debug)]
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
    fn effectiveness(&self, other: &Self) -> f32 {
        match self {
            Self::Normal => match other {
                Self::Rock => 0.5,
                Self::Ghost => 0.0,
                Self::Steel => 0.5,
                _ => 1.0,
            },
            Self::Fire => match other {
                Self::Fire => 0.5,
                Self::Water => 0.5,
                Self::Grass => 2.0,
                Self::Ice => 2.0,
                Self::Bug => 2.0,
                Self::Rock => 0.5,
                Self::Dragon => 0.5,
                Self::Steel => 2.0,
                _ => 1.0,
            },
            Self::Water => match other {
                Self::Fire => 2.0,
                Self::Water => 0.5,
                Self::Grass => 0.5,
                Self::Electric => 1.0,
                Self::Ice => 0.5,
                Self::Steel => 0.5,
                Self::Rock => 2.0,
                Self::Dragon => 0.5,
                _ => 1.0,
            },
            Self::Grass => match other {
                Self::Fire => 0.5,
                Self::Water => 0.5,
                Self::Grass => 0.5,
                Self::Electric => 1.0,
                Self::Ice => 1.0,
                Self::Poison => 0.5,
                Self::Ground => 2.0,
                Self::Flying => 0.5,
                Self::Bug => 0.5,
                Self::Rock => 2.0,
                Self::Dragon => 0.5,
                Self::Steel => 0.5,
                _ => 1.0,
            },
            Self::Electric => match other {
                Self::Water => 2.0,
                Self::Electric => 0.5,
                Self::Grass => 0.5,
                Self::Ground => 2.0,
                Self::Flying => 2.0,
                Self::Dragon => 0.5,
                _ => 1.0,
            },
            Self::Ice => match other {
                Self::Fire => 0.5,
                Self::Water => 0.5,
                Self::Grass => 2.0,
                Self::Ice => 0.5,
                Self::Ground => 2.0,
                Self::Flying => 2.0,
                Self::Dragon => 2.0,
                _ => 1.0,
            },
            Self::Fighting => match other {
                Self::Normal => 2.0,
                Self::Ice => 2.0,
                Self::Rock => 2.0,
                Self::Dark => 2.0,
                Self::Steel => 2.0,
                Self::Poison => 0.5,
                Self::Flying => 0.5,
                Self::Psychic => 0.5,
                Self::Bug => 0.5,
                Self::Fairy => 0.5,
                Self::Ghost => 0.0,
                _ => 1.0,
            },
            Self::Poison => match other {
                Self::Grass => 2.0,
                Self::Fairy => 2.0,
                Self::Poison => 0.5,
                Self::Ground => 0.5,
                Self::Rock => 0.5,
                Self::Ghost => 0.5,
                Self::Steel => 0.0,
                _ => 1.0,
            },
            Self::Ground => match other {
                Self::Fire => 2.0,
                Self::Electric => 2.0,
                Self::Poison => 2.0,
                Self::Rock => 2.0,
                Self::Steel => 2.0,
                Self::Grass => 0.5,
                Self::Bug => 0.5,
                Self::Flying => 0.0,
                _ => 1.0,
            },
            Self::Flying => match other {
                Self::Grass => 2.0,
                Self::Fighting => 2.0,
                Self::Bug => 2.0,
                Self::Electric => 0.5,
                Self::Rock => 0.5,
                Self::Steel => 0.5,
                _ => 1.0,
            },
            Self::Psychic => match other {
                Self::Fighting => 2.0,
                Self::Poison => 2.0,
                Self::Psychic => 0.5,
                Self::Steel => 0.5,
                Self::Dark => 0.0,
                _ => 1.0,
            },
            Self::Bug => match other {
                Self::Grass => 2.0,
                Self::Psychic => 2.0,
                Self::Dark => 2.0,
                Self::Fire => 0.5,
                Self::Fighting => 0.5,
                Self::Poison => 0.5,
                Self::Flying => 0.5,
                Self::Ghost => 0.5,
                Self::Steel => 0.5,
                Self::Fairy => 0.5,
                _ => 1.0,
            },
            Self::Rock => match other {
                Self::Fire => 2.0,
                Self::Ice => 2.0,
                Self::Flying => 2.0,
                Self::Bug => 2.0,
                Self::Fighting => 0.5,
                Self::Ground => 0.5,
                Self::Steel => 0.5,
                _ => 1.0,
            },
            Self::Ghost => match other {
                Self::Psychic => 2.0,
                Self::Ghost => 2.0,
                Self::Dark => 0.5,
                Self::Normal => 0.0,
                _ => 1.0,
            },
            Self::Dragon => match other {
                Self::Dragon => 2.0,
                Self::Steel => 0.5,
                Self::Fairy => 0.0,
                _ => 1.0,
            },
            Self::Dark => match other {
                Self::Psychic => 2.0,
                Self::Ghost => 2.0,
                Self::Fighting => 0.5,
                Self::Dark => 0.5,
                Self::Fairy => 0.5,
                _ => 1.0,
            },
            Self::Steel => match other {
                Self::Ice => 2.0,
                Self::Rock => 2.0,
                Self::Fairy => 2.0,
                Self::Fire => 0.5,
                Self::Water => 0.5,
                Self::Electric => 0.5,
                Self::Steel => 0.5,
                _ => 1.0,
            },
            Self::Fairy => match other {
                Self::Fighting => 2.0,
                Self::Dragon => 2.0,
                Self::Dark => 2.0,
                Self::Fire => 0.5,
                Self::Poison => 0.5,
                Self::Steel => 0.5,
                _ => 1.0,
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
