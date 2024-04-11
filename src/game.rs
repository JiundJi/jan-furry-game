use std::time::SystemTime;

use crate::logic::*;

pub struct Game {
    players: Vec<Player>,
    creation_time: SystemTime,
    start_time: SystemTime,
}

impl Game {
    pub fn new(m: Player) -> Game {
        let mut players: Vec<Player> = vec![];
        players.push(m);

        Game {
            players,
            creation_time: SystemTime::now(),
            start_time: SystemTime::now(),
        }

    }
}
