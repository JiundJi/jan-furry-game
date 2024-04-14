use std::time::SystemTime;

use bevy::{ecs::component::Component, reflect::TypeData};

use crate::logic::*;

#[derive(Component)] pub struct Game {
    players: Vec<Player>,
    creation_time: SystemTime,
    start_time: SystemTime,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            players: vec![],
            creation_time: SystemTime::now(),
            start_time: SystemTime::now(),
        }
    }
}

impl Game {
    fn remove_player(&self, p: Player) -> Self {
        let mut players: Vec<Player> = vec![];

        for i in self.players.iter() {
            if i != &p {
                players.push(i.clone());
            }
        }

        Self {
            players,
            creation_time: self.creation_time,
            start_time: self.start_time,
        }
    }

    fn add_player(&self, p: Player) -> Self {
        let mut players: Vec<Player> = vec![];

        for i in self.players.iter() {
            players.push(i.clone());
        }

        players.push(p);

        Self {
            players,
            creation_time: self.creation_time,
            start_time: self.start_time,
        }
    }
}