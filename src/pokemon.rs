use crate::pokemath::Percentage;
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

        let damage = 1;
        target.current_hp -= damage;

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
    Fire,
    Water,
    Grass,
    Normal,
}
