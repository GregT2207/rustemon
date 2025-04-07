use std::thread::sleep;

use crate::{
    pokemon::{Pokemon, Pokemove, PoketypeColor},
    sleep_print,
};

pub enum TrainerKind {
    User,
    Enemy,
}

pub struct Trainer {
    kind: TrainerKind,
    name: &'static str,
    pokemon_team: Vec<Pokemon>,
    current_pokemon_index: usize,
}

impl Trainer {
    pub fn new(kind: TrainerKind, name: &'static str, pokemon_team: Vec<Pokemon>) -> Trainer {
        let trainer = Trainer {
            kind,
            name,
            pokemon_team,
            current_pokemon_index: 0,
        };

        trainer.print_sent_out();

        trainer
    }

    pub fn current_pokemon(&self) -> Option<&Pokemon> {
        self.pokemon_team.get(self.current_pokemon_index)
    }

    pub fn current_pokemon_mut(&mut self) -> Option<&mut Pokemon> {
        self.pokemon_team.get_mut(self.current_pokemon_index)
    }

    fn send_next_pokemon(&mut self) {
        self.current_pokemon_index += 1;
        self.print_sent_out();
    }

    pub fn print_sent_out(&self) {
        if let Some(current_pokemon) = self.current_pokemon() {
            crate::sleep_print();
            println!(
                "{} sent out {} ({} / {})!",
                self.name,
                current_pokemon.colored_name(),
                current_pokemon.current_hp,
                current_pokemon.max_hp,
            );
        }
    }

    pub fn attack(&mut self, attacking_move: &Pokemove, target: &mut Trainer) -> bool {
        if let Some(self_pokemon) = self.current_pokemon_mut() {
            if let Some(target_pokemon) = target.current_pokemon_mut() {
                let attacked = self_pokemon.attack(attacking_move, target_pokemon);

                self.update();

                return attacked;
            }
        }

        false
    }

    pub fn update(&mut self) {
        if let Some(current_pokemon) = self.current_pokemon() {
            if current_pokemon.current_hp == 0 {
                sleep_print();
                println!("{} fainted!", current_pokemon.colored_name());

                self.send_next_pokemon();
            }
        }
    }
}
