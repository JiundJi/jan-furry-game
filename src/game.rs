use std::time::SystemTime;

use crate::logic::*;

pub struct Game {
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
