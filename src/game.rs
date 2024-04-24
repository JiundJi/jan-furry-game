use std::{collections::HashMap, time::SystemTime};
use bevy::ecs::component::Component;

type PlayerId = u64;

#[derive(Component)]
pub struct GameState {
    players: HashMap<PlayerId, String>,
    history: Vec<GameEvent>,
    creation_time: SystemTime,
    start_time: SystemTime,
}

#[derive(Clone)]
pub enum GameEvent {
    PlayerJoined { player_id: PlayerId, name: String },
    PlayerLeft(PlayerId), 
}


impl GameState {
    pub fn reduce(&mut self, event: &GameEvent) {
        use GameEvent::*;
        
        match event {
            PlayerJoined { player_id, name } => {
                self.players.insert(*player_id, name.to_string());
            },
            PlayerLeft (player_id) => {
                self.players.remove(player_id);
            },

        }
        self.history.push(event.clone());
    }

    pub fn validate(&self, event: &GameEvent) -> bool {
        use GameEvent::*;

        match event {
            PlayerJoined { player_id, name } => {
                if self.players.contains_key(player_id) {
                    return false;
                }
            },
            PlayerLeft(player_id) => {  },
        }

        true
    }

    pub fn dispatch(&mut self, event: &GameEvent) -> Result<(), ()> {
        if !self.validate(event) {
            return Err(());
        }

        self.reduce(event);
        Ok(())
    }
}

